use axum::{Json, extract::State};
use sea_orm::{EntityTrait, ColumnTrait, QueryFilter, ActiveModelTrait,Set};
use jsonwebtoken::{decode, DecodingKey, Validation};
use sea_orm::prelude::Uuid;

use crate::app::AppState;
use crate::entities::users::{self , Entity as Users, ActiveModel as UserActiveModel, UserRole};
use super::dto::{RegisterRequest, RegisterResponse, LoginRequest, LoginResponse, RefreshRequest};
use super::jwt::{generate_token_pair, Claims};

fn internal_error<E: std::fmt::Display>(
    err: E,
) -> (axum::http::StatusCode, String){
    (
        axum::http::StatusCode::INTERNAL_SERVER_ERROR,
        err.to_string(),
    )
}

fn hash_password(plain: &str) -> anyhow::Result<String>{
    Ok(plain.to_string())
}

fn verify_password(plain: &str, hash: &str) -> anyhow::Result<bool>{
    Ok(plain == hash)
}

pub async fn register_handler(
    State(state): State<AppState>,
    Json(body): Json<RegisterRequest>,
) -> Result<Json<RegisterResponse>, (axum::http::StatusCode, String)>{
    let existing = Users::find()
        .filter(users::Column::Username.eq(&body.username))
        .one(&state.db)
        .await
        .map_err(internal_error)?;

    if existing.is_some(){
        return Err((axum::http::StatusCode::BAD_REQUEST,"Username already exists".into()))
    }

    let password_hash = hash_password(&body.password).map_err(internal_error)?;

    let mut new_user = UserActiveModel{
        username: Set(body.username.clone()),
        password_hash: Set(password_hash),
        full_name: Set(body.full_name.clone()),
        role:Set(UserRole::Cashier),
        is_active: Set(true),
        ..Default::default()
    };

    let res = new_user.insert(&state.db).await.map_err(internal_error)?;

    Ok(Json(RegisterResponse{
        id: res.id.to_string(),
        username: res.username,
    }))
}

pub async fn login_handler(
    State(state): State<AppState>,
    Json(body): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, (axum::http::StatusCode, String)>{
    let user = Users::find()
        .filter(users::Column::Username.eq(&body.username))
        .one(&state.db)
        .await
        .map_err(internal_error)?
        .ok_or((axum::http::StatusCode::UNAUTHORIZED,"Invalid credentials".into()))?;

    if !user.is_active{
        return Err((axum::http::StatusCode::UNAUTHORIZED,"User disabled".into()));
    }

    if !verify_password(&body.password, &user.password_hash)
        .map_err(internal_error)?
        {
            return Err((axum::http::StatusCode::UNAUTHORIZED,"Invalid credentials".into()));
        }
    
    let pair = generate_token_pair(
        &user.id.to_string(),
        &user.role.to_string(),
        &state.config,
    )
    .map_err(internal_error)?;

    Ok(Json(LoginResponse{
        access_token: pair.access_token,
        refresh_token: pair.refresh_token,
    }))
}

pub async fn refresh_handler(
    State(state): State<AppState>,
    Json(body): Json<RefreshRequest>,
) -> Result<Json<LoginResponse>, (axum::http::StatusCode, String)>{
    let token_data = decode::<Claims>(
        &body.refresh_token,
        &DecodingKey::from_secret(state.config.refresh_token_secret.as_bytes()),
        &Validation::default(),
    )
    .map_err(|_| (axum::http::StatusCode::UNAUTHORIZED,"Invalid token".into()))?;

    let claims = token_data.claims;

    if claims.typ != "refresh"{
        return Err((axum::http::StatusCode::UNAUTHORIZED,"Invalid token type".into()));
    }


    let user_id = Uuid::parse_str(&claims.sub)
        .map_err(|_| (axum::http::StatusCode::UNAUTHORIZED,"Invalid user id in token".into()))?;

    let user = Users::find_by_id(user_id)
        .one(&state.db)
        .await
        .map_err(internal_error)?
        .ok_or((axum::http::StatusCode::UNAUTHORIZED, "User not found".into()))?;

    if !user.is_active{
        return Err((axum::http::StatusCode::UNAUTHORIZED,"User disabled".into()));
    }

    let pair = generate_token_pair(
        &user.id.to_string(),
        &user.role.to_string(),
        &state.config,
    )
    .map_err(internal_error)?;

    Ok(Json(LoginResponse{
        access_token: pair.access_token,
        refresh_token: pair.refresh_token,
    }))
}