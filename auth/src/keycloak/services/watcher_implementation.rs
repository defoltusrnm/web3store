use std::{sync::Arc, time::Duration};

use tokio::{select, time::sleep};
use tokio_util::sync::CancellationToken;
use utils::errors::AppErr;

use super::{authorization::AdminAccessTokenProvider, watcher::KeycloakWatcher};

pub struct DefaultKeycloakWatcher<TAuthorization: AdminAccessTokenProvider> {
    auth_provider: Arc<TAuthorization>,
}

impl<TAuthorization> DefaultKeycloakWatcher<TAuthorization>
where
    TAuthorization: AdminAccessTokenProvider,
{
    pub fn new(auth_provider: Arc<TAuthorization>) -> Self {
        DefaultKeycloakWatcher { auth_provider }
    }
}

impl<TAuthorization> KeycloakWatcher for DefaultKeycloakWatcher<TAuthorization>
where
    TAuthorization: AdminAccessTokenProvider + Send + Sync,
{
    async fn watch(&self, cancellation_token: &CancellationToken) -> Result<(), AppErr> {
        loop {
            log::info!("pinging keycloak service");
            sleep(Duration::from_secs(3)).await;

            select! {
                token = self.auth_provider.get_access_token() => {
                    if token.inspect_err(|err| log::error!("ping err: {err}")).is_ok() {
                        log::info!("keycloak is online");
                        return Ok(());
                    }
                    else {
                        continue
                    }
                },
                _ = cancellation_token.cancelled() => Result::<(), AppErr>::Err(AppErr::from("keycloak is not online"))
            }?
        }
    }
}
