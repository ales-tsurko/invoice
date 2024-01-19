use std::fs;
use std::path::Path;

use anyhow::Result as Anyhow;
use serde::{Deserialize, Serialize};

// Config
#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct Config {
    pub(crate) suppliers: Vec<Supplier>,
    pub(crate) accounts: Vec<Account>,
    pub(crate) customers: Vec<Customer>,
    pub(crate) services: Vec<String>,
}

impl Config {
    pub(crate) fn open(path: &Path) -> Anyhow<Self> {
        if !path.exists() {
            let default = Self::default();
            let content = toml::to_string(&default)?;
            fs::create_dir_all(
                path.parent()
                    .expect("cannot create directory for configuration file"),
            )?;
            fs::write(path, content)?;

            return Ok(default);
        }

        let content = fs::read_to_string(path)?;
        Ok(toml::from_str::<Self>(&content)?)
    }

    pub(crate) fn validate(&self) {
        let msg = |val: &str| panic!("Please add at list one {} to the config file.", val);

        if self.suppliers.is_empty() {
            msg("business");
        }

        if self.accounts.is_empty() {
            msg("account");
        }

        if self.customers.is_empty() {
            msg("customer");
        }

        if self.services.is_empty() {
            msg("service");
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            suppliers: vec![Supplier {
                name: "Yanka Kupala".to_string(),
                address: "Minsk, vul. Kupaly, 1".to_string(),
                nip: "1234567890".to_string(),
            }],
            accounts: vec![Account {
                number: "XX00000000000000000000000000".to_string(),
                bank: "Bank Name".to_string(),
                swift: "XXXXXXXX".to_string(),
            }],
            customers: vec![Customer {
                name: "Acme Inc".to_string(),
                address: "Earth, Elm str. 8/12-17".to_string(),
            }],
            services: vec!["Doing stuff".to_string()],
        }
    }
}

// Customer
#[derive(Debug, Deserialize, Clone, Serialize)]
pub(crate) struct Customer {
    pub(crate) name: String,
    pub(crate) address: String,
}

impl std::fmt::Display for Customer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

// Account
#[derive(Debug, Deserialize, Clone, Serialize)]
pub(crate) struct Account {
    pub(crate) number: String,
    pub(crate) bank: String,
    pub(crate) swift: String,
}

impl std::fmt::Display for Account {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.number)
    }
}

// Business
#[derive(Debug, Deserialize, Clone, Serialize)]
pub(crate) struct Supplier {
    pub(crate) name: String,
    pub(crate) address: String,
    pub(crate) nip: String,
}

impl std::fmt::Display for Supplier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} | NIP: {}", self.name, self.nip)
    }
}
