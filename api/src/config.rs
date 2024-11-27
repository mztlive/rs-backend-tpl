use serde::Deserialize;
use tokio::fs;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Parse Error: {0}")]
    ParseError(#[from] toml::de::Error),

    #[error("IO Error: {0}")]
    IoError(#[from] std::io::Error),
}

#[derive(Deserialize, Clone)]
pub struct Database {
    pub uri: String,
    pub db_name: String,
}

#[derive(Deserialize, Clone)]
pub struct App {
    pub port: u16,
    pub secret: String,
    pub statistic_host: String,
    pub upload_path: String,
}

#[derive(Deserialize, Clone)]
pub struct AppConfig {
    pub app: App,
    pub database: Database,
}

impl AppConfig {
    pub fn get_statistic_host(&self) -> &str {
        &self.app.statistic_host
    }

    pub fn get_upload_path(&self) -> &str {
        &self.app.upload_path
    }

    pub fn file_url(&self, filename: &str) -> String {
        format!(
            "{}/{}/{}",
            self.get_statistic_host(),
            self.get_upload_path(),
            filename
        )
    }
}

/// Load the configuration from a file
///
/// # Errors
///
/// This function will return an error if the file cannot be read or if the file cannot be parsed.
pub async fn load_config(path: &str) -> Result<AppConfig, Error> {
    let content = fs::read_to_string(path).await?;
    let cfg = toml::from_str(&content)?;

    Ok(cfg)
}
