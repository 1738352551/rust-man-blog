use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
pub struct Config {
    pub database: DatabaseConfig,
    pub server: ServerConfig,
}

#[derive(Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
}

#[derive(Deserialize)]
pub struct ServerConfig {
    pub address: String,
    pub port: u16,
}

pub fn read_config(filename: &str) -> Result<Config, Box<dyn std::error::Error>> {
    let config_str = fs::read_to_string(filename)?;
    let config: Config = serde_yaml::from_str(&config_str)?;
    Ok(config)
}
