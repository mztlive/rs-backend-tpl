use serde::{Deserialize, Serialize};



/// 用户ID包装类型
#[derive(Debug, Clone, Default, Deserialize)]
pub struct UserID(pub String);

/// 账号包装类型
#[derive(Debug, Clone)]
pub struct Account(pub String);
