use tokio_util::sync::CancellationToken;
use utils::errors::AppErr;

pub trait KeycloakWatcher {
    fn watch(
        &self,
        cancellation_token: &CancellationToken,
    ) -> impl Future<Output = Result<(), AppErr>> + Send;
}
