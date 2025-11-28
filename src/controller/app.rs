use axum::{Router, routing::get, middleware};
use sea_orm::DatabaseConnection;

use crate::config::Config;
use crate::service::auth;
use crate::middleware::log_request_response;
#[derive(Clone)]
pub struct AppState{
    pub db: DatabaseConnection,
    pub config: Config,
}

pub fn create_app(db: DatabaseConnection, config:Config) -> Router{
    let state = AppState{ db ,config};

    Router::new()
        .route("/health", get(health_handler))
        .nest("/auth", auth::routes())
        .layer(middleware::from_fn(log_request_response))
        .with_state(state)
}

async fn health_handler() -> &'static str{
    "Ok"
}