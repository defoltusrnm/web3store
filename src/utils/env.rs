use super::errors::AppErr;

pub fn env_var(name: &str) -> Result<String, AppErr> {
    std::env::var(name).map_err(|err| AppErr::from_owned(format!("failed to read {name}: {err}")))
}
