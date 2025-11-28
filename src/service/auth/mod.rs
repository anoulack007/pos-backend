pub mod dto;
pub mod jwt;
pub mod handlers;

use axum::{Router, routing::post};
use crate::app::AppState;
use self::handlers::{register_handler, login_handler, refresh_handler};

pub fn routes() -> Router<AppState>{
    Router::new()
        .route("/register", post(register_handler))
        .route("/login", post(login_handler))
        .route("/refresh", post(refresh_handler))
}