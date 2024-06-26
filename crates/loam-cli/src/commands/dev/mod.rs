use clap::Parser;
use notify::{RecursiveMode, Watcher};
use std::path::Path;
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

impl Cmd {
    pub async fn run(&self) -> Result<(), Error> {
        // Set LOAM_ENV to development
        std::env::set_var("LOAM_ENV", "development");
        let (tx, mut rx) = tokio::sync::mpsc::channel(100);
        let mut watcher =
            notify::recommended_watcher(move |res: Result<notify::Event, notify::Error>| {
                if let Ok(
                    event @ notify::Event {
                        kind: notify::EventKind::Modify(_),
                        ..
                    },
                ) = res
                {
                    if let Some(path) = event.paths.first() {
                        eprintln!("File modified: {path:?}");
                        // Send a signal through the channel to trigger a rebuild
                        if let Err(e) = tx.blocking_send(()) {
                            eprintln!("Error sending through channel: {e}");
                        }
                    }
                }
            })
            .unwrap();

        let workspace_root: &Path = self
            .build_cmd
            .manifest_path
            .parent()
            .unwrap_or_else(|| Path::new("."));
        let env_toml = workspace_root.join("environments.toml");
        if env_toml.exists() {
            watcher.watch(
                workspace_root.join("environments.toml").as_path(),
                RecursiveMode::NonRecursive,
            )?;
        }
        let packages = self.build_cmd.list_packages()?;
        for package in packages {
            let package_path = Path::new(package.manifest_path.parent().unwrap().as_str());
            watcher.watch(package_path, RecursiveMode::Recursive)?;
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
