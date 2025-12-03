use axum::{Json, extract::State};
use axum::http::StatusCode;
use sea_orm::EntityTrait;

use crate::route::AppState;
use crate::entities::users::{self, Entity as Users};
use crate::middleware::auth_guard::AuthUser;
use super::dto::MeResponse;

pub async fn me_handler(
    State(state): State<AppState>,
    auth: AuthUser,
) -> Result<Json<MeResponse>, (StatusCode, String)> {
    let user = Users::find_by_id(auth.id)
        .one(&state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::UNAUTHORIZED, "User not found".to_string()))?;

    Ok(Json(MeResponse {
        id: user.id.to_string(),
        username: user.username,
        full_name: user.full_name,
        role: user.role.to_string(),
    }))
}
