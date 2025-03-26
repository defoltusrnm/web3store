use serde::Deserialize;

#[derive(Deserialize)]
pub struct UserResponse {
    pub id: String,
    pub username: String,
    pub enabled: bool,
}
