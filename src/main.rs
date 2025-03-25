pub mod keycloak;
pub mod utils;

extern crate axum;
use std::time::Duration;

use keycloak::{
    authorization::DefaultAdminTokenProvider,
    credentials::EnvAdminCredentialProvider,
    host::EnvHostAddressProvider,
    management::DefaultKeycloakManagement,
    routes::DefaultAdminRoutes,
    seeding::{DefaultKeycloakSeeding, KeycloakSeeding, KeycloakSeedingArguments},
    watcher::{DefaultKeycloakWatcher, KeycloakWatcher},
};
use tokio_util::sync::CancellationToken;
use utils::{dotenv::configure_dotenv, env::env_var, errors::AppErr, logging::configure_logs};

#[tokio::main]
async fn main() -> Result<(), AppErr> {
    configure_dotenv();
    _ = configure_logs(log::LevelFilter::Info)?;

    let host_provider = &EnvHostAddressProvider::new("KEYCLOAK_HOST");
    let credentials_provider =
        &EnvAdminCredentialProvider::new("KEYCLOAK_ADMIN_LOGIN", "KEYCLOAK_ADMIN_PASSWORD");

    let routes = &DefaultAdminRoutes::new(host_provider);

    let auth_provider = &DefaultAdminTokenProvider::new(routes, credentials_provider);

    let keycloak_watcher = &DefaultKeycloakWatcher::new(auth_provider);
    let watcher_cancellation = &CancellationToken::new();
    let watcher_cancellation_clone = watcher_cancellation.clone();

    tokio::task::spawn(async move {
        tokio::time::sleep(Duration::from_secs(60)).await;
        watcher_cancellation_clone.clone().cancel();
    });
    keycloak_watcher
        .watch(&watcher_cancellation.clone())
        .await?;

    let keycloak_manager = &DefaultKeycloakManagement::new(auth_provider, routes);
    let keycloak_seeder = &DefaultKeycloakSeeding::new(keycloak_manager);

    keycloak_seeder
        .seed(KeycloakSeedingArguments::new(
            &env_var("KEYCLOAK_REALM")?,
            &env_var("KEYCLOAK_CLIENT")?,
            &env_var("KEYCLOAK_CLIENT_SECRET")?,
            &env_var("KEYCLOAK_CUSTOMER_ROLE")?,
            &env_var("KEYCLOAK_VENDOR_ROLE")?,
        ))
        .await?;

    Ok(())
}
