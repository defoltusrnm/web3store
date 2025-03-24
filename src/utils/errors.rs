use derive_more::Display;


#[derive(Display, Debug)]
pub struct AppErr {
    msg: String
}

impl AppErr {
    pub fn from_owned(msg: String) -> Self {
        AppErr { msg }
    }
    pub fn from(msg: &str) -> Self {
        AppErr { msg: msg.to_owned() }
    }
}