use std::env;

use utils::errors::AppErr;

use super::credentials::AdminCredentialProvider;

pub struct EnvAdminCredentialProvider<'a> {
    login_env: &'a str,
    password_env: &'a str,
}

impl<'a> EnvAdminCredentialProvider<'a> {
    pub fn new(login_env: &'a str, password_env: &'a str) -> Self {
        EnvAdminCredentialProvider {
            login_env: login_env,
            password_env: password_env,
        }
    }
}

impl<'a> AdminCredentialProvider for EnvAdminCredentialProvider<'a> {
    async fn get_login(&self) -> Result<String, AppErr> {
        env::var(self.login_env)
            .map_err(|err| AppErr::from_owned(format!("cannot get login env: {err}")))
    }

    async fn get_password(&self) -> Result<String, AppErr> {
        env::var(self.password_env)
            .map_err(|err| AppErr::from_owned(format!("cannot get password env: {err}")))
    }
}
