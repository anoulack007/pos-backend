use axum::extract::FromRequestParts;
use axum::http::{request::Parts, StatusCode, header};
use jsonwebtoken::{decode, DecodingKey, Validation};
use sea_orm::EntityTrait;
use sea_orm::prelude::Uuid;

use crate::route::AppState;
use crate::service::auth::jwt::Claims;
use crate::entities::users::Entity as Users;

#[derive(Debug, Clone)]
pub struct AuthUser {
    pub id: Uuid,
    pub role: String,
}

impl AuthUser {
    pub fn require_admin(&self) -> Result<(), (StatusCode, String)> {
        if self.role == "ADMIN" {
            Ok(())
        } else {
            Err((StatusCode::FORBIDDEN, "Admin only".to_string()))
        }
    }
}

impl FromRequestParts<AppState> for AuthUser {
    type Rejection = (StatusCode, String);

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        // 1) อ่าน Authorization header
        let auth_header = parts
            .headers
            .get(header::AUTHORIZATION)
            .and_then(|v| v.to_str().ok())
            .ok_or((
                StatusCode::UNAUTHORIZED,
                "Missing Authorization header".to_string(),
            ))?;

        // รูปแบบต้องเป็น "Bearer xxx"
        let token = auth_header
            .strip_prefix("Bearer ")
            .ok_or((
                StatusCode::UNAUTHORIZED,
                "Invalid Authorization header".to_string(),
            ))?;

        // 2) decode JWT ด้วย access_token_secret
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(state.config.access_token_secret.as_bytes()),
            &Validation::default(),
        )
        .map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid token".to_string()))?;

        let claims = token_data.claims;

        // ต้องเป็น access token เท่านั้น
        if claims.typ != "access" {
            return Err((StatusCode::UNAUTHORIZED, "Invalid token type".to_string()));
        }

        // 3) แปลง sub → Uuid
        let user_id = Uuid::parse_str(&claims.sub)
            .map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid user id".to_string()))?;

        // 4) โหลด user จาก DB
        let user = Users::find_by_id(user_id)
            .one(&state.db)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
            .ok_or((StatusCode::UNAUTHORIZED, "User not found".to_string()))?;

        if !user.is_active {
            return Err((StatusCode::UNAUTHORIZED, "User disabled".to_string()));
        }

        Ok(AuthUser {
            id: user.id,
            role: user.role.to_string(),
        })
    }
}
