#![allow(clippy::struct_excessive_bools)]
use crate::commands::build::env_toml;
use stellar_xdr::curr::Error as xdrError;
use serde_json;
use soroban_cli::commands::NetworkRunnable;
use soroban_cli::utils::contract_hash;
use soroban_cli::utils::contract_hash;
use soroban_cli::{commands as cli, CommandParser};
use std::collections::BTreeMap as Map;
use std::fmt::Debug;
use std::hash::Hash;
use serde_json;
use std::hash::Hash;
use stellar_xdr::curr::Error as xdrError;

use super::env_toml::Network;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, clap::ValueEnum)]
pub enum LoamEnv {
    Development,
    Testing,
    Staging,
    Production,
}

impl std::fmt::Display for LoamEnv {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{self:?}").to_lowercase())
    }
}

#[derive(clap::Args, Debug, Clone, Copy)]
pub struct Args {
    #[arg(env = "LOAM_ENV", value_enum)]
    pub env: Option<LoamEnv>,
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    EnvironmentsToml(#[from] env_toml::Error),
    #[error("⛔ ️invalid network: must either specify a network name or both network_passphrase and rpc_url")]
    MalformedNetwork,
    #[error(transparent)]
    ParsingNetwork(#[from] cli::network::Error),
    #[error(transparent)]
    GeneratingKey(#[from] cli::keys::generate::Error),
    #[error("⛔ ️can only have one default account; marked as default: {0:?}")]
    OnlyOneDefaultAccount(Vec<String>),
    #[error("⛔ ️you need to provide at least one account, to use as the source account for contract deployment and other operations")]
    NeedAtLeastOneAccount,
    #[error("⛔ ️No contract named {0:?}")]
    BadContractName(String),
    #[error("⛔ ️Contract update not allowed in production for {0:?}")]
    ContractUpdateNotAllowed(String),
    #[error(transparent)]
    ContractInstall(#[from] cli::contract::install::Error),
    #[error(transparent)]
    ContractDeploy(#[from] cli::contract::deploy::wasm::Error),
    #[error(transparent)]
    ContractBindings(#[from] cli::contract::bindings::typescript::Error),
    #[error(transparent)]
    ContractFetch(#[from] cli::contract::fetch::Error),
    #[error(transparent)]
    ConfigLocator(#[from] cli::config::locator::Error),
    #[error(transparent)]
    ConfigAlias(#[from] cli::config::alias::Error),
    #[error(transparent)]
    Clap(#[from] clap::Error),
    #[error(transparent)]
    WasmHash(#[from] xdrError),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Json(#[from] serde_json::Error),
}

impl Args {
    pub async fn run(&self, workspace_root: &std::path::Path) -> Result<(), Error> {
        let Some(current_env) =
            env_toml::Environment::get(workspace_root, &self.loam_env(LoamEnv::Production))?
        else {
            return Ok(());
        };

        Self::add_network_to_env(&current_env.network)?;
        Self::handle_accounts(current_env.accounts.as_deref()).await?;
        self.handle_contracts(
            workspace_root,
            current_env.contracts.as_ref(),
            &current_env.network,
        )
        .await?;

        Ok(())
    }

    fn loam_env(self, default: LoamEnv) -> String {
        self.env.unwrap_or(default).to_string().to_lowercase()
    }

    /// Parse the network settings from the environments.toml file and set `STELLAR_RPC_URL` and
    /// `STELLAR_NETWORK_PASSPHRASE`.
    ///
    /// We could set `STELLAR_NETWORK` instead, but when importing contracts, we want to hard-code
    /// the network passphrase. So if given a network name, we use soroban-cli to fetch the RPC url
    /// & passphrase for that named network, and still set the environment variables.
    fn add_network_to_env(network: &env_toml::Network) -> Result<(), Error> {
        match &network {
            Network {
                name: Some(name), ..
            } => {
                let cli::network::Network {
                    rpc_url,
                    network_passphrase,
                } = (cli::network::Args {
                    network: Some(name.clone()),
                    rpc_url: None,
                    network_passphrase: None,
                })
                .get(&cli::config::locator::Args {
                    global: false,
                    config_dir: None,
                })?;
                eprintln!("🌐 using {name} network");
                std::env::set_var("STELLAR_RPC_URL", rpc_url);
                std::env::set_var("STELLAR_NETWORK_PASSPHRASE", network_passphrase);
            }
            Network {
                rpc_url: Some(rpc_url),
                network_passphrase: Some(passphrase),
                ..
            } => {
                std::env::set_var("STELLAR_RPC_URL", rpc_url);
                std::env::set_var("STELLAR_NETWORK_PASSPHRASE", passphrase);
                eprintln!("🌐 using network at {rpc_url}");
            }
            _ => return Err(Error::MalformedNetwork),
        }

        Ok(())
    }

    fn get_network_args(network: &Network) -> cli::network::Args {
        cli::network::Args {
            rpc_url: network.rpc_url.clone(),
            network_passphrase: network.network_passphrase.clone(),
            network: network.name.clone(),
        }
    }

    fn get_config_locator() -> cli::config::locator::Args {
        cli::config::locator::Args {
            global: false,
            config_dir: None,
        }
    }

    fn get_config_dir(network: &Network) -> cli::config::Args {
        let account =
            std::env::var("STELLAR_ACCOUNT").expect("No STELLAR_ACCOUNT environment variable set");
        cli::config::Args {
            network: Self::get_network_args(network),
            locator: Self::get_config_locator(),
            source_account: account,
            hd_path: Some(0),
        }
    }

    fn get_contract_alias(
        name: &str,
        network: &Network,
    ) -> Result<Option<String>, cli::config::alias::Error> {
        let config_dir = Self::get_config_dir(network);
        let network_passphrase = std::env::var("STELLAR_NETWORK_PASSPHRASE")
            .expect("No STELLAR_NETWORK_PASSPHRASE environment variable set");
        config_dir.get_contract_id(name, &network_passphrase)
    }

    async fn contract_hash_matches(
        &self,
        contract_id: &str,
        hash: &str,
        network: &Network,
    ) -> Result<bool, Error> {
        let hash_vec = cli::contract::fetch::Cmd {
            contract_id: contract_id.to_string(),
            out_file: None,
            locator: Self::get_config_locator(),
            network: Self::get_network_args(network),
        }
        .run_against_rpc_server(None, None)
        .await?;
        let ctrct_hash = contract_hash(&hash_vec)?;
        Ok(hex::encode(ctrct_hash) == hash)
    }

    fn save_contract_alias(
        name: &str,
        contract_id: &str,
        network: &Network,
    ) -> Result<(), cli::config::alias::Error> {
        let config_dir = Self::get_config_dir(network);
        config_dir.save_contract_id(contract_id, name)
    }

    fn write_contract_template(
        self,
        workspace_root: &std::path::Path,
        name: &str,
        contract_id: &str,
    ) -> Result<(), Error> {
        let allow_http = if self.loam_env() == "development" {
            "\n  allowHttp: true,"
        } else {
            ""
        };
        let network = std::env::var("STELLAR_NETWORK_PASSPHRASE")
            .expect("No STELLAR_NETWORK_PASSPHRASE environment variable set");
        let template = format!(
            r#"import * as Client from '{name}';
import {{ rpcUrl }} from './util';
    
export default new Client.Client({{
  networkPassphrase: '{network}',
  contractId: '{contract_id}',
  rpcUrl,{allow_http}
  publicKey: undefined,
}});
"#
        );
        let path = workspace_root.join(format!("src/contracts/{name}.ts"));
        std::fs::write(path, template)?;
        Ok(())
    }

    async fn handle_accounts(accounts: Option<&[env_toml::Account]>) -> Result<(), Error> {
        let Some(accounts) = accounts else {
            return Err(Error::NeedAtLeastOneAccount);
        };

        let default_account_candidates = accounts
            .iter()
            .filter(|&account| account.default)
            .map(|account| account.name.clone())
            .collect::<Vec<_>>();

        let default_account = match (default_account_candidates.as_slice(), accounts) {
            ([], []) => return Err(Error::NeedAtLeastOneAccount),
            ([], [env_toml::Account { name, .. }, ..]) => name.clone(),
            ([candidate], _) => candidate.to_string(),
            _ => return Err(Error::OnlyOneDefaultAccount(default_account_candidates)),
        };

        for account in accounts {
            eprintln!("🔐 creating keys for {:?}", account.name);
            cli::keys::generate::Cmd::parse_arg_vec(&[&account.name])?
                .run()
                .await?;
        }

        std::env::set_var("STELLAR_ACCOUNT", &default_account);

        Ok(())
    }

    async fn handle_contracts(
        &self,
        workspace_root: &std::path::Path,
        contracts: Option<&Map<Box<str>, env_toml::Contract>>,
        network: &Network,
    ) -> Result<(), Error> {
        let Some(contracts) = contracts else {
            return Ok(());
        };
        for (name, settings) in contracts {
            if settings.workspace {
                let wasm_path = workspace_root.join(format!("target/loam/{name}.wasm"));
                if !wasm_path.exists() {
                    return Err(Error::BadContractName(name.to_string()));
                }
                eprintln!("📲 installing {name:?} wasm bytecode on-chain...");
                let hash = cli::contract::install::Cmd::parse_arg_vec(&[
                    "--wasm",
                    wasm_path
                        .to_str()
                        .expect("we do not support non-utf8 paths"),
                ])?
                .run_against_rpc_server(None, None)
                .await?
                .into_result()
                .expect("no hash returned by 'contract install'")
                .to_string();
                eprintln!("    ↳ hash: {hash}");

                // Check if we have an alias saved for this contract
                let alias = Self::get_contract_alias(name, network)?;
                if let Some(contract_id) = alias {
                    match self
                        .contract_hash_matches(&contract_id, &hash, network)
                        .await
                    {
                        Ok(true) => {
                            eprintln!("✅ Contract {name:?} is up to date");
                            continue;
                        }
                        Ok(false) if self.loam_env() == "production" => {
                            return Err(Error::ContractUpdateNotAllowed(name.to_string()));
                        }
                        Ok(false) => eprintln!("🔄 Updating contract {name:?}"),
                        Err(e) => return Err(e),
                    }
                }

                eprintln!("🪞 instantiating {name:?} smart contract");
                let contract_id = cli::contract::deploy::wasm::Cmd::parse_arg_vec(&[
                    "--alias",
                    name,
                    "--wasm-hash",
                    &hash,
                ])?
                .run_against_rpc_server(None, None)
                .await?
                .into_result()
                .expect("no contract id returned by 'contract deploy'");
                eprintln!("    ↳ contract_id: {contract_id}");

                // Save the alias for future use
                Self::save_contract_alias(name, &contract_id, network)?;

                eprintln!("🎭 binding {name:?} contract");
                cli::contract::bindings::typescript::Cmd::parse_arg_vec(&[
                    "--contract-id",
                    &contract_id,
                    "--output-dir",
                    workspace_root
                        .join(format!("packages/{name}"))
                        .to_str()
                        .expect("we do not support non-utf8 paths"),
                    "--overwrite",
                ])?
                .run()
                .await?;

                eprintln!("🍽️ importing {:?} contract", name.clone());
                self.write_contract_template(workspace_root, name, &contract_id)?;
            };
        }

        Ok(())
    }
}
