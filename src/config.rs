use anyhow::{Context, Result};
use serde::Deserialize;
use std::fs;
use std::net::SocketAddr;

#[derive(Deserialize)]
pub struct Config {
    pub debug: bool,
    pub database: DatabaseConfig,
    pub server: ServerConfig,
}

#[derive(Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
}

#[derive(Deserialize)]
pub struct ServerConfig {
    pub bind: SocketAddr,
}

impl Config {
    pub fn from_file(file_path: &str) -> Result<Self> {
        let config_str = fs::read_to_string(file_path).context("Failed to read config file")?;
        serde_yaml::from_str(&config_str).context("Failed to parse config file")
    }
}
