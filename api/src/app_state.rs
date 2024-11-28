use mongodb::{Client, Database};

use config::{AppConfig, Config};
use rbac::ActorHandler as RbacActorHandler;

#[derive(Clone)]
pub struct DatabaseState {
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
    pub db_state: DatabaseState,
    pub config: Config,
    pub rbac: RbacActorHandler,
}
