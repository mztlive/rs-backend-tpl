/// RBAC Actor 模块
///
/// 该模块实现了一个基于 Actor 模式的 RBAC 权限检查系统。
/// 通过 Actor 模式可以安全地处理并发的权限检查请求。
use tokio::sync::{
    mpsc::{self, Receiver},
    oneshot,
};

use super::{RBACEnforcer, RBACRoleStore, RBACUserStore};

/// RBAC Actor 错误类型
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// RBAC 相关错误
    #[error("RBAC error: {0}")]
    RBACError(#[from] super::Error),

    /// 其他错误
    #[error("Other error: {0}")]
    OtherError(String),

    /// 消息相关错误
    #[error("Message  error: {0}")]
    MessageError(String),
}

impl From<String> for Error {
    fn from(err: String) -> Self {
        Error::OtherError(err)
    }
}

impl From<oneshot::error::RecvError> for Error {
    fn from(err: oneshot::error::RecvError) -> Self {
        Error::MessageError(err.to_string())
    }
}

impl<T> From<tokio::sync::mpsc::error::SendError<T>> for Error {
    fn from(err: tokio::sync::mpsc::error::SendError<T>) -> Self {
        Error::MessageError(err.to_string())
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
        /// 方法标识
        method: String,
        /// 路径标识
        path: String,
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
    /// * `role_fetcher` - 角色数据获取器
    /// * `user_fetcher` - 用户数据获取器
    ///
    /// # 返回值
    ///
    /// 返回 Actor 实例的 Result 包装
    pub async fn new<R, U>(
        receiver: Receiver<Command>,
        role_fetcher: R,
        user_fetcher: U,
    ) -> Result<Self, Error>
    where
        R: RBACRoleStore + 'static,
        U: RBACUserStore + 'static,
    {
        let enforcer = RBACEnforcer::new(role_fetcher, user_fetcher).await?;

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
                method,
                path,
                respond_to,
            } => {
                let is_ok = self.enforcer.check_permission(&user, &method, &path).await?;
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
    /// * `role_fetcher` - 角色数据获取器
    /// * `user_fetcher` - 用户数据获取器
    ///
    /// # 返回值
    ///
    /// 返回 ActorHandler 实例
    pub async fn new<R, U>(role_fetcher: R, user_fetcher: U) -> Self
    where
        R: RBACRoleStore + 'static,
        U: RBACUserStore + 'static,
    {
        let (sender, receiver) = mpsc::channel(100);
        let actor = Actor::new(receiver, role_fetcher, user_fetcher)
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
    /// * `method` - 方法标识
    /// * `path` - 路径标识
    ///
    /// # 返回值
    ///
    /// 返回权限检查结果
    pub async fn check_permission(&self, user: String, method: String, path: String) -> Result<bool, Error> {
        let (respond_to, response) = oneshot::channel();
        self.sender
            .send(Command::CheckPermission {
                user,
                method,
                path,
                respond_to,
            })
            .await?;

        let is_ok = response.await?;

        Ok(is_ok)
    }

    pub async fn reset(&self) -> Result<(), Error> {
        self.sender.send(Command::Reset).await?;
        Ok(())
    }
}
