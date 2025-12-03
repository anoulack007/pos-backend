use serde::Serialize;

#[derive(Serialize)]
pub struct MeResponse{
    pub id: String,
    pub username: String,
    pub full_name: Option<String>,
    pub role: String,
}