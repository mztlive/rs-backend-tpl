use casbin::{CoreApi, Enforcer, MgmtApi, RbacApi};

use super::{
    error::Result,
    model::{RBACRole, RBACRoleStore, RBACUser, RBACUserStore},
};

const MODEL: &str = r#"
[request_definition]
r = sub, action

[policy_definition]
p = sub, action

[role_definition]
g = _, _

[policy_effect]
e = some(where (p.eft == allow))

[matchers]
m = g(r.sub, p.sub) && r.action == p.action || r.sub == "bozzasggmy"
"#;

pub struct RBACEnforcer {
    enforcer: Enforcer,
    role_store: Box<dyn RBACRoleStore>,
    user_store: Box<dyn RBACUserStore>,
}

impl RBACEnforcer {
    pub async fn new<R, U>(role_fetcher: R, user_fetcher: U) -> Result<Self>
    where
        R: RBACRoleStore + 'static,
        U: RBACUserStore + 'static,
    {
        let model = casbin::DefaultModel::from_str(MODEL).await?;
        let adapter = casbin::MemoryAdapter::default();
        let enforcer = Enforcer::new(model, adapter).await?;

        let mut rbac = Self {
            enforcer,
            role_store: Box::new(role_fetcher),
            user_store: Box::new(user_fetcher),
        };

        rbac.load_policies().await?;

        Ok(rbac)
    }

    pub async fn load_policies(&mut self) -> Result<()> {
        if let Err(err) = self.enforcer.clear_policy().await {
            println!("Failed to clear policies: {}", err);
        }

        let all_roles: Vec<Box<dyn RBACRole>> = self.role_store.find_all().await?;
        let all_users: Vec<Box<dyn RBACUser>> = self.user_store.find_all().await?;

        for role in all_roles {
            for policy in role.to_casbin_policy() {
                self.enforcer.add_policy(policy).await?;
            }
        }

        for user in all_users {
            self.enforcer
                .add_role_for_user(&user.account(), &user.role_name(), None)
                .await?;
        }

        Ok(())
    }

    pub fn check_permission(&self, user: &str, action: &str) -> Result<bool> {
        Ok(self.enforcer.enforce((user, action))?)
    }
}
