use database::repositories::{AdminRepository, InternalMessageRepository, MessageRepository, RoleRepository};
use mongodb::Database;
use services::{AdminService, InternalMessageService, MessageService, RoleService};

#[derive(Clone)]
pub struct ServiceFactory {
    db: Database,
}

impl ServiceFactory {
    pub fn new(db: Database) -> Self {
        Self { db }
    }

    pub fn admin_service(&self) -> AdminService<AdminRepository, RoleRepository> {
        let admin_repo = AdminRepository::new(self.db.clone());
        let role_repo = RoleRepository::new(self.db.clone());
        AdminService::new(admin_repo, role_repo)
    }

    pub fn role_service(&self) -> RoleService<RoleRepository, AdminRepository> {
        let role_repo = RoleRepository::new(self.db.clone());
        let admin_repo = AdminRepository::new(self.db.clone());
        RoleService::new(role_repo, admin_repo)
    }

    pub fn notify_service(&self) -> MessageService<MessageRepository, InternalMessageRepository> {
        let message_repo = MessageRepository::new(self.db.clone());
        let internal_message_repo = InternalMessageRepository::new(self.db.clone());
        MessageService::new(message_repo, internal_message_repo)
    }

    pub fn internal_message_service(&self) -> InternalMessageService<InternalMessageRepository> {
        let internal_message_repo = InternalMessageRepository::new(self.db.clone());
        InternalMessageService::new(internal_message_repo)
    }
}

pub struct Container {
    pub services: ServiceFactory,
}

impl Container {
    pub fn new(db: Database) -> Self {
        Self {
            services: ServiceFactory::new(db),
        }
    }
}
