use std::fmt::Display;

pub struct RoleQuery {
    pub realm: String,
    pub client_uuid: String,
    pub role_name: String,
}

impl RoleQuery {
    pub fn new(realm: &impl Display, client_uuid: &impl Display, role_name: &impl Display) -> Self {
        RoleQuery {
            realm: realm.to_string(),
            client_uuid: client_uuid.to_string(),
            role_name: role_name.to_string(),
        }
    }
}
