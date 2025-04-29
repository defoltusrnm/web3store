use futures::TryFutureExt;
use utils::errors::{AppErr, HttpAppErr};

pub trait KeycloakExtensions<T, Ok>
where
    T: Future<Output = Result<Ok, AppErr>>,
{
    fn await_err_as_failed_dependency(self) -> impl Future<Output = Result<Ok, HttpAppErr>> + Send;
    fn log_err(self) -> impl Future<Output = Result<Ok, AppErr>> + Send;
}

impl<T, Ok> KeycloakExtensions<T, Ok> for T
where
    T: Future<Output = Result<Ok, AppErr>> + Send,
{
    async fn await_err_as_failed_dependency(self) -> Result<Ok, HttpAppErr> {
        self.inspect_err(|err| log::warn!("request failed with: {err}"))
            .map_err(HttpAppErr::failed_dependency)
            .await
    }

    async fn log_err(self) -> Result<Ok, AppErr> {
        self.inspect_err(|err| log::error!("error occurred: {err}"))
            .await
    }
}
