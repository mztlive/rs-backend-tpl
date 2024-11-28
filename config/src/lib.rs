use clap::{command, Parser};
use serde::Deserialize;
use std::path::Path;
use thiserror::Error;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long, default_value = "./config.toml")]
    config_path: String,
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("TOML parse error: {0}")]
    Toml(#[from] toml::de::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub app: AppConfig,
    pub database: DatabaseConfig,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub port: u16,
    pub secret: String,
    pub upload_path: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DatabaseConfig {
    pub uri: String,
    pub db_name: String,
}

impl Config {
    pub async fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = tokio::fs::read_to_string(path).await?;
        let config = toml::from_str(&content)?;
        Ok(config)
    }

    pub fn file_url(&self, filename: &str) -> String {
        format!("{}/{}", self.get_upload_path(), filename)
    }

    pub fn get_upload_path(&self) -> String {
        self.app.upload_path.clone()
    }

    pub async fn from_args() -> Result<Self> {
        let args = Args::parse();
        Self::from_file(&args.config_path).await
    }
}
