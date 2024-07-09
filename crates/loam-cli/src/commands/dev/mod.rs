use clap::Parser;
use notify::{self, RecursiveMode, Watcher as NotifyWatcher};
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

enum Message {
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
struct Watcher {
    root_env: Arc<PathBuf>,
    packages: Arc<Vec<PathBuf>>,
}

impl Watcher {
    pub fn new(root_env: &Path, packages: &[PathBuf]) -> Self {
        Self {
            root_env: Arc::new(canonicalize_path(root_env)),
            packages: Arc::new(packages.iter().map(canonicalize_path).collect()),
        }
    }

    pub fn is_watched(&self, path: &Path) -> bool {
        let path = canonicalize_path(path);
        self.packages.iter().any(|p| path.starts_with(p))
    }

    pub fn is_env_toml(&self, path: &Path) -> bool {
        canonicalize_path(path) == *self.root_env
    }
}

fn is_temporary_file(path: &Path) -> bool {
    const IGNORED_EXTENSIONS: &[&str] = &["tmp", "swp", "swo"];
    let file_name = path.file_name().and_then(|s| s.to_str()).unwrap_or("");

    // Vim and vscode temporary files
    if path
        .extension()
        .and_then(|ext| ext.to_str())
        .map_or(false, |ext| {
            IGNORED_EXTENSIONS
                .iter()
                .any(|&ignored| ext.eq_ignore_ascii_case(ignored))
        })
    {
        return true;
    }

    // Vim temporary files
    if file_name.ends_with('~') {
        return true;
    }

    // Emacs temporary files
    if file_name.starts_with('#') && file_name.ends_with('#') {
        return true;
    }

    // Add more patterns for other editors as needed

    false
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
        let env_toml_path = workspace_root.join("environments.toml");

        let packages = self
            .build_cmd
            .list_packages()?
            .into_iter()
            .map(|package| PathBuf::from(package.manifest_path.parent().unwrap().as_str()))
            .collect::<Vec<_>>();

        let watcher = Watcher::new(&env_toml_path, &packages);

        for package_path in watcher.packages.iter() {
            eprintln!("Watching {}", package_path.display());
        }

        let watcher_clone = watcher.clone();
        let mut notify_watcher =
            notify::recommended_watcher(move |res: Result<notify::Event, notify::Error>| {
                if let Ok(event) = res {
                    if matches!(
                        event.kind,
                        notify::EventKind::Create(_)
                            | notify::EventKind::Modify(_)
                            | notify::EventKind::Remove(_)
                    ) {
                        if let Some(path) = event.paths.first() {
                            if is_temporary_file(path) {
                                return;
                            }
                            if watcher_clone.is_watched(path) || watcher_clone.is_env_toml(path) {
                                eprintln!("File changed: {path:?}");
                                if let Err(e) = tx.blocking_send(Message::FileChanged) {
                                    eprintln!("Error sending through channel: {e}");
                                }
                            }
                        }
                    }
                }
            })
            .unwrap();

        notify_watcher.watch(
            watcher.root_env.parent().unwrap(),
            RecursiveMode::NonRecursive,
        )?;
        for package_path in watcher.packages.iter() {
            notify_watcher.watch(package_path, RecursiveMode::Recursive)?;
        }

        let build_command = self.cloned_build_command();
        let cmd = build_command.lock().await;
        if let Err(e) = cmd.run().await {
            eprintln!("Build error: {e}");
        }
        eprintln!("Watching for changes. Press Ctrl+C to stop.");

        let rebuild_state_clone = rebuild_state.clone();
        loop {
            tokio::select! {
                _ = rx.recv() => {
                    let mut state = rebuild_state_clone.lock().await;
                    let build_command_inner = self.cloned_build_command();
                    if !*state {
                        *state= true;
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

    async fn debounced_rebuild(
        build_command: Arc<Mutex<build::Cmd>>,
        rebuild_state: Arc<Mutex<bool>>,
    ) {
        // Debounce to avoid multiple rapid rebuilds
        time::sleep(std::time::Duration::from_secs(1)).await;

        eprintln!("Changes detected. Rebuilding...");
        let cmd = build_command.lock().await;
        if let Err(e) = cmd.run().await {
            eprintln!("Build error: {e}");
        }
        eprintln!("Watching for changes. Press Ctrl+C to stop.");

        let mut state = rebuild_state.lock().await;
        *state = false;
    }

    fn cloned_build_command(&mut self) -> Arc<Mutex<build::Cmd>> {
        self.build_cmd
            .build_clients
            .env
            .get_or_insert(LoamEnv::Development);
        self.build_cmd
            .profile
            .get_or_insert_with(|| "debug".to_string());
        Arc::new(Mutex::new(self.build_cmd.clone()))
    }
}
