use http::StatusCode;
use reqwest::{Client, Response};
use tokio::select;
use tokio_util::sync::CancellationToken;

use crate::utils::errors::AppErr;

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
            resp = Client::new().post(url).bearer_auth(token.access_token).json(request).send() => resp.map_err(|err| AppErr::from_owned(format!("create realm http error: {err}"))),
            _ = cancellation_token.cancelled() => Result::<Response, AppErr>::Err(AppErr::from("create realm request cancelled"))
        }?;

        let status = create_realm_response.status();
        match status {
            StatusCode::OK => Ok(()),
            StatusCode::CREATED => Ok(()),
            _ => Err(AppErr::from_owned(format!(
                "create realm status code: {status}"
            ))),
        }
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
            resp = Client::new().post(url).bearer_auth(token.access_token).json(request).send() => resp.map_err(|err| AppErr::from_owned(format!("create realm http error: {err}"))),
            _ = cancellation_token.cancelled() => Result::<Response, AppErr>::Err(AppErr::from("create realm request cancelled"))
        }?;

        let status = create_client_response.status();
        match status {
            StatusCode::OK => Ok(()),
            StatusCode::CREATED => Ok(()),
            _ => Err(AppErr::from_owned(format!(
                "create client status code: {status}"
            ))),
        }
    }

    async fn create_user(
        &self,
        request: &CreateUserRequest,
        cancellation_token: &CancellationToken,
    ) -> Result<String, AppErr> {
        let url = self.routes.get_create_user_route(&request.realm).await?;

        let token = self
            .auth_provider
            .get_access_token(cancellation_token)
            .await?;

        let create_user_response = select! {
            resp = Client::new().post(url).bearer_auth(token.access_token).json(request).send() => resp.map_err(|err| AppErr::from_owned(format!("create realm http error: {err}"))),
            _ = cancellation_token.cancelled() => Result::<Response, AppErr>::Err(AppErr::from("create realm request cancelled"))
        }?;

        let status = create_user_response.status();
        let body = create_user_response
            .text()
            .await
            .inspect_err(|err| log::warn!("cannot read body on create user: {err}"))
            .ok()
            .unwrap_or("".to_owned());

        match status {
            StatusCode::OK => Ok(body),
            StatusCode::CREATED => Ok(body),
            _ => Err(AppErr::from_owned(format!(
                "create user status code: {status} with body {body}"
            ))),
        }
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
            resp = Client::new().get(url).bearer_auth(token.access_token).send() => {
                resp.map_err(|err| AppErr::from_owned(format!("querying users err: {err}")))
            }
            _ = cancellation_token.cancelled() => Result::<Response, AppErr>::Err(AppErr::from("user querying cancelled"))
        }?;

        let status = response.status();

        match status {
            StatusCode::OK => response
                .json::<Vec<UserResponse>>()
                .await
                .map_err(|err| AppErr::from_owned(format!("cannot read users json: {err}"))),
            _ => {
                let body = response
                    .text()
                    .await
                    .map_err(|err| AppErr::from_owned(format!("[{status}]: {err}")))?;
                Result::<Vec<UserResponse>, AppErr>::Err(AppErr::from_owned(format!(
                    "[status]: {body}"
                )))
            }
        }
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
            resp = Client::new().get(url).bearer_auth(token.access_token).send() => {
                resp.map_err(|err| AppErr::from_owned(format!("querying clients err: {err}")))
            }
            _ = cancellation_token.cancelled() => Result::<Response, AppErr>::Err(AppErr::from("client querying cancelled"))
        }?;

        let status = response.status();

        match status {
            StatusCode::OK => response
                .json::<Vec<ClientResponse>>()
                .await
                .map_err(|err| AppErr::from_owned(format!("cannot read client json: {err}"))),
            _ => {
                let body = response
                    .text()
                    .await
                    .map_err(|err| AppErr::from_owned(format!("[{status}]: {err}")))?;
                Result::<Vec<ClientResponse>, AppErr>::Err(AppErr::from_owned(format!(
                    "[status]: {body}"
                )))
            }
        }
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
            resp = Client::new().post(url).bearer_auth(token.access_token).json(request).send() => resp.map_err(|err| AppErr::from_owned(format!("create role http error: {err}"))),
            _ = cancellation_token.cancelled() => Result::<Response, AppErr>::Err(AppErr::from("create role request cancelled"))
        }?;

        let status = create_role_response.status();
        let body = create_role_response
            .text()
            .await
            .inspect_err(|err| log::warn!("cannot read body on create role: {err}"))
            .ok()
            .unwrap_or("".to_owned());

        match status {
            StatusCode::OK => Ok(()),
            StatusCode::CREATED => Ok(()),
            _ => Err(AppErr::from_owned(format!(
                "create role status code: {status} with body {body}"
            ))),
        }
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
            resp = Client::new().get(url).bearer_auth(token.access_token).send() => {
                resp.map_err(|err| AppErr::from_owned(format!("querying role err: {err}")))
            }
            _ = cancellation_token.cancelled() => Result::<Response, AppErr>::Err(AppErr::from("role querying cancelled"))
        }?;

        let status = response.status();

        match status {
            StatusCode::OK => response
                .json::<RoleResponse>()
                .await
                .map_err(|err| AppErr::from_owned(format!("cannot read role json: {err}"))),
            _ => {
                let body = response
                    .text()
                    .await
                    .map_err(|err| AppErr::from_owned(format!("[{status}]: {err}")))?;
                Result::<RoleResponse, AppErr>::Err(AppErr::from_owned(format!("[status]: {body}")))
            }
        }
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
            resp = Client::new().post(url).bearer_auth(token.access_token).json(&request.assign_roles).send() => {
                resp.map_err(|err| AppErr::from_owned(format!("roles assignment err: {err}")))
            }
            _ = cancellation_token.cancelled() => Result::<Response, AppErr>::Err(AppErr::from("roles assignment cancelled"))
        }?;

        let status = response.status();
        let body = response
            .text()
            .await
            .inspect_err(|err| log::warn!("cannot read body on roles assignment: {err}"))
            .ok()
            .unwrap_or("".to_owned());

        match status {
            StatusCode::OK => Ok(()),
            StatusCode::CREATED => Ok(()),
            StatusCode::NO_CONTENT => Ok(()),
            _ => Err(AppErr::from_owned(format!(
                "roles assignment status code: {status} with body {body}"
            ))),
        }
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
            resp = Client::new().put(url).bearer_auth(token.access_token).json(request).send() => resp.map_err(|err| AppErr::from_owned(format!("create realm http error: {err}"))),
            _ = cancellation_token.cancelled() => Result::<Response, AppErr>::Err(AppErr::from("update user's email request cancelled"))
        }?;

        let status = response.status();
        let body = response
            .text()
            .await
            .inspect_err(|err| log::warn!("cannot read body on update user's email: {err}"))
            .ok()
            .unwrap_or("".to_owned());

        match status {
            StatusCode::OK => Ok(()),
            StatusCode::CREATED => Ok(()),
            StatusCode::NO_CONTENT => Ok(()),
            _ => Err(AppErr::from_owned(format!(
                "update user's email status code: {status} with body {body}"
            ))),
        }
    }
}
