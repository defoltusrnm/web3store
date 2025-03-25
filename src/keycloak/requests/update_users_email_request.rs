use serde::Serialize;

#[derive(Serialize)]
pub struct UpdateUsersEmailRequest {
    #[serde(skip)]
    pub realm: String,
    #[serde(skip)]
    pub user_uuid: String,
    pub email: String,
    #[serde(rename = "emailVerified")]
    pub email_verified: bool,
}

impl UpdateUsersEmailRequest {
    pub fn new_verified(realm: &str, user_uuid: &str, email: &str) -> Self {
        UpdateUsersEmailRequest {
            realm: realm.to_owned(),
            user_uuid: user_uuid.to_owned(),
            email: email.to_owned(),
            email_verified: true,
        }
    }
}
