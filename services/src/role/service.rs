use entities::Role;

use crate::{
    admin::{IAdminRepository, IRoleRepository},
    errors::Result,
};
use libs::next_id;

use super::dto::{CreateRoleParams, UpdateRoleParams};

pub struct RoleService<R: IRoleRepository, A: IAdminRepository> {
    repo: R,
    admin_repo: A,
}

impl<R: IRoleRepository, A: IAdminRepository> RoleService<R, A> {
    pub fn new(repo: R, admin_repo: A) -> Self {
        Self { repo, admin_repo }
    }

    pub async fn create_role(&self, params: CreateRoleParams) -> Result<()> {
        let id = next_id().await;
        let role = Role::new(id, params.name, params.permissions);

        self.repo.create(&role).await?;
        Ok(())
    }

    pub async fn get_role_list(&self) -> Result<Vec<Role>> {
        let roles = self.repo.find_all().await?;
        Ok(roles)
    }

    pub async fn update_role(&self, params: UpdateRoleParams) -> Result<()> {
        let mut role = self.repo.find_by_id(&params.id).await?.ok_or("角色不存在")?;

        if let Some(name) = params.name {
            role.name = name;
        }

        if let Some(permissions) = params.permissions {
            role.permissions = permissions;
        }

        self.repo.update(&role).await?;
        Ok(())
    }

    pub async fn delete_role(&self, id: String) -> Result<()> {
        let mut role = self.repo.find_by_id(&id).await?.ok_or("角色不存在")?;

        // 检查是否有管理员正在使用该角色
        let admins = self.admin_repo.find_all().await?;
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
