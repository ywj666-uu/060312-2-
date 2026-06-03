use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub database_path: String,
    pub server_host: String,
    pub server_port: u16,
    pub sync_interval_seconds: u64,
    pub frontend_dist_path: String,
    pub github_api_base_url: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            database_path: "./data.db".to_string(),
            server_host: "127.0.0.1".to_string(),
            server_port: 8080,
            sync_interval_seconds: 3600,
            frontend_dist_path: "./frontend/dist".to_string(),
            github_api_base_url: "https://api.github.com".to_string(),
        }
    }
}

impl AppConfig {
    pub fn load() -> Self {
        match fs::read_to_string("config.toml") {
            Ok(content) => toml::from_str(&content).unwrap_or_default(),
            Err(_) => Self::default(),
        }
    }
}
