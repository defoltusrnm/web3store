use std::fmt::Display;

use serde::Serialize;

#[derive(Serialize)]
pub struct CreateUserRequest {
    #[serde(skip)]
    pub realm: String,
    pub username: String,
    #[serde(rename = "firstName")]
    pub first_name: String,
    #[serde(rename = "lastName")]
    pub last_name: String,
    pub enabled: bool,
    pub credentials: [CreateUserCredentialsRequest; 1],
}

impl CreateUserRequest {
    pub fn new(realm: &impl Display, username: &impl Display, password: &impl Display) -> Self {
        CreateUserRequest {
            realm: realm.to_string(),
            username: username.to_string(),
            first_name: username.to_string(),
            last_name: username.to_string(),
            enabled: true,
            credentials: [CreateUserCredentialsRequest::new(password)],
        }
    }
}

#[derive(Serialize)]
pub struct CreateUserCredentialsRequest {
    #[serde(rename = "type")]
    pub user_type: String,
    pub value: String,
    pub temporary: bool,
}

impl CreateUserCredentialsRequest {
    pub fn new(password: impl Display) -> Self {
        CreateUserCredentialsRequest {
            user_type: "password".to_owned(),
            value: password.to_string(),
            temporary: false,
        }
    }
}
