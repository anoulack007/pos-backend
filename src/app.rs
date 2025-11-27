use axum::{Router, routing::get};
use sea_orm::DatabaseConnection;

use crate::config::Config;
use crate::auth;
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
        .with_state(state)
}

async fn health_handler() -> &'static str{
    "Ok"
}