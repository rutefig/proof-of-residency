// src/config.rs
use serde::Deserialize;
use std::env;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub sp1_prover: String,
    pub sp1_private_key: String,
    pub server_port: u16,
}

impl Config {
    pub fn new() -> Result<Self, config::ConfigError> {
        dotenv::dotenv().ok();
        
        Ok(Config {
            sp1_prover: env::var("SP1_PROVER").unwrap_or_else(|_| "local".to_string()),
            sp1_private_key: env::var("SP1_PRIVATE_KEY").unwrap_or_default(),
            server_port: env::var("SERVER_PORT")
                .unwrap_or_else(|_| "8080".to_string())
                .parse()
                .unwrap_or(8080),
        })
    }
}