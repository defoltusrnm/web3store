use tokio_util::sync::CancellationToken;

use crate::utils::errors::AppErr;

pub trait KeycloakWatcher {
    fn watch(
        &self,
        cancellation_token: &CancellationToken,
    ) -> impl Future<Output = Result<(), AppErr>>;
}