use serde::Deserialize;
use std::collections::BTreeMap as Map;
use std::io;
use std::path::Path;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("⛔ ️parsing environments.toml: {0}")]
    ParsingToml(io::Error),
    #[error("⛔ ️no settings for current LOAM_ENV ({0:?}) found in environments.toml")]
    NoSettingsForCurrentEnv(String),
}

type Environments = Map<Box<str>, Environment>;

#[derive(Debug, serde::Deserialize, Clone)]
pub struct Environment {
    #[serde(default, deserialize_with = "deserialize_accounts")]
    pub accounts: Option<Vec<Account>>,
    pub network: Network,
    pub contracts: Option<Map<Box<str>, Contract>>,
}

fn deserialize_accounts<'de, D>(deserializer: D) -> Result<Option<Vec<Account>>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let opt: Option<Vec<AccountRepresentation>> = Option::deserialize(deserializer)?;
    Ok(opt.map(|vec| vec.into_iter().map(Account::from).collect()))
}

#[derive(Debug, serde::Deserialize, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Network {
    pub name: Option<String>,
    pub rpc_url: Option<String>,
    pub network_passphrase: Option<String>,
    // run_locally: Option<bool>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum AccountRepresentation {
    Simple(String),
    Detailed(Account),
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Account {
    pub name: String,
    #[serde(default)]
    pub default: bool,
}

impl From<AccountRepresentation> for Account {
    fn from(rep: AccountRepresentation) -> Self {
        match rep {
            AccountRepresentation::Simple(name) => Account {
                name,
                default: false,
            },
            AccountRepresentation::Detailed(account) => account,
        }
    }
}

#[derive(Debug, Deserialize, Clone, Default)]
pub struct Contract {
    #[serde(default = "default_client", skip_serializing_if = "std::ops::Not::not")]
    pub client: bool,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub init: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

fn default_client() -> bool {
    true
}

impl Environment {
    pub fn get(workspace_root: &Path, loam_env: &str) -> Result<Option<Environment>, Error> {
        let env_toml = workspace_root.join("environments.toml");

        if !env_toml.exists() {
            return Ok(None);
        }

        let toml_str = std::fs::read_to_string(env_toml).map_err(Error::ParsingToml)?;
        let mut parsed_toml: Environments = toml::from_str(&toml_str).unwrap();
        let current_env = parsed_toml.remove(loam_env);
        if current_env.is_none() {
            return Err(Error::NoSettingsForCurrentEnv(loam_env.to_string()));
        };
        Ok(current_env)
    }
}
