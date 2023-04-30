use std::{
    collections::{HashMap, HashSet},
    path::{Path, PathBuf},
    process::Command,
};

use cargo_metadata::{camino::Utf8PathBuf, Package};
use proc_macro2::TokenStream;
use quote::quote;

use crate::util::{self, generate_soroban};

pub trait PackageExt {
    fn is_dep(&self) -> bool;
}

impl PackageExt for Package {
    /// Check if the package has the specified key in its metadata
    fn is_dep(&self) -> bool {
        #[allow(clippy::redundant_closure_for_method_calls)]
        self.metadata
            .as_object()
            .and_then(|metadata| metadata.get("riff"))
            .and_then(|riff| riff.as_object())
            .and_then(|riff_object| riff_object.get("export"))
            .and_then(|export| export.as_bool())
            .unwrap_or_default()
    }
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("{0}")]
    Stream(TokenStream),
}
impl From<Error> for TokenStream {
    fn from(value: Error) -> Self {
        match value {
            Error::Stream(ts) => ts,
        }
    }
}

/// Find all riff deps then use `syn_file_expand` to generate the needed functions from each dep
pub fn generate(paths: &[PathBuf]) -> TokenStream {
    let methods = paths
        .iter()
        .filter_map(|path| {
            let file = util::parse_crate_as_file(path)?;
            Some(generate_soroban(&file))
        })
        .collect::<Vec<_>>();
    quote! {
    struct SorobanContract;
    #[soroban_sdk::contractimpl]
    impl SorobanContract {
            #(#methods)*
    }}
}

pub fn get_deps(manifest_path: &Path) -> Result<Vec<Package>, Error> {
    let metadata = cargo_metadata::MetadataCommand::new()
        .manifest_path(manifest_path)
        .exec()
        .unwrap();

    let p = metadata
        .root_package()
        .ok_or_else(|| Error::Stream(quote!(compile_error!("failed to find root package"))))?;

    let packages = metadata
        .packages
        .iter()
        .map(|p| (format!("{}v{}", p.name, p.version), p))
        .collect::<HashMap<String, &Package>>();

    let output = Command::new("cargo")
        .current_dir(manifest_path.parent().unwrap())
        .args(["tree", "--prefix", "none", "--edges", "normal"])
        .output()
        .map_err(|_| Error::Stream(quote! {compile_error!("failed to run cargo tree on")}))?;
    let stdout = output.stdout;
    let stdout_str = String::from_utf8(stdout).unwrap();

    let res = stdout_str
        .lines()
        .filter_map(|line| {
            let s: Vec<&str> = line.split(' ').collect();
            let package_id = format!("{}{}", s[0], s[1]);
            let res = packages.get(&package_id).copied();
            if let Some(r) = &res {
                if r == &p {
                    return None;
                }
            }
            res.map(Clone::clone)
        })
        .collect::<Vec<_>>();
    Ok(res)
}

pub fn out_dir(target_dir: &Path, name: &str) -> PathBuf {
    target_dir.join("loam").join(name.replace('-', "_"))
}

pub fn get_loam_deps(manifest_path: &Path) -> Result<Vec<(Utf8PathBuf, PathBuf)>, Error> {
    get_deps(manifest_path)?
        .into_iter()
        .filter(Package::is_dep)
        .map(|p| {
            let version = &p.version;
            let name = &p.name;
            let dir = PathBuf::from(format!("{name}{version}"));
            let out_dir = out_dir(&dir, name);
            let res = (
                p.manifest_path
                    .parent()
                    .ok_or_else(|| {
                        Error::Stream(
                            quote! {compile_error("Failed to get parent of {}", package.name)},
                        )
                    })?
                    .join("src")
                    .join("lib.rs"),
                out_dir,
            );
            Ok(res)
        })
        .collect::<Result<HashSet<_>, Error>>()
        .map(IntoIterator::into_iter)
        .map(Iterator::collect::<Vec<_>>)
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_loam_deps() {
        let pwd = std::env::current_dir().unwrap();
        println!("{pwd:?}");
        let manifest_path = pwd.join("./test/normal/Cargo.toml");
        let mut c = cargo_metadata::MetadataCommand::new();
        c.manifest_path(&manifest_path);
        let metadata = c.exec().unwrap();
        let normal = metadata.root_package().unwrap();
        println!("{normal:#?}{}", normal.name);
        let deps = get_deps(&manifest_path).unwrap();
        println!("{deps:#?}\n{}", deps.len());
        let deps = get_loam_deps(&manifest_path).unwrap();
        println!("{deps:#?}\n{}", deps.len());
    }
}
