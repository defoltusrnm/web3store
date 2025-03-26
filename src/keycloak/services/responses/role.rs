use serde::Deserialize;

#[derive(Deserialize)]
pub struct RoleResponse {
    pub id: String,
    pub name: String,
}
