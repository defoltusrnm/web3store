use std::fmt::Display;

pub struct ClientsQuery {
    pub realm: String,
    pub client_id: String,
}

impl ClientsQuery {
    pub fn new(realm: &impl Display, client_id: &impl Display) -> Self {
        ClientsQuery {
            realm: realm.to_string(),
            client_id: client_id.to_string(),
        }
    }
}
