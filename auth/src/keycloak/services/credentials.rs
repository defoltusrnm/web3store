use utils::errors::AppErr;

pub trait AdminCredentialProvider {
    fn get_login(&self) -> impl Future<Output = Result<String, AppErr>> + Send;
    fn get_password(&self) -> impl Future<Output = Result<String, AppErr>> + Send;
}
