pub mod keycloak;
pub mod utils;

extern crate axum;
use keycloak::{
    authorization::DefaultAdminTokenProvider,
    credentials::EnvAdminCredentialProvider,
    host::EnvHostAddressProvider,
    management::{
        CreateClientRequest, CreateRealmRequest, DefaultKeycloakManagement, KeycloakManagement,
    },
    routes::DefaultAdminRoutes,
};
use tokio_util::sync::CancellationToken;
use utils::{dotenv::configure_dotenv, errors::AppErr, logging::configure_logs};

#[tokio::main]
async fn main() -> Result<(), AppErr> {
    configure_dotenv();
    _ = configure_logs(log::LevelFilter::Trace)?;

    let realm_name = std::env::var("KEYCLOAK_REALM")
        .map_err(|err| AppErr::from_owned(format!("cannot get realm name: {err}")))?;

    let host_provider = &EnvHostAddressProvider::new("KEYCLOAK_HOST");
    let credentials_provider =
        &EnvAdminCredentialProvider::new("KEYCLOAK_ADMIN_LOGIN", "KEYCLOAK_ADMIN_PASSWORD");

    let auth_provider = &DefaultAdminTokenProvider::new(host_provider, credentials_provider);
    let keycloak_manager = DefaultKeycloakManagement::new(auth_provider, host_provider);

    _ = keycloak_manager
        .create_realm::<DefaultAdminRoutes>(
            &CreateRealmRequest::new(&realm_name),
            &CancellationToken::new(),
        )
        .await?;

    log::info!("realm created");

    let client_name = std::env::var("KEYCLOAK_CLIENT")
        .map_err(|err| AppErr::from_owned(format!("cannot get realm name: {err}")))?;

    let client_secret = std::env::var("KEYCLOAK_CLIENT_SECRET")
        .map_err(|err| AppErr::from_owned(format!("cannot get realm name: {err}")))?;

    _ = keycloak_manager
        .create_client::<DefaultAdminRoutes>(
            &CreateClientRequest::new(&client_name, &realm_name, &client_secret),
            &CancellationToken::new(),
        )
        .await?;

    log::info!("client created");

    Ok(())
}
