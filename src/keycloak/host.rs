use std::env;

use crate::utils::errors::AppErr;

pub trait HostAddressProvider {
    fn get_host(self) -> impl Future<Output = Result<String, AppErr>>;
}

pub struct EnvHostAddressProvider {
    host_env: String,
}

impl EnvHostAddressProvider {
    pub fn new(host_env: &str) -> Self {
        EnvHostAddressProvider { host_env: host_env.to_owned() }
    }
}

impl HostAddressProvider for EnvHostAddressProvider {
    async fn get_host(self) -> Result<String, AppErr> {
        env::var(self.host_env)
            .map_err(|err| AppErr::from_owned(format!("cannot get login env: {err}")))
    }
}
