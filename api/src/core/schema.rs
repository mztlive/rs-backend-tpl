use serde::Deserialize;

/// 用户ID包装类型
#[derive(Debug, Clone, Default, Deserialize)]
pub struct UserID(pub String);

/// 账号包装类型
#[derive(Debug, Clone, Deserialize)]
pub struct Account(pub String);

impl From<UserID> for String {
    fn from(user_id: UserID) -> Self {
        user_id.0
    }
}

impl From<Account> for String {
    fn from(account: Account) -> Self {
        account.0
    }
}
