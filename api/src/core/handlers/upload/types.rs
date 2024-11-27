use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct UploadResponse {
    pub url: String,
}
