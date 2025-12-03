use axum::{Router,middleware, routing::get};
use sea_orm::DatabaseConnection;

use crate::config::{Config, logging::log_request_response};
use crate::service::{auth, users};

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
    pub config: Config,
}

pub fn create_app(db: DatabaseConnection, config: Config) -> Router {
    let state = AppState { db, config };

    Router::new()
        .route("/health", get(health_handler))
        .nest("/auth", auth::routes())
        .nest("/users", users::routes())
        .layer(middleware::from_fn(log_request_response))
        .with_state(state)
}

async fn health_handler() -> &'static str {
    "Ok"
}
