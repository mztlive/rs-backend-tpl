use config::Config;
use container::ServiceFactory;
use libs::cache::Cache;
use mongodb::{Client, Database};
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
    pub services: ServiceFactory,
}

impl AppState {
    pub fn new(db_state: DatabaseState, config: Config, rbac: RbacActorHandler) -> Self {
        Self {
            services: ServiceFactory::new(db_state.db.clone()),
            db_state,
            config,
            rbac,
        }
    }
}
