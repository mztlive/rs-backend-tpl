use entity_base::BaseModel;
use entity_derive::Entity;
use serde::{Deserialize, Serialize};

use rbac::RBACRole;

/// 表示一个路由项的结构体
///
/// # 字段
///
/// * `module` - 模块名称
/// * `path` - 路由路径
/// * `description` - 路由描述
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteItem {
    pub module: String,
    pub path: String,
    pub description: String,
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
    /// 将角色转换为 Casbin 策略格式
    ///
    /// # 返回值
    ///
    /// 返回一个二维字符串数组,每个内部数组包含角色名和对应的权限路径
    fn to_casbin_policy(&self) -> Vec<Vec<String>> {
        let mut out: Vec<Vec<String>> = vec![];

        self.permissions
            .iter()
            .for_each(|p| out.push(vec![self.name.clone(), p.path.clone()]));

        out
    }
}
