use std::fs;
use toml::from_str;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub ledger: LedgerConfig,
}

#[derive(Deserialize)]
pub struct LedgerConfig {
    pub path: String,
    pub passphrase: String,
    pub schema_version: u32,
}

pub fn load_config() -> Result<Config, Box<dyn std::error::Error>> {
    let content = fs::read_to_string("config.toml")?;
    let config: Config = from_str(&content)?;
    Ok(config)
}