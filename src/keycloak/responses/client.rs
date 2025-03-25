use serde::Deserialize;

#[derive(Deserialize)]
pub struct ClientResponse {
    pub id: String,
    #[serde(rename = "clientId")]
    pub client_id: String,
}
