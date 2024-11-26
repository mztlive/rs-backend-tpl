/// RBAC Actor 模块
///
/// 该模块实现了一个基于 Actor 模式的 RBAC 权限检查系统。
/// 通过 Actor 模式可以安全地处理并发的权限检查请求。
use tokio::sync::{
    mpsc::{self, Receiver},
    oneshot,
};

use super::{RBACEnforcer, RBACRoleStore, RBACUserStore};
use mongodb::Database;

/// RBAC Actor 错误类型
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// RBAC 相关错误
    #[error("RBAC error: {0}")]
    RBACError(#[from] super::Error),

    /// 其他错误
    #[error("Other error: {0}")]
    OtherError(String),
}

impl From<String> for Error {
    fn from(err: String) -> Self {
        Error::OtherError(err)
    }
}

/// Actor 命令枚举
///
/// 定义了 Actor 可以处理的所有命令类型
pub enum Command {
    /// 检查权限命令
    CheckPermission {
        /// 用户标识
        user: String,
        /// 操作标识
        action: String,
        /// 用于返回检查结果的 channel
        respond_to: oneshot::Sender<bool>,
    },
    /// 重置权限策略命令
    Reset,
}

/// RBAC Actor 结构体
///
/// 负责处理权限检查的核心逻辑
struct Actor {
    /// 接收命令的 channel
    receiver: Receiver<Command>,
    /// RBAC 执行器实例
    enforcer: RBACEnforcer,
}

impl Actor {
    /// 创建新的 Actor 实例
    ///
    /// # 参数
    ///
    /// * `receiver` - 接收命令的 channel
    /// * `database` - MongoDB 数据库实例
    /// * `role_fetcher` - 角色数据获取器
    /// * `user_fetcher` - 用户数据获取器
    ///
    /// # 返回值
    ///
    /// 返回 Actor 实例的 Result 包装
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

    /// 处理接收到的命令
    ///
    /// # 参数
    ///
    /// * `command` - 待处理的命令
    ///
    /// # 返回值
    ///
    /// 返回处理结果
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

/// Actor 运行函数
///
/// 持续监听并处理接收到的命令
async fn run_actor(mut actor: Actor) {
    while let Some(command) = actor.receiver.recv().await {
        if let Err(err) = actor.handle_message(command).await {
            println!("Failed to handle message: {}", err);
        }
    }
}

/// Actor 处理器
///
/// 提供与 Actor 通信的公开接口
#[derive(Clone)]
pub struct ActorHandler {
    /// 发送命令的 channel
    sender: mpsc::Sender<Command>,
}

impl ActorHandler {
    /// 创建新的 ActorHandler 实例
    ///
    /// # 参数
    ///
    /// * `database` - MongoDB 数据库实例
    /// * `role_fetcher` - 角色数据获取器
    /// * `user_fetcher` - 用户数据获取器
    ///
    /// # 返回值
    ///
    /// 返回 ActorHandler 实例
    pub async fn new<R, U>(database: Database, role_fetcher: R, user_fetcher: U) -> Self
    where
        R: RBACRoleStore + 'static,
        U: RBACUserStore + 'static,
    {
        let (sender, receiver) = mpsc::channel(100);
        let actor = Actor::new(receiver, database, role_fetcher, user_fetcher)
            .await
            .expect("Failed to create RBAC actor");

        tokio::spawn(run_actor(actor));

        Self { sender }
    }

    /// 检查权限
    ///
    /// # 参数
    ///
    /// * `user` - 用户标识
    /// * `action` - 操作标识
    ///
    /// # 返回值
    ///
    /// 返回权限检查结果
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

    /// 重置权限策略
    ///
    /// # 返回值
    ///
    /// 返回重置操作的结果
    pub async fn reset(&self) -> Result<(), String> {
        self.sender
            .send(Command::Reset)
            .await
            .map_err(|err| format!("cannot reset rbac policies: {0}", err))
    }
}
