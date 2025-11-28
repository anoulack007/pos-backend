use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct RegisterRequest{
    pub username: String,
    pub password: String,
    pub full_name: Option<String>,
}

#[derive(Serialize)]
pub struct RegisterResponse{
    pub id: String,
    pub username: String,
}


#[derive(Deserialize)]
pub struct LoginRequest{
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginResponse{
    pub access_token: String,
    pub refresh_token: String,
}

#[derive(Deserialize)]
pub struct RefreshRequest{
    pub refresh_token: String,
}
