pub struct ClientsQuery {
    pub realm: String,
    pub client_id: String,
}

impl ClientsQuery {
    pub fn new(realm: &str, client_id: &str) -> Self {
        ClientsQuery {
            realm: realm.to_owned(),
            client_id: client_id.to_owned(),
        }
    }
}
