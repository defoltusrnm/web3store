use std::{env, fmt::Display};

use utils::errors::AppErr;

use super::host::HostAddressProvider;

pub struct EnvHostAddressProvider {
    host_env: String,
}

impl EnvHostAddressProvider {
    pub fn new(host_env: &impl Display) -> Self {
        EnvHostAddressProvider {
            host_env: host_env.to_string(),
        }
    }
}

impl HostAddressProvider for EnvHostAddressProvider {
    async fn get_host(&self) -> Result<String, AppErr> {
        env::var(&self.host_env)
            .map_err(|err| AppErr::from_owned(format!("cannot get login env: {err}")))
    }
}
