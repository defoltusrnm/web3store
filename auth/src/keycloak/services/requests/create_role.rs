use std::fmt::Display;

use serde::Serialize;

#[derive(Serialize)]
pub struct CreateRoleRequest {
    #[serde(skip)]
    pub realm: String,
    #[serde(skip)]
    pub client_uuid: String,
    pub name: String,
    pub description: String,
    pub composite: bool,
    #[serde(rename = "clientRole")]
    pub client_role: bool,
}

impl CreateRoleRequest {
    pub fn new(
        realm: &impl Display,
        client_uuid: &impl Display,
        name: &impl Display,
        description: &impl Display,
    ) -> Self {
        CreateRoleRequest {
            realm: realm.to_string(),
            client_uuid: client_uuid.to_string(),
            name: name.to_string(),
            description: description.to_string(),
            composite: false,
            client_role: true,
        }
    }
}
