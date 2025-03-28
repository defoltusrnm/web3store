use std::time::Duration;

use tokio::{select, time::sleep};
use tokio_util::sync::CancellationToken;
use utils::errors::AppErr;

use super::{authorization::AdminAccessTokenProvider, watcher::KeycloakWatcher};

pub struct DefaultKeycloakWatcher<'a, TAuthorization: AdminAccessTokenProvider> {
    auth_provider: &'a TAuthorization,
}

impl<'a, TAuthorization> DefaultKeycloakWatcher<'a, TAuthorization>
where
    TAuthorization: AdminAccessTokenProvider,
{
    pub fn new(auth_provider: &'a TAuthorization) -> Self {
        DefaultKeycloakWatcher { auth_provider }
    }
}

impl<'a, TAuthorization> KeycloakWatcher for DefaultKeycloakWatcher<'a, TAuthorization>
where
    TAuthorization: AdminAccessTokenProvider,
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
