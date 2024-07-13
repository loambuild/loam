use clap::Parser;
use rust_embed::{EmbeddedFile, RustEmbed};
use soroban_cli::commands::contract::init as soroban_init;
use std::{
    fs::{create_dir_all, metadata, remove_dir_all, write, Metadata},
    io,
    path::Path,
};

const FRONTEND_TEMPLATE: &str = "https://github.com/loambuild/frontend";

#[derive(RustEmbed)]
#[folder = "../../examples/soroban/core"]
struct ExampleCore;

#[derive(RustEmbed)]
#[folder = "../../examples/soroban/status_message"]
struct ExampleStatusMessage;

/// A command to initialize a new project
#[derive(Parser, Debug, Clone)]
pub struct Cmd {
    /// The path to the project must be provided to initialize
    pub project_path: String,
}
/// Errors that can occur during initialization
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Io error: {0}")]
    IoError(#[from] io::Error),
    #[error("Soroban init error: {0}")]
    SorobanInitError(#[from] soroban_init::Error),
    #[error("Failed to convert bytes to string: {0}")]
    ConverBytesToStringErr(#[from] std::str::Utf8Error),
}

impl Cmd {
    /// Run the initialization command by calling the soroban init command
    ///
    /// # Example:
    ///
    /// ```
    /// /// From the command line
    /// loam init /path/to/project
    /// ```
    #[allow(clippy::unused_self)]
    pub fn run(&self) -> Result<(), Error> {
        // Create a new project using the soroban init command
        // by default uses a provided frontend template
        // Examples cannot currently be added by user
        soroban_init::Cmd {
            project_path: self.project_path.clone(),
            with_example: vec![],
            frontend_template: FRONTEND_TEMPLATE.to_string(),
        }
        .run()?;

        // remove soroban hello_world default contract
        remove_dir_all(Path::new(&self.project_path.clone()).join("contracts/hello_world/"))
            .map_err(|e| {
                eprintln!("Error removing directory");
                e
            })?;

        copy_example_contracts(Path::new(&self.project_path.clone()))?;

        Ok(())
    }
}

fn copy_example_contracts(to: &Path) -> Result<(), Error> {
    for item in ExampleCore::iter() {
        copy_file(
            &to.join("contracts/core"),
            item.as_ref(),
            ExampleCore::get(&item),
        )?;
    }
    for item in ExampleStatusMessage::iter() {
        copy_file(
            &to.join("contracts/status_message"),
            item.as_ref(),
            ExampleStatusMessage::get(&item),
        )?;
    }

    Ok(())
}

fn copy_file(
    example_path: &Path,
    filename: &str,
    embedded_file: Option<EmbeddedFile>,
) -> Result<(), Error> {
    let to = example_path.join(filename);
    if file_exists(&to) {
        println!(
            "ℹ️  Skipped creating {} as it already exists",
            &to.to_string_lossy()
        );
        return Ok(());
    }
    create_dir_all(to.parent().expect("invalid path")).map_err(|e| {
        eprintln!("Error creating directory path for: {to:?}");
        e
    })?;

    let Some(embedded_file) = embedded_file else {
        println!("⚠️  Failed to read file: {filename}");
        return Ok(());
    };

    let file_contents = std::str::from_utf8(embedded_file.data.as_ref()).map_err(|e| {
        eprintln!("Error converting file contents in {filename:?} to string",);
        e
    })?;

    println!("➕  Writing {}", &to.to_string_lossy());
    write(&to, file_contents).map_err(|e| {
        eprintln!("Error writing file: {to:?}");
        e
    })?;
    Ok(())
}

// TODO: import from stellar-cli init (not currently pub there)
fn file_exists(file_path: &Path) -> bool {
    metadata(file_path)
        .as_ref()
        .map(Metadata::is_file)
        .unwrap_or(false)
}
