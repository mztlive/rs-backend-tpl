use mongodb::{Client, Database};

use crate::config::AppConfig;
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
    pub config: AppConfig,
    pub rbac: RbacActorHandler,
}
