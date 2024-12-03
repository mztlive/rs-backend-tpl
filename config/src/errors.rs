use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("TOML parse error: {0}")]
    Toml(#[from] toml::de::Error),

    #[error("Nacos error: {0}")]
    Nacos(#[from] nacos_sdk::api::error::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
