use database::repositories::{role::RoleRepository, IRepository};
use entities::{Role, RouteItem};

use crate::errors::Result;
use crate::utils::next_id;

pub struct RoleService {
    repo: RoleRepository,
}

impl RoleService {
    pub fn new() -> Self {
        Self {
            repo: RoleRepository::new(),
        }
    }

    pub async fn create_role(
        &self,
        name: String,
        permissions: Vec<RouteItem>,
        db: &mongodb::Database,
    ) -> Result<()> {
        let id = next_id().await;
        let role = Role::new(id, name, permissions);

        self.repo.create(&role, db).await?;
        Ok(())
    }

    pub async fn get_role_list(&self, db: &mongodb::Database) -> Result<Vec<Role>> {
        let roles = self.repo.find_all(db).await?;
        Ok(roles)
    }

    pub async fn update_role(
        &self,
        id: String,
        name: Option<String>,
        permissions: Option<Vec<RouteItem>>,
        db: &mongodb::Database,
    ) -> Result<()> {
        let mut role = self.repo.find_by_id(&id, db).await?.ok_or("角色不存在")?;

        if let Some(name) = name {
            role.name = name;
        }

        if let Some(permissions) = permissions {
            role.permissions = permissions;
        }

        self.repo.update(&role, db).await?;
        Ok(())
    }

    pub async fn delete_role(&self, id: String, db: &mongodb::Database) -> Result<()> {
        let mut role = self.repo.find_by_id(&id, db).await?.ok_or("角色不存在")?;

        // 检查是否有管理员正在使用该角色
        let admin_repo = database::repositories::user::AdminRepository::new();
        let admins = admin_repo.find_all(db).await?;
        let role_in_use = admins.iter().any(|admin| admin.role_name == role.name);

        if role_in_use {
            return Err("该角色正在使用中,无法删除".into());
        }

        role.base.delete();
        self.repo.update(&role, db).await?;

        Ok(())
    }

    pub async fn get_role_by_id(&self, id: String, db: &mongodb::Database) -> Result<Option<Role>> {
        let role = self.repo.find_by_id(&id, db).await?;
        Ok(role)
    }

    pub async fn get_role_by_name(&self, name: &str, db: &mongodb::Database) -> Result<Option<Role>> {
        let roles = self.repo.find_all(db).await?;
        let role = roles.into_iter().find(|r| r.name == name);
        Ok(role)
    }
}
