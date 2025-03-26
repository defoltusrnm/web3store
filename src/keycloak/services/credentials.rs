use crate::utils::errors::AppErr;

pub trait AdminCredentialProvider {
    fn get_login(&self) -> impl Future<Output = Result<String, AppErr>>;
    fn get_password(&self) -> impl Future<Output = Result<String, AppErr>>;
}
