use utils::errors::AppErr;

pub trait HostAddressProvider {
    fn get_host(&self) -> impl Future<Output = Result<String, AppErr>>;
}
