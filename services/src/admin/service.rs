use crate::errors::Result;
use entities::{Admin, Secret};

use super::repository::{IAdminRepository, IRoleRepository};
use super::dto::{CreateAdminParams, UpdateAdminParams, UpdateAdminRoleParams};

pub struct AdminService<A: IAdminRepository, R: IRoleRepository> {
    admin_repo: A,
    role_repo: R,
}

impl<A: IAdminRepository, R: IRoleRepository> AdminService<A, R> {
    pub fn new(admin_repo: A, role_repo: R) -> Self {
        Self {
            admin_repo,
            role_repo,
        }
    }

    pub async fn create_admin(&self, params: CreateAdminParams) -> Result<()> {
        // 检查账号是否已存在
        if let Some(_) = self.admin_repo.find_by_account(&params.account).await? {
            return Err("账号已存在".into());
        }

        if !self.role_repo.exists(&params.role_name).await? {
            return Err("指定的角色不存在".into());
        }

        let secret = Secret::new(params.account.clone(), params.password)?;
        let id = libs::next_id().await;

        let user = Admin::new(id, secret, params.name, params.role_name);
        self.admin_repo.create(&user).await?;

        Ok(())
    }

    pub async fn get_admin_list(&self) -> Result<Vec<Admin>> {
        let users = self.admin_repo.find_all().await?;
        Ok(users)
    }

    pub async fn update_admin(&self, params: UpdateAdminParams) -> Result<()> {
        let mut user = self
            .admin_repo
            .find_by_id(&params.id)
            .await?
            .ok_or("管理员不存在")?;

        if let Some(name) = params.name {
            user.name = name;
        }

        if let Some(password) = params.password {
            user.secret.change_password(password)?;
        }

        if let Some(role_name) = params.role_name {
            if !self.role_repo.exists(&role_name).await? {
                return Err("指定的角色不存在".into());
            }

            user.role_name = role_name;
        }

        self.admin_repo.update(&user).await?;
        Ok(())
    }

    pub async fn update_admin_role(&self, params: UpdateAdminRoleParams) -> Result<()> {
        let mut user = self
            .admin_repo
            .find_by_id(&params.id)
            .await?
            .ok_or("管理员不存在")?;

        // 检查新角色是否存在
        if !self.role_repo.exists(&params.role_name).await? {
            return Err("指定的角色不存在".into());
        }

        user.role_name = params.role_name;
        self.admin_repo.update(&user).await?;

        Ok(())
    }

    pub async fn delete_admin(&self, id: String) -> Result<()> {
        let mut user = self.admin_repo.find_by_id(&id).await?.ok_or("管理员不存在")?;

        user.base.delete();
        self.admin_repo.update(&user).await?;

        Ok(())
    }
}
