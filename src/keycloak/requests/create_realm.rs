use serde::Serialize;

#[derive(Serialize)]
pub struct CreateRealmRequest {
    pub realm: String,
    pub enabled: bool,
}

impl CreateRealmRequest {
    pub fn new(realm: &str) -> Self {
        CreateRealmRequest {
            realm: realm.to_owned(),
            enabled: true,
        }
    }
}
