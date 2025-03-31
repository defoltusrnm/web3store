use axum::{Json, Router, response::Result, routing::post};
use http::StatusCode;
use serde::{Deserialize, Serialize};
use utils::{
    env::env_var,
    errors::{AppErr, HttpAppErr},
};

use crate::{
    kafka::kafka_producer,
    keycloak::{
        keycloak_ex::KeycloakExtensions,
        services::{
            authorization_implementation::DefaultAdminTokenProvider,
            credentials_implementation::EnvAdminCredentialProvider,
            host_implementation::EnvHostAddressProvider,
            management::KeycloakManagement,
            management_implementation::DefaultKeycloakManagement,
            queries::{clients::ClientsQuery, role::RoleQuery, users::UsersQuery},
            requests::{
                assign_roles::{AssignRoleRequest, AssignRolesRequest},
                create_user::CreateUserRequest,
                update_users_email_request::UpdateUsersEmailRequest,
            },
            routes_implementation::DefaultAdminRoutes,
        },
    },
};

pub fn create_vendor_router() -> Router {
    Router::new().route("/api/vendors", post(vendor_customer))
}

async fn vendor_customer(Json(request): Json<VendorCustomerRequest>) -> Result<StatusCode> {
    let host_provider = &EnvHostAddressProvider::new("KEYCLOAK_HOST");
    let credentials_provider =
        &EnvAdminCredentialProvider::new("KEYCLOAK_ADMIN_LOGIN", "KEYCLOAK_ADMIN_PASSWORD");
    let routes = &DefaultAdminRoutes::new(host_provider);
    let auth_provider = &DefaultAdminTokenProvider::new(routes, credentials_provider);
    let manager = &DefaultKeycloakManagement::new(auth_provider, routes);

    let realm_name = env_var("KEYCLOAK_REALM")?;
    let client_name = env_var("KEYCLOAK_CLIENT")?;
    let role_name = env_var("KEYCLOAK_VENDOR_ROLE")?;

    let clients = manager
        .query_clients(&ClientsQuery::new(&realm_name, &client_name))
        .await_err_as_failed_dependency()
        .await?;

    let client = clients
        .get(0)
        .map(Result::Ok)
        .unwrap_or(Err(HttpAppErr::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "failed to create user",
        )))?;

    let role = manager
        .query_role(&RoleQuery::new(&realm_name, &client.id, &role_name))
        .await_err_as_failed_dependency()
        .await?;

    manager
        .create_user(&CreateUserRequest::new(
            &realm_name,
            &request.email,
            &request.password,
        ))
        .await_err_as_failed_dependency()
        .await?;

    let users = manager
        .query_users(&UsersQuery::new(&realm_name, &request.email))
        .await_err_as_failed_dependency()
        .await?;

    let user = users.get(0).map(Result::Ok).unwrap_or(Err(HttpAppErr::new(
        StatusCode::INTERNAL_SERVER_ERROR,
        "failed to create user",
    )))?;

    manager
        .update_users_email(&UpdateUsersEmailRequest::new_verified(
            &realm_name,
            &user.id,
            &request.email,
        ))
        .await_err_as_failed_dependency()
        .await?;

    manager
        .assign_roles(&AssignRolesRequest::new(
            &realm_name,
            &user.id,
            &client.id,
            &[AssignRoleRequest::new(&role.id, &role.name)],
        ))
        .await_err_as_failed_dependency()
        .await?;

    match produce_vendor_created(request.email).await {
        Ok(()) => log::info!("event created"),
        Err(err) => log::error!("failed to send event: {err}"),
    };

    Ok(StatusCode::CREATED)
}

#[derive(Deserialize)]
struct VendorCustomerRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
struct VendorCreatedEvent {
    pub email: String,
}

async fn produce_vendor_created(email: String) -> Result<(), AppErr> {
    let kafka_host = env_var("KAFKA_HOST")?;
    let kafka_topic = env_var("KAFKA_VENDOR_TOPIC")?;

    kafka_producer::produce_event(&kafka_host, &kafka_topic, VendorCreatedEvent { email }).await?;

    Ok(())
}
