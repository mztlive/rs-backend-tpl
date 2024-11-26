use tokio::sync::{
    mpsc::{self, Receiver},
    oneshot,
};

use mongodb::Database;
use rbac::{RBACEnforcer, RBACRoleStore, RBACUserStore};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("RBAC error: {0}")]
    RBACError(#[from] rbac::Error),

    #[error("Other error: {0}")]
    OtherError(String),
}

impl From<String> for Error {
    fn from(err: String) -> Self {
        Error::OtherError(err)
    }
}

pub enum Command {
    CheckPermission {
        user: String,
        action: String,
        respond_to: oneshot::Sender<bool>,
    },
    Reset,
}

struct RbacActor {
    receiver: Receiver<Command>,
    enforcer: RBACEnforcer,
}

impl RbacActor {
    pub async fn new<R, U>(
        receiver: Receiver<Command>,
        database: Database,
        role_fetcher: R,
        user_fetcher: U,
    ) -> Result<Self, Error>
    where
        R: RBACRoleStore + 'static,
        U: RBACUserStore + 'static,
    {
        let enforcer = RBACEnforcer::new(database, role_fetcher, user_fetcher).await?;

        Ok(Self { receiver, enforcer })
    }

    async fn handle_message(&mut self, command: Command) -> Result<(), Error> {
        match command {
            Command::CheckPermission {
                user,
                action,
                respond_to,
            } => {
                let is_ok = self.enforcer.check_permission(&user, &action)?;
                respond_to.send(is_ok).map_err(|err| err.to_string())?;
            }
            Command::Reset => {
                self.enforcer.load_policies().await?;
            }
        }
        Ok(())
    }
}

async fn run_actor(mut actor: RbacActor) {
    while let Some(command) = actor.receiver.recv().await {
        if let Err(err) = actor.handle_message(command).await {
            println!("Failed to handle message: {}", err);
        }
    }
}

#[derive(Clone)]
pub struct RbacActorHandler {
    sender: mpsc::Sender<Command>,
}

impl RbacActorHandler {
    pub async fn new<R, U>(database: Database, role_fetcher: R, user_fetcher: U) -> Self
    where
        R: RBACRoleStore + 'static,
        U: RBACUserStore + 'static,
    {
        let (sender, receiver) = mpsc::channel(100);
        let actor = RbacActor::new(receiver, database, role_fetcher, user_fetcher)
            .await
            .expect("Failed to create RBAC actor");

        tokio::spawn(run_actor(actor));

        Self { sender }
    }

    pub async fn check_permission(&self, user: String, action: String) -> Result<bool, String> {
        let (respond_to, response) = oneshot::channel();
        self.sender
            .send(Command::CheckPermission {
                user,
                action,
                respond_to,
            })
            .await
            .map_err(|err| format!("cannot send message to rbac actor: {0}", err))?;

        response
            .await
            .map_err(|err| format!("cannot receive response from rbac actor: {0}", err))
    }

    pub async fn reset(&self) -> Result<(), String> {
        self.sender
            .send(Command::Reset)
            .await
            .map_err(|err| format!("cannot reset rbac policies: {0}", err))
    }
}
