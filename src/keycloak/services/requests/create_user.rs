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
    pub fn new(realm: &str, username: &str, password: &str) -> Self {
        CreateUserRequest {
            realm: realm.to_owned(),
            username: username.to_owned(),
            first_name: username.to_owned(),
            last_name: username.to_owned(),
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
    pub fn new(password: &str) -> Self {
        CreateUserCredentialsRequest {
            user_type: "password".to_owned(),
            value: password.to_owned(),
            temporary: false,
        }
    }
}
