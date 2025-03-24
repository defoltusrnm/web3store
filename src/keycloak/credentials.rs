use std::env;

use crate::utils::errors::AppErr;

pub trait AdminCredentialProvider {
    fn get_login(&self) -> impl Future<Output = Result<String, AppErr>>;
    fn get_password(&self) -> impl Future<Output = Result<String, AppErr>>;
}

pub struct EnvAdminCredentialProvider {
    login_env: String,
    password_env: String,
}

impl EnvAdminCredentialProvider {
    pub fn new(login_env: &str, password_env: &str) -> Self {
        EnvAdminCredentialProvider {
            login_env: login_env.to_owned(),
            password_env: password_env.to_owned(),
        }
    }
}

impl AdminCredentialProvider for EnvAdminCredentialProvider {
    async fn get_login(&self) -> Result<String, AppErr> {
        env::var(self.login_env.clone())
            .map_err(|err| AppErr::from_owned(format!("cannot get login env: {err}")))
    }

    async fn get_password(&self) -> Result<String, AppErr> {
        env::var(self.password_env.clone())
            .map_err(|err| AppErr::from_owned(format!("cannot get password env: {err}")))
    }
}
