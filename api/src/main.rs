mod app_state;
mod core;
mod jwt;

use app_state::{AppState, DatabaseState};
use config::Config;
use container::ServiceFactory;
use core::routes;
use database::repositories::{AdminRepository, RoleRepository};
use log::info;
use rbac::ActorHandler;

#[tokio::main]
async fn main() {
    libs::logger::init();

    let config = Config::from_args().await.expect("Failed to load config");

    info!("Starting application with config: {}", config.app.port);
    start(config).await
}

async fn start(cfg: Config) {
    let (client, db) = database::mongodb::connect(&cfg.database.uri, &cfg.database.db_name)
        .await
        .expect("Failed to connect to database");

    let app_port = cfg.app.port;

    let state = AppState::new(
        DatabaseState::new(client, db.clone()),
        cfg,
        ActorHandler::new(RoleRepository::new(db.clone()), AdminRepository::new(db.clone())).await,
    );

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
