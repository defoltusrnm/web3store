pub mod keycloak;
pub mod utils;

extern crate axum;
use std::time::Duration;

use axum::{Json, Router, routing::post};
use http::StatusCode;
use keycloak::{
    keycloak_factory::{create_default_manager, create_default_seeder, create_default_watcher},
    services::{
        management::KeycloakManagement,
        requests::create_user::CreateUserRequest,
        seeding::{KeycloakSeeding, KeycloakSeedingArguments},
        watcher::KeycloakWatcher,
    },
};
use serde::Deserialize;
use tokio_util::sync::CancellationToken;
use utils::{dotenv::configure_dotenv, env::env_var, errors::AppErr, logging::configure_logs};

#[tokio::main]
async fn main() -> Result<(), AppErr> {
    configure_dotenv();
    _ = configure_logs(log::LevelFilter::Info)?;

    let keycloak_watcher = create_default_watcher();
    let watcher_cancellation = &CancellationToken::new();
    let watcher_cancellation_clone = watcher_cancellation.clone();

    tokio::task::spawn(async move {
        tokio::time::sleep(Duration::from_secs(60)).await;
        watcher_cancellation_clone.clone().cancel();
    });
    keycloak_watcher
        .watch(&watcher_cancellation.clone())
        .await?;

    let keycloak_seeder = create_default_seeder();

    keycloak_seeder
        .seed(KeycloakSeedingArguments::new(
            &env_var("KEYCLOAK_REALM")?,
            &env_var("KEYCLOAK_CLIENT")?,
            &env_var("KEYCLOAK_CLIENT_SECRET")?,
            &env_var("KEYCLOAK_CUSTOMER_ROLE")?,
            &env_var("KEYCLOAK_VENDOR_ROLE")?,
        ))
        .await?;

    let app = Router::new().merge(create_customer_router());

    let listener = tokio::net::TcpListener::bind(env_var("SERVICE_HOST")?)
        .await
        .map_err(|err| AppErr::from_owned(format!("failed to bind: {err}")))?;

    axum::serve(listener, app)
        .await
        .map_err(|err| AppErr::from_owned(format!("server failed {err}")))?;

    Ok(())
}

fn create_customer_router() -> _ {
    Router::new().route("api/customers", post(create_customer))
}

async fn create_customer(Json(request): Json<CreateCustomerRequest>) -> StatusCode {
    let manager = create_default_manager();

    manager
        .create_user(
            &CreateUserRequest::new(&env_var("REALM_NAME")?, &request.email, &request.password),
            &CancellationToken::new(),
        )
        .await?;

    StatusCode::CREATED
}

#[derive(Deserialize)]
struct CreateCustomerRequest {
    pub email: String,
    pub password: String,
}
