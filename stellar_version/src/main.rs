use cargo_metadata::{MetadataCommand, Package};
use std::process;

fn main() {
    // Run the `cargo metadata` command to get workspace metadata
    let metadata = match MetadataCommand::new().exec() {
        Ok(metadata) => metadata,
        Err(error) => {
            eprintln!("Failed to get cargo metadata: {error}");
            process::exit(1);
        }
    };

    // Iterate over all packages in the workspace
    for package in metadata.packages {
        if package_is_using_stellar_cli(&package) {
            if let Some(version) = get_stellar_cli_version(&package) {
                println!("{version}");
                return;
            }
        }
    }

    eprintln!("stellar-cli dependency not found in any crate.");
    process::exit(1);
}

fn package_is_using_stellar_cli(package: &Package) -> bool {
    package
        .dependencies
        .iter()
        .any(|dep| dep.name == "stellar-cli")
}

fn get_stellar_cli_version(package: &Package) -> Option<String> {
    package
        .dependencies
        .iter()
        .find(|dep| dep.name == "stellar-cli")
        .map(|dep| dep.req.to_string())
}
