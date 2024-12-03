//! # 配置管理模块
//!
//! 本模块为应用程序提供全面的配置管理功能,
//! 支持基于文件和基于 Nacos 的配置源。
//!
//! ## 功能特性
//!
//! * 从 TOML 文件加载配置
//! * 集成 Nacos 配置中心
//! * 通过 RwLock 实现线程安全的配置访问
//! * 使用 Nacos 时支持配置热重载
//! * 通过命令行参数解析选择配置源
//!
//! ## 使用方法
//!
//! ### 基于文件的配置
//!
//! ```no_run
//! use config::SafeConfig;
//!
//! #[tokio::main]
//! async fn main() {
//!     let config = SafeConfig::from_args().await.unwrap();
//!     let current_config = config.get_config().await.unwrap();
//!     println!("服务器端口: {}", current_config.app.port);
//! }
//! ```
//!
//! ### 基于 Nacos 的配置
//!
//! ```no_run
//! // 启用 nacos 运行:
//! // ./program --enable-nacos --nacos-addr="http://localhost:8848"
//! //           --nacos-namespace="public" --nacos-group="DEFAULT_GROUP"
//! //           --nacos-data-id="config.toml"
//! ```
//!
//! ## 配置结构
//!
//! 配置被组织成几个嵌套的组件:
//! * `Config`: 顶层配置容器
//! * `AppConfig`: 应用程序特定设置
//! * `DatabaseConfig`: 数据库连接设置

use clap::Parser;
use command::Args;
use nacos::NacosConfig;
use nacos_watch::NacosConfigWatcher;
use serde::Deserialize;
use std::{path::Path, sync::Arc};
use tokio::sync::RwLock;

mod command;
mod errors;
mod nacos;
mod nacos_watch;
pub use errors::*;
pub use nacos::NacosConfigClient;

/// 包含所有应用程序设置的主配置结构。
///
/// 这个结构是所有配置设置的根容器,
/// 可以从 TOML 格式反序列化。
#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    /// 应用程序特定配置
    pub app: AppConfig,
    /// 数据库特定配置
    pub database: DatabaseConfig,
}

/// 应用程序特定的配置设置。
///
/// 包含控制核心应用程序行为的设置。
#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    /// 应用程序将监听的 HTTP 服务器端口号
    pub port: u16,
    /// 用于 JWT 令牌生成和验证的密钥
    pub secret: String,
    /// 上传文件存储的基础路径
    pub upload_path: String,
}

/// 数据库连接配置。
///
/// 包含建立数据库连接所需的所有设置。
#[derive(Debug, Deserialize, Clone)]
pub struct DatabaseConfig {
    /// 完整的数据库连接 URI,包括协议、主机和端口
    pub uri: String,
    /// 要连接的数据库名称
    pub db_name: String,
}

impl Config {
    /// 从指定路径加载 TOML 文件配置。
    ///
    /// # 参数
    ///
    /// * `path` - TOML 配置文件的路径
    ///
    /// # 返回
    ///
    /// * `Result<Self>` - 解析后的配置或错误
    pub async fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = tokio::fs::read_to_string(path).await?;
        let config = toml::from_str(&content)?;
        Ok(config)
    }

    /// 为上传的文件生成完整 URL。
    ///
    /// # 参数
    ///
    /// * `filename` - 上传文件的名称
    ///
    /// # 返回
    ///
    /// * `String` - 访问文件的完整 URL
    pub fn file_url(&self, filename: &str) -> String {
        format!("{}/{}", self.get_upload_path(), filename)
    }

    /// 获取配置的上传路径。
    ///
    /// # 返回
    ///
    /// * `String` - 配置的上传路径
    pub fn get_upload_path(&self) -> String {
        self.app.upload_path.clone()
    }

    /// 从 TOML 字符串解析配置。
    ///
    /// # 参数
    ///
    /// * `content` - TOML 格式的配置字符串
    ///
    /// # 返回
    ///
    /// * `Result<Self>` - 解析后的配置或错误
    pub fn from_str(content: &str) -> Result<Self> {
        let config = toml::from_str(content)?;
        Ok(config)
    }
}

/// 线程安全的配置包装器。
///
/// 使用读写锁提供对配置数据的安全并发访问。
#[derive(Clone)]
pub struct SafeConfig {
    /// 配置的线程安全引用
    pub inner: Arc<RwLock<Config>>,
}

impl SafeConfig {
    /// 从命令行参数创建新的 SafeConfig 实例。
    ///
    /// 本方法依赖 command 模块中定义的 Args 结构体来解析命令行参数。
    /// Args 结构体通过 clap 实现命令行参数的解析，定义在 `command.rs` 文件中。
    ///
    /// Args 结构体包含以下字段:
    /// * `config_path`: 配置文件路径
    /// * `enable_nacos`: 是否启用 Nacos 配置中心
    /// * `nacos_addr`: Nacos 服务器地址
    /// * `nacos_namespace`: Nacos 命名空间
    /// * `nacos_group`: Nacos 配置组
    /// * `nacos_data_id`: Nacos 配置ID
    ///
    /// # 示例
    ///
    /// ```bash
    /// # 使用本地配置文件
    /// ./program --config-path=config.toml
    ///
    /// # 使用 Nacos 配置中心
    /// ./program --enable-nacos \
    ///           --nacos-addr="http://localhost:8848" \
    ///           --nacos-namespace="public" \
    ///           --nacos-group="DEFAULT_GROUP" \
    ///           --nacos-data-id="config.toml"
    /// ```
    ///
    /// # 返回
    ///
    /// * `Result<Self>` - 初始化的 SafeConfig 实例或错误
    pub async fn from_args() -> Result<Self> {
        let args = Args::parse();

        if args.is_enable_nacos() {
            return Self::from_nacos_with_watcher(args.to_nacos_config()).await;
        }

        let config = Config::from_file(&args.config_path).await?;
        Ok(Self {
            inner: Arc::new(RwLock::new(config)),
        })
    }

    /// 获取当前配置的克隆。
    ///
    /// # 返回
    ///
    /// * `Result<Config>` - 当前配置的克隆或错误
    pub async fn get_config(&self) -> Result<Config> {
        let config = self.inner.read().await;
        Ok(config.clone())
    }

    /// 从 Nacos 初始化配置并设置配置观察器。
    ///
    /// # 参数
    ///
    /// * `args` - Nacos 配置参数
    ///
    /// # 返回
    ///
    /// * `Result<Self>` - 初始化的 SafeConfig 实例或错误
    async fn from_nacos_with_watcher(args: NacosConfig) -> Result<Self> {
        let nacos_client = NacosConfigClient::from_config(args).await?;

        let content = nacos_client.get_config().await?;
        let config = Config::from_str(&content)?;
        let safe_config = Self {
            inner: Arc::new(RwLock::new(config)),
        };

        let watcher = NacosConfigWatcher::new(safe_config.clone(), nacos_client);
        watcher.start_watch().await?;

        Ok(safe_config)
    }

    /// 从 Nacos 重新加载配置。
    ///
    /// # 参数
    ///
    /// * `nacos_client` - Nacos 客户端的引用
    ///
    /// # 返回
    ///
    /// * `Result<()>` - 成功或错误指示
    async fn reload_from_nacos(&self, nacos_client: &NacosConfigClient) -> Result<()> {
        let content = nacos_client.get_config().await?;
        let config = toml::from_str(&content)?;
        let mut writer = self.inner.write().await;
        *writer = config;

        log::info!("从 nacos 重新加载配置成功");
        Ok(())
    }
}
