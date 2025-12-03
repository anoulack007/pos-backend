pub mod dto;
pub mod handlers;

use axum::{Router, routing::get};
use crate::route::AppState;
use self::handlers::me_handler;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/me", get(me_handler))
}
