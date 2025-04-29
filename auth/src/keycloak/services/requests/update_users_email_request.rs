use std::fmt::Display;

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
    pub fn new_verified(
        realm: &impl Display,
        user_uuid: &impl Display,
        email: &impl Display,
    ) -> Self {
        UpdateUsersEmailRequest {
            realm: realm.to_string(),
            user_uuid: user_uuid.to_string(),
            email: email.to_string(),
            email_verified: true,
        }
    }
}
