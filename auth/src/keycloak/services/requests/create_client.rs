use std::fmt::Display;

use serde::Serialize;

#[derive(Serialize)]
pub struct CreateClientRequest {
    #[serde(skip)]
    pub realm: String,
    #[serde(rename = "clientId")]
    pub client_id: String,
    pub enabled: bool,
    #[serde(rename = "publicClient")]
    pub public_client: bool,
    pub secret: String,
    #[serde(rename = "directAccessGrantsEnabled")]
    pub direct_access_grants_enabled: bool,
}

impl CreateClientRequest {
    pub fn new(client: &impl Display, realm: &impl Display, secret: &impl Display) -> Self {
        CreateClientRequest {
            realm: realm.to_string(),
            client_id: client.to_string(),
            enabled: true,
            public_client: true,
            secret: secret.to_string(),
            direct_access_grants_enabled: true,
        }
    }
}
