use axum::{Json, Router, response::Result, routing::post};
use http::StatusCode;
use serde::Deserialize;
use tokio_util::sync::CancellationToken;

use crate::{
    keycloak::services::{
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
    utils::{env::env_var, errors::HttpAppErr},
};

pub fn create_customer_router() -> Router {
    Router::new().route("/api/customers", post(create_customer))
}

async fn create_customer(Json(request): Json<CreateCustomerRequest>) -> Result<StatusCode> {
    let host_provider = &EnvHostAddressProvider::new("KEYCLOAK_HOST");
    let credentials_provider =
        &EnvAdminCredentialProvider::new("KEYCLOAK_ADMIN_LOGIN", "KEYCLOAK_ADMIN_PASSWORD");
    let routes = &DefaultAdminRoutes::new(host_provider);
    let auth_provider = &DefaultAdminTokenProvider::new(routes, credentials_provider);
    let manager = &DefaultKeycloakManagement::new(auth_provider, routes);

    let realm_name = env_var("KEYCLOAK_REALM")
        .map_err(|_| HttpAppErr::new(StatusCode::INTERNAL_SERVER_ERROR, "Error"))?;

    let client_name = env_var("KEYCLOAK_CLIENT")
        .map_err(|_| HttpAppErr::new(StatusCode::INTERNAL_SERVER_ERROR, "Error"))?;

    let role_name = env_var("KEYCLOAK_CUSTOMER_ROLE")
        .map_err(|_| HttpAppErr::new(StatusCode::INTERNAL_SERVER_ERROR, "Error"))?;

    let clients = manager
        .query_clients(
            &ClientsQuery::new(&realm_name, &client_name),
            &CancellationToken::new(),
        )
        .await
        .inspect_err(|err| log::error!("failed to retrieve client: {err}"))
        .map_err(|_| HttpAppErr::new(StatusCode::FAILED_DEPENDENCY, "keycloak error"))?;

    let client = clients
        .get(0)
        .map(Result::Ok)
        .unwrap_or(Err(HttpAppErr::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "failed to create user",
        )))?;

    let role = manager
        .query_role(
            &RoleQuery::new(&realm_name, &client.id, &role_name),
            &CancellationToken::new(),
        )
        .await
        .inspect_err(|err| log::error!("failed to retrieve role: {err}"))
        .map_err(|_| HttpAppErr::new(StatusCode::FAILED_DEPENDENCY, "keycloak error"))?;

    manager
        .create_user(
            &CreateUserRequest::new(&realm_name, &request.email, &request.password),
            &CancellationToken::new(),
        )
        .await
        .inspect_err(|err| log::error!("failed to create user: {err}"))
        .map_err(|_| HttpAppErr::new(StatusCode::FAILED_DEPENDENCY, "keycloak error"))?;

    let users = manager
        .query_users(
            &UsersQuery::new(&realm_name, &request.email),
            &CancellationToken::new(),
        )
        .await
        .inspect_err(|err| log::error!("failed to retrieve user: {err}"))
        .map_err(|_| HttpAppErr::new(StatusCode::FAILED_DEPENDENCY, "keycloak error"))?;

    let user = users.get(0).map(Result::Ok).unwrap_or(Err(HttpAppErr::new(
        StatusCode::INTERNAL_SERVER_ERROR,
        "failed to create user",
    )))?;

    manager
        .update_users_email(
            &UpdateUsersEmailRequest::new_verified(&realm_name, &user.id, &request.email),
            &CancellationToken::new(),
        )
        .await
        .inspect_err(|err| log::error!("failed to update user email: {err}"))
        .map_err(|_| HttpAppErr::new(StatusCode::FAILED_DEPENDENCY, "keycloak error"))?;

    manager
        .assign_roles(
            &AssignRolesRequest::new(
                &realm_name,
                &user.id,
                &client.id,
                &[AssignRoleRequest::new(&role.id, &role.name)],
            ),
            &CancellationToken::new(),
        )
        .await
        .inspect_err(|err| log::error!("failed to update user email: {err}"))
        .map_err(|_| HttpAppErr::new(StatusCode::FAILED_DEPENDENCY, "keycloak error"))?;

    Ok(StatusCode::CREATED)
}

#[derive(Deserialize)]
struct CreateCustomerRequest {
    pub email: String,
    pub password: String,
}
