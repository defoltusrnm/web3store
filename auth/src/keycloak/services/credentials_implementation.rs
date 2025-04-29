use std::env;

use utils::errors::AppErr;

use super::credentials::AdminCredentialProvider;

pub struct EnvAdminCredentialProvider {
    login_env: String,
    password_env: String,
}

impl EnvAdminCredentialProvider {
    pub fn new(login_env: &str, password_env: &str) -> Self {
        EnvAdminCredentialProvider {
            login_env: login_env.to_string(),
            password_env: password_env.to_string(),
        }
    }
}

impl AdminCredentialProvider for EnvAdminCredentialProvider {
    async fn get_login(&self) -> Result<String, AppErr> {
        env::var(&self.login_env)
            .map_err(|err| AppErr::from_owned(format!("cannot get login env: {err}")))
    }

    async fn get_password(&self) -> Result<String, AppErr> {
        env::var(&self.password_env)
            .map_err(|err| AppErr::from_owned(format!("cannot get password env: {err}")))
    }
}
