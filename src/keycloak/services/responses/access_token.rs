use derive_more::Display;
use serde::Deserialize;


#[derive(Deserialize, Display)]
pub struct AccessTokenResponse {
    pub access_token: String,
}
