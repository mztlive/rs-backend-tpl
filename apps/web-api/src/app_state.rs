use config::{Config, SafeConfig};
use container::ServiceFactory;
use mongodb::{Client, Database};
use rbac::ActorHandler as RbacActorHandler;

#[derive(Clone)]
pub struct DatabaseState {
    #[allow(dead_code)]
    pub client: Client,
    pub db: Database,
}

impl DatabaseState {
    pub fn new(client: Client, db: Database) -> Self {
        Self { client, db }
    }
}

#[derive(Clone)]
pub struct AppState {
    db_state: DatabaseState,
    config: SafeConfig,
    rbac: RbacActorHandler,
    services: ServiceFactory,
}

impl AppState {
    pub fn new(db_state: DatabaseState, config: SafeConfig, rbac: RbacActorHandler) -> Self {
        Self {
            services: ServiceFactory::new(db_state.db.clone()),
            db_state,
            config,
            rbac,
        }
    }

    pub async fn config(&self) -> Result<Config> {
        self.config.get_config().await.map_err(Error::ConfigError)
    }

    pub fn db(&self) -> &Database {
        &self.db_state.db
    }

    pub fn rbac(&self) -> &RbacActorHandler {
        &self.rbac
    }

    pub fn service_factory(&self) -> &ServiceFactory {
        &self.services
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("config error: {0}")]
    ConfigError(#[from] config::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
