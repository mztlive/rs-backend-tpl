use crate::errors::Error;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Secret {
    pub account: String,
    pub password: String,
}

impl Secret {
    pub fn fake() -> Self {
        let password = format!("{:x}", md5::compute("fake".as_bytes()));

        Self {
            account: "fake".to_string(),
            password,
        }
    }

    pub fn new(account: String, password: String) -> std::result::Result<Self, Error> {
        if password.is_empty() {
            return Err(Error::LogicError("密码不能为空".to_string()));
        }

        let password = format!("{:x}", md5::compute(password.as_bytes()));

        Ok(Self { account, password })
    }

    pub fn change_password(&mut self, password: String) {
        self.password = format!("{:x}", md5::compute(password.as_bytes()));
    }

    pub fn is_match(&self, password: &str) -> bool {
        format!("{:x}", md5::compute(password.as_bytes())) == self.password
    }
}
