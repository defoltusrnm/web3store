pub mod keycloak;
pub mod utils;

extern crate axum;
use keycloak::{
    authorization::DefaultAdminTokenProvider,
    credentials::EnvAdminCredentialProvider,
    host::EnvHostAddressProvider,
    management::{
        ClientsQuery, CreateClientRequest, CreateRealmRequest, CreateRoleRequest,
        CreateUserRequest, DefaultKeycloakManagement, KeycloakManagement, RoleQuery, UsersQuery,
    },
    routes::DefaultAdminRoutes,
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

    let auth_provider = &DefaultAdminTokenProvider::new(host_provider, credentials_provider);
    let keycloak_manager = &DefaultKeycloakManagement::new(auth_provider, host_provider);

    _ = keycloak_manager
        .create_realm::<DefaultAdminRoutes>(
            &CreateRealmRequest::new(&realm_name),
            &CancellationToken::new(),
        )
        .await?;

    log::info!("realm created");

    _ = keycloak_manager
        .create_client::<DefaultAdminRoutes>(
            &CreateClientRequest::new(&client_name, &realm_name, &client_secret),
            &CancellationToken::new(),
        )
        .await?;

    log::info!("client created");

    let clients = keycloak_manager
        .query_clients::<DefaultAdminRoutes>(
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
        .create_role::<DefaultAdminRoutes>(
            &CreateRoleRequest::new(&realm_name, &client.id, &role_name, "some_role"),
            &CancellationToken::new(),
        )
        .await?;

    log::info!("role created");

    let role = keycloak_manager
        .query_role::<DefaultAdminRoutes>(
            &RoleQuery::new(&realm_name, &client.id, &role_name),
            &CancellationToken::new(),
        )
        .await?;

    log::info!("got role {0}, {1}", role.id, role.name);

    _ = keycloak_manager
        .create_user::<DefaultAdminRoutes>(
            &CreateUserRequest::new(&realm_name, "test1", "test"),
            &CancellationToken::new(),
        )
        .await?;

    log::info!("user created");

    let users = keycloak_manager
        .query_users::<DefaultAdminRoutes>(
            &UsersQuery::new(&realm_name, "test1"),
            &CancellationToken::new(),
        )
        .await?;

    let user = users
        .get(0)
        .map(Result::Ok)
        .unwrap_or(Result::Err(AppErr::from("cannot get user from payload")))?;

    log::info!("got user: {0}, {1}", user.id, user.username);

    Ok(())
}
