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
    pub fn new(realm: &str, client_uuid: &str, name: &str, description: &str) -> Self {
        CreateRoleRequest {
            realm: realm.to_owned(),
            client_uuid: client_uuid.to_owned(),
            name: name.to_owned(),
            description: description.to_owned(),
            composite: false,
            client_role: true,
        }
    }
}
