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
    pub fn new(client: &str, realm: &str, secret: &str) -> Self {
        CreateClientRequest {
            realm: realm.to_owned(),
            client_id: client.to_owned(),
            enabled: true,
            public_client: true,
            secret: secret.to_owned(),
            direct_access_grants_enabled: true,
        }
    }
}
