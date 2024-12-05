use chrono::Local;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BaseModel {
    pub id: String,
    pub version: u64,
    pub created_at: u64,
    pub updated_at: u64,
    pub deleted_at: u64,
}

impl BaseModel {
    pub fn new(id: String) -> Self {
        let now = Local::now().timestamp();
        Self {
            id,
            version: 1,
            created_at: now as u64,
            updated_at: now as u64,
            deleted_at: 0,
        }
    }

    pub fn delete(&mut self) {
        self.deleted_at = Local::now().timestamp() as u64;
    }

    pub fn fake() -> Self {
        Self {
            id: "fake".to_string(),
            ..Default::default()
        }
    }
}

pub trait HasId {
    fn get_id(&self) -> &str;
}

pub trait HasVersion {
    fn get_version(&self) -> u64;
}

pub trait Entity: HasId + HasVersion {}
