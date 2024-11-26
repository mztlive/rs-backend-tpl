mod actors;
mod app_state;
mod config;
mod database;
mod entities;
mod api;
mod jwt;
mod libs;
mod rbac;
mod statics;

use actors::rbac::RbacActorHandler;
use app_state::{AppState, DatabaseState};
use clap::Parser;
use config::AppConfig;
use database::repositories::{role::RoleRepository, user::UserRepository};
use api::routes;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long, default_value = "./config.toml")]
    config_path: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let app_cfg = config::load_config(&args.config_path)
        .await
        .expect("Failed to load config");

    start(app_cfg).await
}

async fn start(cfg: AppConfig) {
    let (client, db) = database::mongodb::connect(&cfg.database.uri, &cfg.database.db_name)
        .await
        .expect("Failed to connect to database");

    let app_port = cfg.app.port;

    let state = AppState {
        db_state: DatabaseState::new(client, db.clone()),
        config: cfg,
        rbac: RbacActorHandler::new(db, RoleRepository::new(), UserRepository::new()).await,
    };

    run_app(app_port, state).await
}

/// 启动应用程序并监听指定端口
///
/// # 参数
/// * `app_port` - 应用程序监听的端口号
/// * `state` - 应用程序状态，包含数据库连接、配置等信息
///
/// # 示例
///
/// ```
/// let state = AppState::new();
/// run_app(3000, state).await;
/// ```
///
/// # 错误
/// 如果无法绑定到指定端口或无法启动服务，此函数将返回错误。
async fn run_app(app_port: u16, state: AppState) {
    let app = routes::create(state);

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", app_port))
        .await
        .expect("Failed to bind to port");

    axum::serve(listener, app).await.expect("Failed to serve");
}
