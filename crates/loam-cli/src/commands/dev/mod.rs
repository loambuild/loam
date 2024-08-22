use clap::Parser;
use ignore::gitignore::{Gitignore, GitignoreBuilder};
use notify::{self, RecursiveMode, Watcher as _};
use std::{
    env, fs,
    path::{Path, PathBuf},
    sync::Arc,
};
use tokio::sync::mpsc;
use tokio::sync::Mutex;
use tokio::time;

use crate::commands::build;

use super::build::clients::LoamEnv;
use super::build::env_toml::ENV_FILE;

pub enum Message {
    FileChanged,
}

#[derive(Parser, Debug, Clone)]
#[group(skip)]
pub struct Cmd {
    #[command(flatten)]
    pub build_cmd: build::Cmd,
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Watcher(#[from] notify::Error),
    #[error(transparent)]
    Build(#[from] build::Error),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

fn canonicalize_path(path: &Path) -> PathBuf {
    if path.as_os_str().is_empty() {
        env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
    } else if path.components().count() == 1 {
        // Path is a single component, assuming it's a filename
        env::current_dir()
            .unwrap_or_else(|_| PathBuf::from("."))
            .join(path)
    } else {
        fs::canonicalize(path).unwrap_or_else(|_| path.to_path_buf())
    }
}

#[derive(Clone)]
pub struct Watcher {
    env_toml_dir: Arc<PathBuf>,
    packages: Arc<Vec<PathBuf>>,
    ignores: Arc<Gitignore>,
}

impl Watcher {
    pub fn new(env_toml_dir: &Path, packages: &[PathBuf]) -> Self {
        let env_toml_dir: Arc<PathBuf> = Arc::new(canonicalize_path(env_toml_dir));
        let packages: Arc<Vec<PathBuf>> = Arc::new(packages.iter().map(|p| canonicalize_path(p)).collect());
        
        let mut builder = GitignoreBuilder::new(&*env_toml_dir);
        for package in packages.iter() {
            builder.add(package);
        }

        let common_ignores = vec![
            "*.swp", "*.swo", "*.swx", // Vim swap files
            "4913", // Vim temp files
            ".DS_Store", // macOS
            "Thumbs.db", // Windows
            "*~", // Backup files
            "*.bak", // Backup files
            ".vscode/", // VS Code
            ".idea/", // IntelliJ
            "*.tmp", // Temporary files
            "*.log", // Log files
        ];
        
        for pattern in common_ignores {
            builder.add_line(None, pattern).expect("Failed to add ignore pattern");
        }

        let ignores = Arc::new(builder.build().expect("Failed to build GitIgnore"));

        Self {
            env_toml_dir,
            packages,
            ignores,
        }
    }

    pub fn is_watched(&self, path: &Path) -> bool {
        let path = canonicalize_path(path);
        !self.ignores.matched(&path, path.is_dir()).is_ignore()
    }

    pub fn is_env_toml(&self, path: &Path) -> bool {
        path == self.env_toml_dir.join(ENV_FILE)
    }

    pub fn handle_event(&self, event: &notify::Event, tx: &mpsc::Sender<Message>) {
        if matches!(
            event.kind,
            notify::EventKind::Create(_)
                | notify::EventKind::Modify(_)
                | notify::EventKind::Remove(_)
        ) {
            if let Some(path) = event.paths.first() {
                if self.is_watched(path) {
                    eprintln!("File changed: {path:?}");
                    if let Err(e) = tx.blocking_send(Message::FileChanged) {
                        eprintln!("Error sending through channel: {e:?}");
                    }
                }
            }
        }
    }
}

impl Cmd {
    pub async fn run(&mut self) -> Result<(), Error> {
        let (tx, mut rx) = mpsc::channel::<Message>(100);
        let rebuild_state = Arc::new(Mutex::new(false));
        let workspace_root: &Path = self
            .build_cmd
            .manifest_path
            .parent()
            .unwrap_or_else(|| Path::new("."));
        let env_toml_dir = workspace_root;

        let packages = self
            .build_cmd
            .list_packages()?
            .into_iter()
            .map(|package| PathBuf::from(package.manifest_path.parent().unwrap().as_str()))
            .collect::<Vec<_>>();

        let watcher = Watcher::new(env_toml_dir, &packages);

        for package_path in watcher.packages.iter() {
            eprintln!("Watching {}", package_path.display());
        }

        let mut notify_watcher =
            notify::recommended_watcher(move |res: Result<notify::Event, notify::Error>| {
                if let Ok(event) = res {
                    watcher.handle_event(&event, &tx);
                }
            })
            .unwrap();

        notify_watcher.watch(
            &canonicalize_path(env_toml_dir),
            RecursiveMode::NonRecursive,
        )?;
        for package_path in packages {
            notify_watcher.watch(&canonicalize_path(&package_path), RecursiveMode::Recursive)?;
        }

        let build_command = self.cloned_build_command();
        if let Err(e) = build_command.run().await {
            eprintln!("Build error: {e}");
        }
        eprintln!("Watching for changes. Press Ctrl+C to stop.");

        let rebuild_state_clone = rebuild_state.clone();
        loop {
            tokio::select! {
                _ = rx.recv() => {
                    let mut state = rebuild_state_clone.lock().await;
                    let build_command_inner = build_command.clone();
                    if !*state {
                        *state = true;
                        tokio::spawn(Self::debounced_rebuild(build_command_inner, Arc::clone(&rebuild_state_clone)));
                    }
                }
                _ = tokio::signal::ctrl_c() => {
                    eprintln!("Stopping dev mode.");
                    break;
                }
            }
        }
        Ok(())
    }

    async fn debounced_rebuild(build_command: Arc<build::Cmd>, rebuild_state: Arc<Mutex<bool>>) {
        // Debounce to avoid multiple rapid rebuilds
        time::sleep(std::time::Duration::from_secs(1)).await;

        eprintln!("Changes detected. Rebuilding...");
        if let Err(e) = build_command.run().await {
            eprintln!("Build error: {e}");
        }
        eprintln!("Watching for changes. Press Ctrl+C to stop.");

        let mut state = rebuild_state.lock().await;
        *state = false;
    }

    fn cloned_build_command(&mut self) -> Arc<build::Cmd> {
        self.build_cmd
            .build_clients_args
            .env
            .get_or_insert(LoamEnv::Development);
        Arc::new(self.build_cmd.clone())
    }
}
