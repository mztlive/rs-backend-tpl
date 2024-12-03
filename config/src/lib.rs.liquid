//! # Configuration Management Module
//!
//! This module provides comprehensive configuration management for the application,
//! supporting both file-based and Nacos-based configuration sources.
//!
//! ## Features
//!
//! * Load configuration from TOML files
//! * Integration with Nacos configuration center
//! * Thread-safe configuration access via RwLock
//! * Hot reloading of configuration when using Nacos
//! * Command-line argument parsing for configuration source selection
//!
//! ## Usage
//!
//! ### File-based Configuration
//!
//! ```no_run
//! use config::SafeConfig;
//!
//! #[tokio::main]
//! async fn main() {
//!     let config = SafeConfig::from_args().await.unwrap();
//!     let current_config = config.get_config().await.unwrap();
//!     println!("Server port: {}", current_config.app.port);
//! }
//! ```
//!
//! ### Nacos-based Configuration
//!
//! ```no_run
//! // Run with nacos enabled:
//! // ./program --enable-nacos --nacos-addr="http://localhost:8848"
//! //           --nacos-namespace="public" --nacos-group="DEFAULT_GROUP"
//! //           --nacos-data-id="config.toml"
//! ```
//!
//! ## Configuration Structure
//!
//! The configuration is structured into several nested components:
//! * `Config`: Top-level configuration container
//! * `AppConfig`: Application-specific settings
//! * `DatabaseConfig`: Database connection settings

use clap::Parser;
use command::Args;
use nacos::NacosConfig;
use nacos_watch::NacosConfigWatcher;
use serde::Deserialize;
use std::{path::Path, sync::Arc};
use tokio::sync::RwLock;

mod command;
mod errors;
mod nacos;
mod nacos_watch;
pub use errors::*;
pub use nacos::NacosConfigClient;

/// Main configuration struct containing all application settings.
///
/// This struct is the root container for all configuration settings,
/// deserializable from TOML format.
#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    /// Application-specific configuration
    pub app: AppConfig,
    /// Database-specific configuration
    pub database: DatabaseConfig,
}

/// Application-specific configuration settings.
///
/// Contains settings that control the core application behavior.
#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    /// The HTTP server port number the application will listen on
    pub port: u16,
    /// Secret key used for JWT token generation and validation
    pub secret: String,
    /// Base path where uploaded files will be stored
    pub upload_path: String,
}

/// Database connection configuration.
///
/// Contains all necessary settings for establishing database connections.
#[derive(Debug, Deserialize, Clone)]
pub struct DatabaseConfig {
    /// Complete database connection URI including protocol, host, and port
    pub uri: String,
    /// Name of the database to connect to
    pub db_name: String,
}

impl Config {
    /// Loads configuration from a TOML file at the specified path.
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the TOML configuration file
    ///
    /// # Returns
    ///
    /// * `Result<Self>` - The parsed configuration or an error
    pub async fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = tokio::fs::read_to_string(path).await?;
        let config = toml::from_str(&content)?;
        Ok(config)
    }

    /// Generates a complete URL for an uploaded file.
    ///
    /// # Arguments
    ///
    /// * `filename` - Name of the uploaded file
    ///
    /// # Returns
    ///
    /// * `String` - Complete URL to access the file
    pub fn file_url(&self, filename: &str) -> String {
        format!("{}/{}", self.get_upload_path(), filename)
    }

    /// Retrieves the configured upload path.
    ///
    /// # Returns
    ///
    /// * `String` - The configured upload path
    pub fn get_upload_path(&self) -> String {
        self.app.upload_path.clone()
    }

    /// Parses configuration from a TOML string.
    ///
    /// # Arguments
    ///
    /// * `content` - TOML-formatted configuration string
    ///
    /// # Returns
    ///
    /// * `Result<Self>` - The parsed configuration or an error
    pub fn from_str(content: &str) -> Result<Self> {
        let config = toml::from_str(content)?;
        Ok(config)
    }
}

/// Thread-safe configuration wrapper.
///
/// Provides safe concurrent access to configuration data using a read-write lock.
#[derive(Clone)]
pub struct SafeConfig {
    /// Thread-safe reference to the configuration
    pub inner: Arc<RwLock<Config>>,
}

impl SafeConfig {
    /// Creates a new SafeConfig instance from command line arguments.
    ///
    /// Determines whether to use file-based or Nacos-based configuration
    /// based on provided command line arguments.
    ///
    /// # Returns
    ///
    /// * `Result<Self>` - The initialized SafeConfig instance or an error
    pub async fn from_args() -> Result<Self> {
        let args = Args::parse();

        if args.is_enable_nacos() {
            return Self::from_nacos_with_watcher(args.to_nacos_config()).await;
        }

        let config = Config::from_file(&args.config_path).await?;
        Ok(Self {
            inner: Arc::new(RwLock::new(config)),
        })
    }

    /// Retrieves a clone of the current configuration.
    ///
    /// # Returns
    ///
    /// * `Result<Config>` - A clone of the current configuration or an error
    pub async fn get_config(&self) -> Result<Config> {
        let config = self.inner.read().await;
        Ok(config.clone())
    }

    /// Initializes configuration from Nacos and sets up a configuration watcher.
    ///
    /// # Arguments
    ///
    /// * `args` - Nacos configuration parameters
    ///
    /// # Returns
    ///
    /// * `Result<Self>` - The initialized SafeConfig instance or an error
    async fn from_nacos_with_watcher(args: NacosConfig) -> Result<Self> {
        let nacos_client = NacosConfigClient::from_config(args).await?;

        let content = nacos_client.get_config().await?;
        let config = Config::from_str(&content)?;
        let safe_config = Self {
            inner: Arc::new(RwLock::new(config)),
        };

        let watcher = NacosConfigWatcher::new(safe_config.clone(), nacos_client);
        watcher.start_watch().await?;

        Ok(safe_config)
    }

    /// Reloads configuration from Nacos.
    ///
    /// # Arguments
    ///
    /// * `nacos_client` - Reference to the Nacos client
    ///
    /// # Returns
    ///
    /// * `Result<()>` - Success or error indication
    async fn reload_from_nacos(&self, nacos_client: &NacosConfigClient) -> Result<()> {
        let content = nacos_client.get_config().await?;
        let config = toml::from_str(&content)?;
        let mut writer = self.inner.write().await;
        *writer = config;

        log::info!("Reload config from nacos success");
        Ok(())
    }
}
