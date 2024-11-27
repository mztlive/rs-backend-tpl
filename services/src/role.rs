use database::repositories::user::AdminRepository;
use database::repositories::{role::RoleRepository, IRepository};
use entities::{Role, RouteItem};
use mongodb::Database;

use crate::errors::Result;
use crate::utils::next_id;

pub struct RoleService {
    repo: RoleRepository,
}

impl RoleService {
    pub fn new(database: Database) -> Self {
        Self {
            repo: RoleRepository::new(database),
        }
    }

    pub async fn create_role(&self, name: String, permissions: Vec<RouteItem>) -> Result<()> {
        let id = next_id().await;
        let role = Role::new(id, name, permissions);

        self.repo.create(&role).await?;
        Ok(())
    }

    pub async fn get_role_list(&self) -> Result<Vec<Role>> {
        let roles = self.repo.find_all().await?;
        Ok(roles)
    }

    pub async fn update_role(
        &self,
        id: String,
        name: Option<String>,
        permissions: Option<Vec<RouteItem>>,
    ) -> Result<()> {
        let mut role = self.repo.find_by_id(&id).await?.ok_or("角色不存在")?;

        if let Some(name) = name {
            role.name = name;
        }

        if let Some(permissions) = permissions {
            role.permissions = permissions;
        }

        self.repo.update(&role).await?;
        Ok(())
    }

    pub async fn delete_role(&self, id: String) -> Result<()> {
        let mut role = self.repo.find_by_id(&id).await?.ok_or("角色不存在")?;

        // 检查是否有管理员正在使用该角色
        let admin_repo = AdminRepository::new(self.repo.get_database().clone());
        let admins = admin_repo.find_all().await?;
        let role_in_use = admins.iter().any(|admin| admin.role_name == role.name);

        if role_in_use {
            return Err("该角色正在使用中,无法删除".into());
        }

        role.base.delete();
        self.repo.update(&role).await?;

        Ok(())
    }

    pub async fn get_role_by_id(&self, id: String) -> Result<Option<Role>> {
        let role = self.repo.find_by_id(&id).await?;
        Ok(role)
    }

    pub async fn get_role_by_name(&self, name: &str) -> Result<Option<Role>> {
        let roles = self.repo.find_all().await?;
        let role = roles.into_iter().find(|r| r.name == name);
        Ok(role)
    }
}
