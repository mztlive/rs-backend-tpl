use crate::errors::Error;
use serde::{Deserialize, Serialize};

/// 表示用户认证信息的结构体
///
/// # 字段
///
/// * `account` - 用户账号
/// * `password` - 用户密码(MD5加密后的哈希值)
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Secret {
    pub account: String,
    pub password: String,
}

impl Secret {
    /// 创建一个用于测试的假Secret实例
    ///
    /// # 返回值
    ///
    /// 返回一个包含假数据的Secret实例
    pub fn fake() -> Self {
        let password = format!("{:x}", md5::compute("fake".as_bytes()));

        Self {
            account: "fake".to_string(),
            password,
        }
    }

    /// 创建一个新的Secret实例
    ///
    /// # 参数
    ///
    /// * `account` - 用户账号
    /// * `password` - 用户原始密码
    ///
    /// # 返回值
    ///
    /// 返回Result,成功时包含新的Secret实例,失败时返回错误
    pub fn new(account: String, password: String) -> std::result::Result<Self, Error> {
        if password.is_empty() {
            return Err(Error::LogicError("密码不能为空".to_string()));
        }

        let password = format!("{:x}", md5::compute(password.as_bytes()));

        Ok(Self { account, password })
    }

    /// 修改用户密码
    ///
    /// # 参数
    ///
    /// * `password` - 新的密码
    pub fn change_password(&mut self, password: String) {
        self.password = format!("{:x}", md5::compute(password.as_bytes()));
    }

    /// 验证密码是否匹配
    ///
    /// # 参数
    ///
    /// * `password` - 待验证的密码
    ///
    /// # 返回值
    ///
    /// 如果密码匹配返回true,否则返回false
    pub fn is_match(&self, password: &str) -> bool {
        format!("{:x}", md5::compute(password.as_bytes())) == self.password
    }
}
