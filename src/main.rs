pub mod keycloak;
pub mod utils;

extern crate axum;
use std::time::Duration;

use keycloak::{
    authorization::DefaultAdminTokenProvider,
    credentials::EnvAdminCredentialProvider,
    host::EnvHostAddressProvider,
    management::{DefaultKeycloakManagement, KeycloakManagement},
    queries::{clients::ClientsQuery, role::RoleQuery, users::UsersQuery},
    requests::{
        assign_roles::{AssignRoleRequest, AssignRolesRequest},
        create_client::CreateClientRequest,
        create_realm::CreateRealmRequest,
        create_role::CreateRoleRequest,
        create_user::CreateUserRequest,
        update_users_email_request::UpdateUsersEmailRequest,
    },
    routes::DefaultAdminRoutes,
    watcher::{DefaultKeycloakWatcher, KeycloakWatcher},
};
use tokio_util::sync::CancellationToken;
use utils::{dotenv::configure_dotenv, env::env_var, errors::AppErr, logging::configure_logs};

#[tokio::main]
async fn main() -> Result<(), AppErr> {
    configure_dotenv();
    _ = configure_logs(log::LevelFilter::Info)?;

    let realm_name = env_var("KEYCLOAK_REALM")?;
    let client_name = env_var("KEYCLOAK_CLIENT")?;
    let client_secret = env_var("KEYCLOAK_CLIENT_SECRET")?;
    let role_name = env_var("KEYCLOAK_ROLE")?;

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

    _ = keycloak_manager
        .create_realm(
            &CreateRealmRequest::new(&realm_name),
            &CancellationToken::new(),
        )
        .await?;

    log::info!("realm created");

    _ = keycloak_manager
        .create_client(
            &CreateClientRequest::new(&client_name, &realm_name, &client_secret),
            &CancellationToken::new(),
        )
        .await?;

    log::info!("client created");

    let clients = keycloak_manager
        .query_clients(
            &ClientsQuery::new(&realm_name, &client_name),
            &CancellationToken::new(),
        )
        .await?;

    let client = clients
        .get(0)
        .map(Result::Ok)
        .unwrap_or(Result::Err(AppErr::from("cannot get client from payload")))?;

    log::info!("got client: {0}, {1}", client.id, client.client_id);

    _ = keycloak_manager
        .create_role(
            &CreateRoleRequest::new(&realm_name, &client.id, &role_name, "some_role"),
            &CancellationToken::new(),
        )
        .await?;

    log::info!("role created");

    let role = keycloak_manager
        .query_role(
            &RoleQuery::new(&realm_name, &client.id, &role_name),
            &CancellationToken::new(),
        )
        .await?;

    log::info!("got role {0}, {1}", role.id, role.name);

    _ = keycloak_manager
        .create_user(
            &CreateUserRequest::new(&realm_name, "test1", "test"),
            &CancellationToken::new(),
        )
        .await?;

    log::info!("user created");

    let users = keycloak_manager
        .query_users(
            &UsersQuery::new(&realm_name, "test1"),
            &CancellationToken::new(),
        )
        .await?;

    let user = users
        .get(0)
        .map(Result::Ok)
        .unwrap_or(Result::Err(AppErr::from("cannot get user from payload")))?;

    log::info!("got user: {0}, {1}", user.id, user.username);

    _ = keycloak_manager
        .update_users_email(
            &UpdateUsersEmailRequest::new_verified(&realm_name, &user.id, "test@test.test"),
            &CancellationToken::new(),
        )
        .await?;

    log::info!("user's email updated");

    _ = keycloak_manager
        .assign_roles(
            &AssignRolesRequest::new(
                &realm_name,
                &user.id,
                &client.id,
                &[AssignRoleRequest::new(&role.id, &role.name)],
            ),
            &CancellationToken::new(),
        )
        .await?;

    log::info!("role assigned to user");

    Ok(())
}
