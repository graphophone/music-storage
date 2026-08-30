use std::fs;

use anyhow::Result;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub rustfs: RustfsConfig,
}

impl Config {
    pub fn build(filename: &str) -> Result<Config> {
        let conf_str = fs::read_to_string(filename)?;
        let conf = toml::from_str(&conf_str)?;
        Ok(conf)
    }
}

#[derive(Deserialize)]
pub struct RustfsConfig {
    pub access_key: String,
    pub secret_key: String,
    pub endpoint_url: String,
    pub region: String,
    pub bucket: String,
}
