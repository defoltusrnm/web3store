use std::fmt::Display;

pub struct UsersQuery {
    pub realm: String,
    pub username: String,
}

impl UsersQuery {
    pub fn new(realm: &impl Display, username: &impl Display) -> Self {
        UsersQuery {
            realm: realm.to_string(),
            username: username.to_string(),
        }
    }
}
