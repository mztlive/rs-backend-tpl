use crate::errors::Result;
use database::repositories::{role::RoleRepository, user::AdminRepository, IRepository};
use entities::{Admin, Secret};

pub struct AdminService {
    admin_repo: AdminRepository,
    role_repo: RoleRepository,
}

impl AdminService {
    pub fn new() -> Self {
        Self {
            admin_repo: AdminRepository::new(),
            role_repo: RoleRepository::new(),
        }
    }

    pub async fn create_admin(
        &self,
        account: String,
        password: String,
        name: String,
        role_name: String,
        db: &mongodb::Database,
    ) -> Result<()> {
        // 检查账号是否已存在
        if let Some(_) = self.admin_repo.find_by_account(&account, db).await? {
            return Err("账号已存在".into());
        }

        let secret = Secret::new(account.clone(), password)?;
        let id = crate::utils::next_id().await;

        let user = Admin::new(id, secret, name, role_name);
        self.admin_repo.create(&user, db).await?;

        Ok(())
    }

    pub async fn get_admin_list(&self, db: &mongodb::Database) -> Result<Vec<Admin>> {
        let users = self.admin_repo.find_all(db).await?;
        Ok(users)
    }

    pub async fn update_admin(
        &self,
        id: String,
        name: Option<String>,
        password: Option<String>,
        role_name: Option<String>,
        db: &mongodb::Database,
    ) -> Result<()> {
        let mut user = self.admin_repo.find_by_id(&id, db).await?.ok_or("管理员不存在")?;

        if let Some(name) = name {
            user.name = name;
        }

        if let Some(password) = password {
            user.secret.change_password(password);
        }

        if let Some(role_name) = role_name {
            user.role_name = role_name;
        }

        self.admin_repo.update(&user, db).await?;
        Ok(())
    }

    pub async fn update_admin_role(
        &self,
        id: String,
        role_name: String,
        db: &mongodb::Database,
    ) -> Result<()> {
        let mut user = self.admin_repo.find_by_id(&id, db).await?.ok_or("管理员不存在")?;

        // 检查新角色是否存在
        let roles = self.role_repo.find_all(db).await?;
        let role_exists = roles.iter().any(|role| role.name == role_name);

        if !role_exists {
            return Err("指定的角色不存在".into());
        }

        user.role_name = role_name;
        self.admin_repo.update(&user, db).await?;

        Ok(())
    }

    pub async fn delete_admin(&self, id: String, db: &mongodb::Database) -> Result<()> {
        let mut user = self.admin_repo.find_by_id(&id, db).await?.ok_or("管理员不存在")?;

        user.base.delete();
        self.admin_repo.update(&user, db).await?;

        Ok(())
    }
}
