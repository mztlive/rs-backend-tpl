use mongodb::Database;
use entities::Message;
use super::{collection_names::MESSAGE, IRepository};

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