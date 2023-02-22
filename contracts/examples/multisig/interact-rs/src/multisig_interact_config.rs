use std::io::Read;

use serde::Deserialize;

/// Configuration file
const CONFIG_FILE: &str = "config.toml";

/// Multisig Interact configuration
#[derive(Debug, Deserialize)]
pub struct Config {
    gateway: String,
    pem: String,
}

impl Config {
    // Loads configuration from file and deserializes it
    pub fn load_config() -> Self {
        let mut file = std::fs::File::open(CONFIG_FILE).unwrap();
        let mut content = String::new();
        file.read_to_string(&mut content).unwrap();
        toml::from_str(&content).unwrap()
    }

    // Returns the gateway
    pub fn gateway(&self) -> &str {
        &self.gateway
    }

    // Returns the pem
    pub fn pem(&self) -> &str {
        &self.pem
    }
}
