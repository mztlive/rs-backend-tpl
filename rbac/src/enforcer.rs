use casbin::{CoreApi, Enforcer, MgmtApi, RbacApi};

use super::{
    errors::Result,
    model::{RBACRole, RBACRoleStore, RBACUser, RBACUserStore},
};

const MODEL: &str = r#"
[request_definition]
r = sub, method, path

[policy_definition]
p = sub, method, path

[role_definition]
g = _, _

[policy_effect]
e = some(where (p.eft == allow))

[matchers]
m = g(r.sub, p.sub) && (r.method == p.method || p.method == "*") && keyMatch2(r.path, p.path) || r.sub == "fake"
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

    pub async fn check_permission(&self, user: &str, method: &str, path: &str) -> Result<bool> {
        Ok(self.enforcer.enforce((user, method, path))?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use std::sync::Arc;

    #[derive(Clone)]
    struct MockRole {
        name: String,
        permissions: Vec<(String, String, String)>,
    }

    impl RBACRole for MockRole {
        fn to_casbin_policy(&self) -> Vec<Vec<String>> {
            self.permissions
                .iter()
                .map(|(method, path, _)| vec![self.name.clone(), method.clone(), path.clone()])
                .collect()
        }

        fn get_name(&self) -> String {
            self.name.clone()
        }

        fn check_permission(&self, method: &str, path: &str) -> bool {
            self.permissions.iter().any(|(m, p, _)| m == method && p == path)
        }
    }

    #[derive(Clone)]
    struct MockUser {
        account: String,
        role: String,
    }

    impl RBACUser for MockUser {
        fn account(&self) -> String {
            self.account.clone()
        }

        fn role_name(&self) -> String {
            self.role.clone()
        }
    }

    struct MockRoleStore {
        roles: Arc<Vec<MockRole>>,
    }

    #[async_trait]
    impl RBACRoleStore for MockRoleStore {
        async fn find_all(&self) -> Result<Vec<Box<dyn RBACRole>>> {
            Ok(self
                .roles
                .iter()
                .cloned()
                .map(|r| Box::new(r) as Box<dyn RBACRole>)
                .collect())
        }
    }

    struct MockUserStore {
        users: Arc<Vec<MockUser>>,
    }

    #[async_trait]
    impl RBACUserStore for MockUserStore {
        async fn find_all(&self) -> Result<Vec<Box<dyn RBACUser>>> {
            Ok(self
                .users
                .iter()
                .cloned()
                .map(|u| Box::new(u) as Box<dyn RBACUser>)
                .collect())
        }
    }

    #[tokio::test]
    async fn test_basic_permission_check() -> Result<()> {
        // Setup test data
        let admin_role = MockRole {
            name: "admin".to_string(),
            permissions: vec![
                ("GET".to_string(), "/users".to_string(), "List users".to_string()),
                (
                    "POST".to_string(),
                    "/users".to_string(),
                    "Create user".to_string(),
                ),
            ],
        };

        let user_role = MockRole {
            name: "user".to_string(),
            permissions: vec![("GET".to_string(), "/users".to_string(), "List users".to_string())],
        };

        let roles = Arc::new(vec![admin_role, user_role]);
        let users = Arc::new(vec![
            MockUser {
                account: "admin@example.com".to_string(),
                role: "admin".to_string(),
            },
            MockUser {
                account: "user@example.com".to_string(),
                role: "user".to_string(),
            },
        ]);

        let enforcer = RBACEnforcer::new(MockRoleStore { roles }, MockUserStore { users }).await?;

        // Test admin permissions
        assert!(
            enforcer
                .check_permission("admin@example.com", "GET", "/users")
                .await?
        );
        assert!(
            enforcer
                .check_permission("admin@example.com", "POST", "/users")
                .await?
        );

        // Test user permissions
        assert!(
            enforcer
                .check_permission("user@example.com", "GET", "/users")
                .await?
        );
        assert!(
            !enforcer
                .check_permission("user@example.com", "POST", "/users")
                .await?
        );

        // Test non-existent user
        assert!(
            !enforcer
                .check_permission("nobody@example.com", "GET", "/users")
                .await?
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_path_pattern_matching() -> Result<()> {
        let role = MockRole {
            name: "editor".to_string(),
            permissions: vec![
                (
                    "PUT".to_string(),
                    "/posts/:id".to_string(),
                    "Edit post".to_string(),
                ),
                (
                    "DELETE".to_string(),
                    "/posts/:id".to_string(),
                    "Delete post".to_string(),
                ),
            ],
        };

        let roles = Arc::new(vec![role]);
        let users = Arc::new(vec![MockUser {
            account: "editor@example.com".to_string(),
            role: "editor".to_string(),
        }]);

        let enforcer = RBACEnforcer::new(MockRoleStore { roles }, MockUserStore { users }).await?;

        // Test path pattern matching
        assert!(
            enforcer
                .check_permission("editor@example.com", "PUT", "/posts/123")
                .await?
        );
        assert!(
            enforcer
                .check_permission("editor@example.com", "DELETE", "/posts/456")
                .await?
        );

        // Test invalid paths
        assert!(
            !enforcer
                .check_permission("editor@example.com", "PUT", "/posts/")
                .await?
        );
        assert!(
            !enforcer
                .check_permission("editor@example.com", "PUT", "/posts/123/comments")
                .await?
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_wildcard_method() -> Result<()> {
        let role = MockRole {
            name: "viewer".to_string(),
            permissions: vec![(
                "*".to_string(),
                "/public/*".to_string(),
                "Access public content".to_string(),
            )],
        };

        let roles = Arc::new(vec![role]);
        let users = Arc::new(vec![MockUser {
            account: "viewer@example.com".to_string(),
            role: "viewer".to_string(),
        }]);

        let enforcer = RBACEnforcer::new(MockRoleStore { roles }, MockUserStore { users }).await?;

        // Test wildcard method matching
        assert!(
            enforcer
                .check_permission("viewer@example.com", "GET", "/public/page1")
                .await?
        );
        assert!(
            enforcer
                .check_permission("viewer@example.com", "POST", "/public/page1")
                .await?
        );

        // Test invalid paths
        assert!(
            !enforcer
                .check_permission("viewer@example.com", "GET", "/private/page1")
                .await?
        );

        Ok(())
    }
}
