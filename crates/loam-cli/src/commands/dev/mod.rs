use clap::Parser;
use notify::{self, RecursiveMode, Watcher};
use std::{
    fs,
    path::{Path, PathBuf},
    sync::Arc,
};
use tokio::sync::mpsc;
use tokio::time;

use crate::commands::build;

use super::build::clients::LoamEnv;

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
    fs::canonicalize(path).unwrap_or_else(|_| path.to_path_buf())
}

fn is_parent_in_watched_dirs(parent: &Path, watched_dirs: &[Arc<PathBuf>]) -> bool {
    watched_dirs.iter().any(|p| canonicalize_path(p) == parent)
}

fn is_temporary_file(path: &Path) -> bool {
    let file_name = path.file_name().and_then(|s| s.to_str()).unwrap_or("");
    
    // Vim temporary files
    if file_name.starts_with(".") {
        return true;
    }
    if file_name.ends_with("~") {
        return true;
    }

    // Emacs temporary files
    if file_name.starts_with("#") && file_name.ends_with("#") {
        return true;
    }
    // VSCode temporary files
    if file_name.ends_with(".tmp") {
        return true;
    }

    // Add more patterns for other editors as needed

    false
}

impl Cmd {
    pub async fn run(&self) -> Result<(), Error> {
        let (tx, mut rx) = mpsc::channel(100);
        let workspace_root: &Path = self
            .build_cmd
            .manifest_path
            .parent()
            .unwrap_or_else(|| Path::new("."));
        let env_toml_path = Arc::new(workspace_root.join("environments.toml"));
        let env_toml_parent = Arc::new(
            env_toml_path
                .parent()
                .unwrap_or(Path::new(""))
                .to_path_buf(),
        );

        let mut watched_dirs = Vec::new();
        let packages = self
            .build_cmd
            .list_packages()?
            .into_iter()
            .map(|package| {
                Arc::new(PathBuf::from(
                    package.manifest_path.parent().unwrap().as_str(),
                ))
            })
            .collect::<Vec<_>>();

        for package_path in &packages {
            watched_dirs.push(package_path.clone());
            eprintln!("Watching {}", package_path.as_path().display());
        }
        let watched_dirs_clone = watched_dirs.clone();
        let env_toml_path_clone = Arc::clone(&env_toml_path);
        let env_toml_parent_clone = Arc::clone(&env_toml_parent);
        let mut watcher =
            notify::recommended_watcher(move |res: Result<notify::Event, notify::Error>| {
                if let Ok(event) = res {
                    if matches!(
                        event.kind,
                        notify::EventKind::Create(_)
                            | notify::EventKind::Modify(_)
                            | notify::EventKind::Remove(_)
                    ) {
                        if let Some(path) = event.paths.first() {
                            // Ignore temporary files
                            if is_temporary_file(path) {
                                return;
                            }
                            let env_toml_parent_abs = canonicalize_path(&env_toml_parent_clone);
                            let env_toml_path_abs = canonicalize_path(&env_toml_path_clone);
                            let parent_is_env_toml_parent =
                                path.parent() == Some(env_toml_parent_abs.as_path());
                            let path_is_env_toml = path == env_toml_path_abs.as_path();
                            let parent_is_in_watched_dirs = is_parent_in_watched_dirs(
                                &env_toml_parent_abs,
                                &watched_dirs_clone,
                            );

                            let trigger_rebuild = !parent_is_env_toml_parent
                                || parent_is_in_watched_dirs
                                || path_is_env_toml;

                            if trigger_rebuild {
                                eprintln!("File changed: {path:?}");
                                if let Err(e) = tx.blocking_send(()) {
                                    eprintln!("Error sending through channel: {e}");
                                }
                            }
                        }
                    }
                }
            })
            .unwrap();
        // Watch the parent directory of environments.toml
        watcher.watch(env_toml_parent.as_path(), RecursiveMode::NonRecursive)?;
        for package_path in watched_dirs {
            watcher.watch(package_path.as_path(), RecursiveMode::Recursive)?;
        }

        if let Err(e) = self.build().await {
            eprintln!("Build error: {e}");
        }
        println!("Watching for changes. Press Ctrl+C to stop.");

        loop {
            tokio::select! {
                _ = rx.recv() => {
                    println!("Changes detected. Rebuilding...");
                    if let Err(e) = self.build().await {
                        eprintln!("Build error: {e}");
                    }
                }
                _ = tokio::signal::ctrl_c() => {
                    println!("Stopping dev mode.");
                    break;
                }
            }
            // Debounce to avoid multiple rapid rebuilds
            time::sleep(std::time::Duration::from_secs(1)).await;
        }

        Ok(())
    }

    async fn build(&self) -> Result<(), Error> {
        let mut build_cmd = self.build_cmd.clone();
        build_cmd
            .build_clients
            .env
            .get_or_insert(LoamEnv::Development);
        build_cmd.profile.get_or_insert_with(|| "debug".to_string());
        build_cmd.run().await?;
        Ok(())
    }
}
