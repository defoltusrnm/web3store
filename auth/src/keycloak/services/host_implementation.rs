use std::env;

use crate::utils::errors::AppErr;

use super::host::HostAddressProvider;

pub struct EnvHostAddressProvider<'a> {
    host_env: &'a str,
}

impl<'a> EnvHostAddressProvider<'a> {
    pub fn new(host_env: &'a str) -> Self {
        EnvHostAddressProvider { host_env }
    }
}

impl<'a> HostAddressProvider for EnvHostAddressProvider<'a> {
    async fn get_host(&self) -> Result<String, AppErr> {
        env::var(self.host_env)
            .map_err(|err| AppErr::from_owned(format!("cannot get login env: {err}")))
    }
}
