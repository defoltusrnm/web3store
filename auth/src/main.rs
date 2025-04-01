pub mod create_customer;
pub mod create_vendor;
pub mod kafka;
pub mod keycloak;
pub mod login;
pub mod refresh_token;

extern crate axum;
use std::time::Duration;

use axum::{Router, response::Result};
use create_customer::create_customer_router;
use create_vendor::create_vendor_router;
use futures::TryFutureExt;
use keycloak::services::{
    authorization_implementation::DefaultAdminTokenProvider,
    credentials_implementation::EnvAdminCredentialProvider,
    host_implementation::EnvHostAddressProvider,
    management_implementation::DefaultKeycloakManagement,
    routes_implementation::DefaultAdminRoutes,
    seeding::{KeycloakSeeding, KeycloakSeedingArguments},
    seeding_implementation::DefaultKeycloakSeeding,
    watcher::KeycloakWatcher,
    watcher_implementation::DefaultKeycloakWatcher,
};
use login::create_login_router;
use refresh_token::create_refresh_token_router;
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
    let keycloak_manager = &DefaultKeycloakManagement::new(auth_provider, routes);

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

    let app = Router::new()
        .merge(create_customer_router())
        .merge(create_vendor_router())
        .merge(create_login_router())
        .merge(create_refresh_token_router());

    let listener = tokio::net::TcpListener::bind(env_var("SERVICE_HOST")?)
        .map_err(|err| AppErr::from_owned(format!("failed to bind: {err}")))
        .await?;

    log::info!("app started at: {0}", env_var("SERVICE_HOST")?);

    axum::serve(listener, app)
        .await
        .map_err(|err| AppErr::from_owned(format!("server failed {err}")))?;

    Ok(())
}
