use entity_base::BaseModel;
use entity_derive::Entity;
use regex::Regex;
use serde::{Deserialize, Serialize};

use rbac::RBACRole;

/// 表示一个路由项的结构体
///
/// # 字段
///
/// * `module` - 模块名称
/// * `method` - HTTP 方法
/// * `path` - 路由路径
/// * `description` - 路由描述
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteItem {
    pub module: String,
    pub method: String,
    pub path: String,
    pub description: String,
}

impl RouteItem {
    /// 创建一个新的路由项实例
    ///
    /// # 参数
    ///
    /// * `module` - 模块名称
    /// * `method` - HTTP 方法
    /// * `path` - 路由路径
    /// * `description` - 路由描述
    ///
    /// # 返回值
    ///
    /// 返回一个新的 RouteItem 实例
    pub fn new(module: String, method: String, path: String, description: String) -> Self {
        Self {
            module,
            method,
            path,
            description,
        }
    }

    /// 检查路由项是否匹配给定的请求
    ///
    /// # 参数
    ///
    /// * `method` - HTTP 方法
    /// * `path` - 路由路径
    ///
    /// # 返回值
    ///
    /// 如果路由项匹配给定的请求,则返回 true;否则返回 false
    pub fn matches(&self, method: &str, path: &str) -> bool {
        if self.method != method && self.method != "*" {
            return false;
        }

        // 将路由模式转换为正则表达式
        let path_pattern = self.path.replace(":id", r"[^/]+");
        let re = Regex::new(&format!("^{}$", path_pattern));

        re.is_ok() && re.unwrap().is_match(path)
    }
}

/// 表示一个角色的结构体
///
/// # 字段
///
/// * `base` - 基础模型字段
/// * `name` - 角色名称
/// * `permissions` - 角色拥有的权限列表
#[derive(Debug, Serialize, Deserialize, Clone, Entity)]
pub struct Role {
    #[serde(flatten)]
    pub base: BaseModel,
    pub name: String,
    pub permissions: Vec<RouteItem>,
}

impl Role {
    /// 创建一个新的角色实例
    ///
    /// # 参数
    ///
    /// * `id` - 角色ID
    /// * `name` - 角色名称
    /// * `permissions` - 角色权限列表
    ///
    /// # 返回值
    ///
    /// 返回一个新的 Role 实例
    pub fn new(id: String, name: String, permissions: Vec<RouteItem>) -> Self {
        Role {
            base: BaseModel::new(id),
            name,
            permissions,
        }
    }
}

impl RBACRole for Role {
    fn to_casbin_policy(&self) -> Vec<Vec<String>> {
        let mut out: Vec<Vec<String>> = vec![];

        for permission in &self.permissions {
            out.push(vec![
                self.name.clone(),
                permission.method.clone(),
                permission.path.clone(),
            ]);
        }

        out
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn check_permission(&self, method: &str, path: &str) -> bool {
        self.permissions.iter().any(|p| p.matches(method, path))
    }
}
