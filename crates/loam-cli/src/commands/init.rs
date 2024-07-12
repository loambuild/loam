use soroban_cli::commands::contract::init as soroban_init;
use std::{
    fs::{copy, create_dir_all, metadata, read_dir, remove_dir_all, Metadata},
    io,
    path::Path,
};

// command line argument parser
use clap::Parser;

// TO-DO: Store these somewhere else eventually. Sororban uses examples repo.
const LOAM_EXAMPLES: &str = "../../../../examples/soroban/";
const FRONTEND_TEMPLATE: &str = "https://github.com/loambuild/frontend";

/// A command to initialize a new project
#[derive(Parser, Debug, Clone)]
pub struct Cmd {
    /// The path to the project must be provided to initialize
    pub project_path: String,
}

// TO-DO: import function from stellar init. This is a slightly modified version of the function that does not edit cargo files
fn copy_example_contracts(from: &Path, to: &Path, contracts: &[String]) -> Result<(), Error> {
    let project_contracts_path = to.join("contracts");
    for contract in contracts {
        println!("ℹ️  Initializing example contract: {contract}");
        let contract_as_string = contract.to_string();
        let contract_path = Path::new(&contract_as_string);
        let from_contract_path = from.join(contract_path);
        let to_contract_path = project_contracts_path.join(contract_path);
        create_dir_all(&to_contract_path).map_err(|e| {
            eprintln!("Error creating directory: {contract_path:?}");
            e
        })?;

        copy_contents(&from_contract_path, &to_contract_path)?;
    }

    Ok(())
}

// TO-DO: imported from stellar init. Removes code that adds to gitignore and readme
fn copy_contents(from: &Path, to: &Path) -> Result<(), Error> {
    for entry in read_dir(from).map_err(|e| {
        eprintln!("Error reading directory: {from:?}");
        e
    })? {
        let entry = entry.map_err(|e| {
            eprintln!("Error reading entry in directory: {from:?}");
            e
        })?;
        let path = entry.path();
        let entry_name = entry.file_name().to_string_lossy().to_string();
        let new_path = to.join(&entry_name);

        if path.is_dir() {
            create_dir_all(&new_path).map_err(|e| {
                eprintln!("Error creating directory: {new_path:?}");
                e
            })?;
            copy_contents(&path, &new_path)?;
        } else {
            if file_exists(&new_path) {
                println!(
                    "ℹ️  Skipped creating {} as it already exists",
                    &new_path.to_string_lossy()
                );
                continue;
            }

            println!("➕  Writing {}", &new_path.to_string_lossy());
            copy(&path, &new_path).map_err(|e| {
                eprintln!(
                    "Error copying from {:?} to {:?}",
                    path.to_string_lossy(),
                    new_path.to_string_lossy()
                );
                e
            })?;
        }
    }

    Ok(())
}

// TO-DO: imported from stellar init
fn file_exists(file_path: &Path) -> bool {
    metadata(file_path)
        .as_ref()
        .map(Metadata::is_file)
        .unwrap_or(false)
}

/// Errors that can occur during initialization
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Io error: {0}")]
    IoError(#[from] io::Error),
    #[error("Soroban init error: {0}")]
    SorobanInitError(#[from] soroban_init::Error),
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
        let mut examples: Vec<String> = vec![];

        // Create a new project using the soroban init command
        // by default uses a provided frontend template
        // Examples cannot currently be added by user
        soroban_init::Cmd {
            project_path: self.project_path.clone(),
            with_example: examples.clone(),
            frontend_template: FRONTEND_TEMPLATE.to_string(),
        }
        .run()?;

        // remove soroban hello_world default contract
        remove_dir_all(Path::new(&self.project_path.clone()).join("contracts/hello_world/"))
            .map_err(|e| {
                eprintln!("Error removing directory");
                e
            })?;

        // core and status_message are default examples
        examples.push("core".to_string());
        examples.push("status_message".to_string());

        copy_example_contracts(
            Path::new(LOAM_EXAMPLES),
            Path::new(&self.project_path.clone()),
            &examples,
        )?;

        Ok(())
    }
}
