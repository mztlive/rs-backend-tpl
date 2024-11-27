use std::time::Duration;

use axum::{
    middleware,
    routing::{delete, get, post, put},
    Router,
};
use tower::ServiceBuilder;
use tower_http::{
    cors::{Any, CorsLayer},
    timeout::TimeoutLayer,
};

use crate::app_state::AppState;

use super::handlers;
use super::handlers::upload;
use super::middlewares;

/// Creates the main application router with all the routes configured.
///
/// This function sets up all the routes for the application, including authentication,
/// organization management, project management, and more. It also configures middleware
/// for timeout and CORS settings.
///
/// # Arguments
///
/// * `app_state` - The application state containing shared resources across routes.
///
/// # Returns
///
/// Returns a `Router` with all the routes and middleware configured.
pub fn create(app_state: AppState) -> Router {
    // build our application with a single route
    let app = Router::new()
        .route("/login", post(handlers::login::login))
        .nest("/", secret_routes(app_state.clone()))
        .with_state(app_state)
        .layer(
            ServiceBuilder::new()
                .layer(TimeoutLayer::new(Duration::from_secs(30)))
                .layer(
                    CorsLayer::new()
                        .allow_origin(Any)
                        .allow_methods(Any)
                        .allow_headers(Any),
                ),
        );
    app
}

fn rbac_routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/admins", post(handlers::admin::create_admin))
        .route("/admins", get(handlers::admin::get_admin_list))
        .route("/admins/:id", put(handlers::admin::update_admin))
        .route("/admins/:id", delete(handlers::admin::delete_admin))
        .route("/admins/:id/role", put(handlers::admin::update_admin_role))
        .route("/roles", post(handlers::role::create_role))
        .route("/roles", get(handlers::role::get_role_list))
        .route("/roles/:id", put(handlers::role::update_role))
        .route("/roles/:id", delete(handlers::role::delete_role))
        .route_layer(middleware::from_fn_with_state(state.clone(), middlewares::rbac))
}

/// Defines secret routes that require authorization.
///
/// These routes are intended for authenticated users to access specific functionalities
/// such as fetching user information, managing tasks, and handling qualifications.
///
/// # Arguments
///
/// * `state` - The application state containing shared resources and configurations.
///
/// # Returns
///
/// Returns a `Router` configured with secret routes.
fn secret_routes(state: AppState) -> Router<AppState> {
    Router::new()
        .nest("/", rbac_routes(state.clone()))
        .route("/upload", post(upload::upload_file))
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            middlewares::operation_log,
        ))
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            middlewares::authorization,
        ))
}
