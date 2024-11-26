use casbin::{CoreApi, Enforcer, MgmtApi, RbacApi};
use mongodb::Database;

use super::model::{Error, RBACRole, RBACRoleFetcher, RBACUser, RBACUserFetcher};

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
    database: Database,
    role_fetcher: Box<dyn RBACRoleFetcher>,
    user_fetcher: Box<dyn RBACUserFetcher>,
}

impl RBACEnforcer {
    pub async fn new<R, U>(database: Database, role_fetcher: R, user_fetcher: U) -> Result<Self, Error>
    where
        R: RBACRoleFetcher + 'static,
        U: RBACUserFetcher + 'static,
    {
        let model = casbin::DefaultModel::from_str(MODEL).await?;
        let adapter = casbin::MemoryAdapter::default();
        let enforcer = Enforcer::new(model, adapter).await?;

        let mut rbac = Self {
            enforcer,
            database,
            role_fetcher: Box::new(role_fetcher),
            user_fetcher: Box::new(user_fetcher),
        };

        rbac.load_policies().await?;

        Ok(rbac)
    }

    pub async fn load_policies(&mut self) -> Result<(), Error> {
        if let Err(err) = self.enforcer.clear_policy().await {
            println!("Failed to clear policies: {}", err);
        }

        let all_roles: Vec<Box<dyn RBACRole>> = self.role_fetcher.find_all(&self.database).await?;
        let all_users: Vec<Box<dyn RBACUser>> = self.user_fetcher.find_all(&self.database).await?;
        let roles_len = all_roles.len();
        let users_len = all_users.len();

        for role in all_roles {
            for policy in role.to_casbin_policy() {
                println!("policy: {:?}", policy);
                self.enforcer.add_policy(policy).await?;
            }
        }

        for user in all_users {
            self.enforcer
                .add_role_for_user(&user.account(), &user.role_name(), None)
                .await?;
        }

        println!("load {} roles and {} users", roles_len, users_len);

        Ok(())
    }

    pub fn check_permission(&self, user: &str, action: &str) -> Result<bool, Error> {
        Ok(self.enforcer.enforce((user, action))?)
    }
}
