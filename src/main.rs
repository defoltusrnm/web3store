pub mod keycloak;
pub mod utils;

extern crate axum;
use keycloak::{
    authorization::{AdminAccessTokenProvider, DefaultAdminTokenProvider},
    credentials::EnvAdminCredentialProvider,
    host::EnvHostAddressProvider,
};
use tokio_util::sync::CancellationToken;
use utils::{dotenv::configure_dotenv, errors::AppErr, logging::configure_logs};

#[tokio::main]
async fn main() -> Result<(), AppErr> {
    configure_dotenv();
    _ = configure_logs(log::LevelFilter::Trace)?;

    let token = DefaultAdminTokenProvider::get_access_token(
        EnvHostAddressProvider::new("KEYCLOAK_HOST"),
        EnvAdminCredentialProvider::new("KEYCLOAK_ADMIN_LOGIN", "KEYCLOAK_ADMIN_PASSWORD"),
        CancellationToken::new()
    )
    .await?;

    log::info!("{token}");

    Ok(())
}
