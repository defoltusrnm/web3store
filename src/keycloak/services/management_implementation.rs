use reqwest::{Client, Response};
use tokio::select;
use tokio_util::sync::CancellationToken;

use crate::utils::{
    errors::AppErr,
    http::{ResponseExtended, SendExtended},
};

use super::{
    authorization::AdminAccessTokenProvider,
    management::KeycloakManagement,
    queries::{clients::ClientsQuery, role::RoleQuery, users::UsersQuery},
    requests::{
        assign_roles::AssignRolesRequest, create_client::CreateClientRequest,
        create_realm::CreateRealmRequest, create_role::CreateRoleRequest,
        create_user::CreateUserRequest, update_users_email_request::UpdateUsersEmailRequest,
    },
    responses::{client::ClientResponse, role::RoleResponse, user::UserResponse},
    routes::AdminRoutes,
};

pub struct DefaultKeycloakManagement<'a, TAuthorization, TRoutes>
where
    TAuthorization: AdminAccessTokenProvider,
    TRoutes: AdminRoutes,
{
    auth_provider: &'a TAuthorization,
    routes: &'a TRoutes,
}

impl<'a, TAuthorization, TRoutes> DefaultKeycloakManagement<'a, TAuthorization, TRoutes>
where
    TAuthorization: AdminAccessTokenProvider,
    TRoutes: AdminRoutes,
{
    pub fn new(auth_provider: &'a TAuthorization, routes: &'a TRoutes) -> Self {
        DefaultKeycloakManagement {
            auth_provider,
            routes,
        }
    }
}

impl<'a, TAuthorization: AdminAccessTokenProvider, TRoutes: AdminRoutes> KeycloakManagement
    for DefaultKeycloakManagement<'a, TAuthorization, TRoutes>
{
    async fn create_realm(
        &self,
        request: &CreateRealmRequest,
        cancellation_token: &CancellationToken,
    ) -> Result<(), AppErr> {
        let url = self.routes.get_create_realm_route().await?;

        let token = self
            .auth_provider
            .get_access_token(cancellation_token)
            .await?;

        let create_realm_response = select! {
            resp = Client::new().quick_post(&url, request, Some(token.access_token)) => resp,
            _ = cancellation_token.cancelled() => AppErr::cancelled()
        }?;

        create_realm_response.ensure_success().await?;
        Ok(())
    }

    async fn create_client(
        &self,
        request: &CreateClientRequest,
        cancellation_token: &CancellationToken,
    ) -> Result<(), AppErr> {
        let url = self.routes.get_create_client_route(&request.realm).await?;

        let token = self
            .auth_provider
            .get_access_token(cancellation_token)
            .await?;

        let create_client_response = select! {
            resp = Client::new().quick_post(&url, request, Some(token.access_token)) => resp,
            _ = cancellation_token.cancelled() => AppErr::cancelled()
        }?;

        create_client_response.ensure_success().await?;
        Ok(())
    }

    async fn create_user(
        &self,
        request: &CreateUserRequest,
        cancellation_token: &CancellationToken,
    ) -> Result<(), AppErr> {
        let url = self.routes.get_create_user_route(&request.realm).await?;

        let token = self
            .auth_provider
            .get_access_token(cancellation_token)
            .await?;

        let create_user_response = select! {
            resp = Client::new().quick_post(&url, request, Some(token.access_token)) => resp,
            _ = cancellation_token.cancelled() => Result::<Response, AppErr>::Err(AppErr::from("create realm request cancelled"))
        }?;

        create_user_response.ensure_success().await?;
        Ok(())
    }

    async fn query_users(
        &self,
        request: &UsersQuery,
        cancellation_token: &CancellationToken,
    ) -> Result<Vec<UserResponse>, AppErr> {
        let url = self
            .routes
            .get_users_query_route(&request.realm, &request.username)
            .await?;

        let token = self
            .auth_provider
            .get_access_token(cancellation_token)
            .await?;

        let response = select! {
            resp = Client::new().quick_get(&url, Some(token.access_token)) => resp,
            _ = cancellation_token.cancelled() => AppErr::cancelled()
        }?;

        let users = response.ensure_success_json::<Vec<UserResponse>>().await?;
        Ok(users)
    }

    async fn query_clients(
        &self,
        request: &ClientsQuery,
        cancellation_token: &CancellationToken,
    ) -> Result<Vec<ClientResponse>, AppErr> {
        let url = self
            .routes
            .get_clients_query_route(&request.realm, &request.client_id)
            .await?;

        let token = self
            .auth_provider
            .get_access_token(cancellation_token)
            .await?;

        let response = select! {
            resp = Client::new().quick_get(&url, Some(token.access_token)) => resp,
            _ = cancellation_token.cancelled() => AppErr::cancelled()
        }?;

        let clients = response
            .ensure_success_json::<Vec<ClientResponse>>()
            .await?;
        Ok(clients)
    }

    async fn create_role(
        &self,
        request: &CreateRoleRequest,
        cancellation_token: &CancellationToken,
    ) -> Result<(), AppErr> {
        let url = self
            .routes
            .get_create_role_route(&request.realm, &request.client_uuid)
            .await?;

        let token = self
            .auth_provider
            .get_access_token(cancellation_token)
            .await?;

        let create_role_response = select! {
            resp = Client::new().quick_post(&url, request, Some(token.access_token)) => resp,
            _ = cancellation_token.cancelled() => AppErr::cancelled()
        }?;

        create_role_response.ensure_success().await?;
        Ok(())
    }

    async fn query_role(
        &self,
        request: &RoleQuery,
        cancellation_token: &CancellationToken,
    ) -> Result<RoleResponse, AppErr> {
        let url = self
            .routes
            .get_role_query_route(&request.realm, &request.client_uuid, &request.role_name)
            .await?;

        let token = self
            .auth_provider
            .get_access_token(cancellation_token)
            .await?;

        let response = select! {
            resp = Client::new().quick_get(&url, Some(token.access_token)) => resp,
            _ = cancellation_token.cancelled() => AppErr::cancelled()
        }?;

        let role = response.ensure_success_json::<RoleResponse>().await?;
        Ok(role)
    }

    async fn assign_roles(
        &self,
        request: &AssignRolesRequest,
        cancellation_token: &CancellationToken,
    ) -> Result<(), AppErr> {
        let url = self
            .routes
            .get_assign_roles_query_route(&request.realm, &request.user_uuid, &request.client_uuid)
            .await?;

        let token = self
            .auth_provider
            .get_access_token(cancellation_token)
            .await?;

        let response = select! {
            resp = Client::new().quick_post(&url, &request.assign_roles, Some(token.access_token)) =>  resp,
            _ = cancellation_token.cancelled() => AppErr::cancelled()
        }?;

        response.ensure_success().await?;
        Ok(())
    }

    async fn update_users_email(
        &self,
        request: &UpdateUsersEmailRequest,
        cancellation_token: &CancellationToken,
    ) -> Result<(), AppErr> {
        let url = self
            .routes
            .get_update_user_route(&request.realm, &request.user_uuid)
            .await?;

        let token = self
            .auth_provider
            .get_access_token(cancellation_token)
            .await?;

        let response = select! {
            resp = Client::new().quick_post(&url, &request, Some(token.access_token)) => resp,
            _ = cancellation_token.cancelled() => AppErr::cancelled()
        }?;

        response.ensure_success().await?;
        Ok(())
    }
}
