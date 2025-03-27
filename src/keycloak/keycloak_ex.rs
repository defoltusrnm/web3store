use crate::utils::async_ex::AsyncResult;
use crate::utils::errors::{AppErr, HttpAppErr};

pub trait KeycloakExtensions<T, Ok>
where
    T: Future<Output = Result<Ok, AppErr>>,
{
    fn await_err_as_failed_dependency(self) -> impl Future<Output = Result<Ok, HttpAppErr>>;
    fn await_log_err(self) -> impl Future<Output = Result<Ok, AppErr>>;
}

impl<T, Ok> KeycloakExtensions<T, Ok> for T
where
    T: Future<Output = Result<Ok, AppErr>>,
{
    async fn await_err_as_failed_dependency(self) -> Result<Ok, HttpAppErr> {
        self.await_inspect_err(|err| log::warn!("request failed with: {err}"))
            .await_map_err(HttpAppErr::failed_dependency)
            .await
    }

    async fn await_log_err(self) -> Result<Ok, AppErr> {
        self.await_inspect_err(|err| log::error!("error occurred: {err}"))
            .await
    }
}
