use std::fmt::Display;

use serde::Serialize;

#[derive(Serialize)]
pub struct CreateRealmRequest {
    pub realm: String,
    pub enabled: bool,
}

impl CreateRealmRequest {
    pub fn new(realm: &impl Display) -> Self {
        CreateRealmRequest {
            realm: realm.to_string(),
            enabled: true,
        }
    }
}
