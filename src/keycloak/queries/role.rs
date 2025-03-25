pub struct RoleQuery {
    pub realm: String,
    pub client_uuid: String,
    pub role_name: String,
}

impl RoleQuery {
    pub fn new(realm: &str, client_uuid: &str, role_name: &str) -> Self {
        RoleQuery {
            realm: realm.to_owned(),
            client_uuid: client_uuid.to_owned(),
            role_name: role_name.to_owned(),
        }
    }
}
