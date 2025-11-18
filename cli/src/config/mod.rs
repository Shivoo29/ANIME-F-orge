pub mod providers;

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub backend: String,
    pub model: String,
    pub api_key: Option<String>,
    pub output_dir: Option<String>,
    pub ollama_endpoint: Option<String>,
    pub marketplace_token: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            backend: "ollama".to_string(),
            model: "llama2".to_string(),
            api_key: None,
            output_dir: Some("./animations".to_string()),
            ollama_endpoint: Some("http://localhost:11434".to_string()),
            marketplace_token: None,
        }
    }
}

impl Config {
    pub fn config_dir() -> Result<PathBuf> {
        let home = dirs::home_dir()
            .context("Could not determine home directory")?;

        Ok(home.join(".animaforge"))
    }

    pub fn config_path() -> Result<PathBuf> {
        Ok(Self::config_dir()?.join("config.toml"))
    }

    pub fn load() -> Result<Self> {
        let config_path = Self::config_path()?;

        if !config_path.exists() {
            // Create default config
            let config = Self::default();
            config.save()?;
            return Ok(config);
        }

        let content = fs::read_to_string(&config_path)
            .context("Failed to read config file")?;

        let config: Config = toml::from_str(&content)
            .context("Failed to parse config file")?;

        Ok(config)
    }

    pub fn save(&self) -> Result<()> {
        let config_dir = Self::config_dir()?;

        // Create config directory if it doesn't exist
        fs::create_dir_all(&config_dir)
            .context("Failed to create config directory")?;

        let config_path = Self::config_path()?;

        let content = toml::to_string_pretty(self)
            .context("Failed to serialize config")?;

        fs::write(&config_path, content)
            .context("Failed to write config file")?;

        Ok(())
    }
}
