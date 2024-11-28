use super::super::base::{cursor_to_vec, IFilter, IPaginator};
use super::super::{collection_names::MESSAGE, IRepository};
use crate::errors::Error;
use async_trait::async_trait;
use entities::{Message, MessageStatus};
use mongodb::bson::doc;
use mongodb::Database;
use services::errors::Result as ServiceResult;

pub struct MessageRepository {
    pub coll_name: String,
    database: Database,
}

impl MessageRepository {
    pub fn new(database: Database) -> Self {
        Self {
            coll_name: MESSAGE.to_string(),
            database,
        }
    }
}

impl IRepository<Message> for MessageRepository {
    fn get_collection_name(&self) -> &str {
        &self.coll_name
    }

    fn get_database(&self) -> &Database {
        &self.database
    }
}
