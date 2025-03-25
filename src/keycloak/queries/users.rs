pub struct UsersQuery {
    pub realm: String,
    pub username: String,
}

impl UsersQuery {
    pub fn new(realm: &str, username: &str) -> Self {
        UsersQuery {
            realm: realm.to_owned(),
            username: username.to_owned(),
        }
    }
}
