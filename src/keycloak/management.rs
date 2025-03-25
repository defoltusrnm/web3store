use http::StatusCode;
use reqwest::{Client, Response};
use serde::{Deserialize, Serialize};
use tokio::select;
use tokio_util::sync::CancellationToken;

use crate::utils::errors::AppErr;

use super::{
    authorization::AdminAccessTokenProvider, host::HostAddressProvider, routes::AdminRoutes,
};

pub trait KeycloakManagement {
    fn create_realm<TRoutes: AdminRoutes>(
        &self,
        request: &CreateRealmRequest,
        cancellation_token: &CancellationToken,
    ) -> impl Future<Output = Result<(), AppErr>>;

    fn create_client<TRoutes: AdminRoutes>(
        &self,
        request: &CreateClientRequest,
        cancellation_token: &CancellationToken,
    ) -> impl Future<Output = Result<(), AppErr>>;

    fn create_user<TRoutes: AdminRoutes>(
        &self,
        request: &CreateUserRequest,
        cancellation_token: &CancellationToken,
    ) -> impl Future<Output = Result<String, AppErr>>;

    fn query_users<TRoutes: AdminRoutes>(
        &self,
        request: &UsersQuery,
        cancellation_token: &CancellationToken,
    ) -> impl Future<Output = Result<Vec<UserResponse>, AppErr>>;

    fn query_clients<TRoutes: AdminRoutes>(
        &self,
        request: &ClientsQuery,
        cancellation_token: &CancellationToken,
    ) -> impl Future<Output = Result<Vec<ClientResponse>, AppErr>>;

    fn create_role<TRoutes: AdminRoutes>(
        &self,
        request: &CreateRoleRequest,
        cancellation_token: &CancellationToken,
    ) -> impl Future<Output = Result<(), AppErr>>;

    fn query_role<TRoutes: AdminRoutes>(
        &self,
        request: &RoleQuery,
        cancellation_token: &CancellationToken,
    ) -> impl Future<Output = Result<RoleResponse, AppErr>>;

    fn assign_roles<TRoutes: AdminRoutes>(
        &self,
        request: &AssignRolesRequest,
        cancellation_token: &CancellationToken,
    ) -> impl Future<Output = Result<(), AppErr>>;

    fn update_users_email<TRoutes: AdminRoutes>(
        &self,
        request: &UpdateUsersEmailRequest,
        cancellation_token: &CancellationToken,
    ) -> impl Future<Output = Result<(), AppErr>>;
}

pub struct DefaultKeycloakManagement<'a, TAuthorization, THost>
where
    TAuthorization: AdminAccessTokenProvider,
    THost: HostAddressProvider,
{
    auth_provider: &'a TAuthorization,
    host_provider: &'a THost,
}

impl<'a, TAuthorization, THost> DefaultKeycloakManagement<'a, TAuthorization, THost>
where
    TAuthorization: AdminAccessTokenProvider,
    THost: HostAddressProvider,
{
    pub fn new(auth_provider: &'a TAuthorization, host_provider: &'a THost) -> Self {
        DefaultKeycloakManagement {
            auth_provider,
            host_provider,
        }
    }
}

impl<'a, TAuthorization: AdminAccessTokenProvider, THost: HostAddressProvider> KeycloakManagement
    for DefaultKeycloakManagement<'a, TAuthorization, THost>
{
    async fn create_realm<TRoutes: AdminRoutes>(
        &self,
        request: &CreateRealmRequest,
        cancellation_token: &CancellationToken,
    ) -> Result<(), AppErr> {
        let url = TRoutes::get_create_realm_route(self.host_provider).await?;

        let token = self
            .auth_provider
            .get_access_token::<TRoutes>(cancellation_token)
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

    async fn create_client<TRoutes: AdminRoutes>(
        &self,
        request: &CreateClientRequest,
        cancellation_token: &CancellationToken,
    ) -> Result<(), AppErr> {
        let url = TRoutes::get_create_client_route(self.host_provider, &request.realm).await?;

        let token = self
            .auth_provider
            .get_access_token::<TRoutes>(cancellation_token)
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

    async fn create_user<TRoutes: AdminRoutes>(
        &self,
        request: &CreateUserRequest,
        cancellation_token: &CancellationToken,
    ) -> Result<String, AppErr> {
        let url = TRoutes::get_create_user_route(self.host_provider, &request.realm).await?;

        let token = self
            .auth_provider
            .get_access_token::<TRoutes>(cancellation_token)
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

    async fn query_users<TRoutes: AdminRoutes>(
        &self,
        request: &UsersQuery,
        cancellation_token: &CancellationToken,
    ) -> Result<Vec<UserResponse>, AppErr> {
        let url =
            TRoutes::get_users_query_route(self.host_provider, &request.realm, &request.username)
                .await?;

        let token = self
            .auth_provider
            .get_access_token::<TRoutes>(cancellation_token)
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

    async fn query_clients<TRoutes: AdminRoutes>(
        &self,
        request: &ClientsQuery,
        cancellation_token: &CancellationToken,
    ) -> Result<Vec<ClientResponse>, AppErr> {
        let url = TRoutes::get_clients_query_route(
            self.host_provider,
            &request.realm,
            &request.client_id,
        )
        .await?;

        let token = self
            .auth_provider
            .get_access_token::<TRoutes>(cancellation_token)
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

    async fn create_role<TRoutes: AdminRoutes>(
        &self,
        request: &CreateRoleRequest,
        cancellation_token: &CancellationToken,
    ) -> Result<(), AppErr> {
        let url = TRoutes::get_create_role_route(
            self.host_provider,
            &request.realm,
            &request.client_uuid,
        )
        .await?;

        let token = self
            .auth_provider
            .get_access_token::<TRoutes>(cancellation_token)
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

    async fn query_role<TRoutes: AdminRoutes>(
        &self,
        request: &RoleQuery,
        cancellation_token: &CancellationToken,
    ) -> Result<RoleResponse, AppErr> {
        let url = TRoutes::get_role_query_route(
            self.host_provider,
            &request.realm,
            &request.client_uuid,
            &request.role_name,
        )
        .await?;

        let token = self
            .auth_provider
            .get_access_token::<TRoutes>(cancellation_token)
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

    async fn assign_roles<TRoutes: AdminRoutes>(
        &self,
        request: &AssignRolesRequest,
        cancellation_token: &CancellationToken,
    ) -> Result<(), AppErr> {
        let url = TRoutes::get_assign_roles_query_route(
            self.host_provider,
            &request.realm,
            &request.user_uuid,
            &request.client_uuid,
        )
        .await?;

        let token = self
            .auth_provider
            .get_access_token::<TRoutes>(cancellation_token)
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

    async fn update_users_email<TRoutes: AdminRoutes>(
        &self,
        request: &UpdateUsersEmailRequest,
        cancellation_token: &CancellationToken,
    ) -> Result<(), AppErr> {
        let url =
            TRoutes::get_update_user_route(self.host_provider, &request.realm, &request.user_uuid)
                .await?;

        let token = self
            .auth_provider
            .get_access_token::<TRoutes>(cancellation_token)
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

#[derive(Serialize)]
pub struct CreateRealmRequest {
    pub realm: String,
    pub enabled: bool,
}

impl CreateRealmRequest {
    pub fn new(realm: &str) -> Self {
        CreateRealmRequest {
            realm: realm.to_owned(),
            enabled: true,
        }
    }
}

#[derive(Serialize)]
pub struct CreateClientRequest {
    #[serde(skip)]
    pub realm: String,
    #[serde(rename = "clientId")]
    pub client_id: String,
    pub enabled: bool,
    #[serde(rename = "publicClient")]
    pub public_client: bool,
    pub secret: String,
    #[serde(rename = "directAccessGrantsEnabled")]
    pub direct_access_grants_enabled: bool,
}

impl CreateClientRequest {
    pub fn new(client: &str, realm: &str, secret: &str) -> Self {
        CreateClientRequest {
            realm: realm.to_owned(),
            client_id: client.to_owned(),
            enabled: true,
            public_client: false,
            secret: secret.to_owned(),
            direct_access_grants_enabled: true,
        }
    }
}

#[derive(Serialize)]
pub struct CreateUserRequest {
    #[serde(skip)]
    pub realm: String,
    pub username: String,
    pub enabled: bool,
    pub credentials: [CreateUserCredentialsRequest; 1],
}

impl CreateUserRequest {
    pub fn new(realm: &str, username: &str, password: &str) -> Self {
        CreateUserRequest {
            realm: realm.to_owned(),
            username: username.to_owned(),
            enabled: true,
            credentials: [CreateUserCredentialsRequest::new(password)],
        }
    }
}

#[derive(Serialize)]
pub struct CreateUserCredentialsRequest {
    #[serde(rename = "type")]
    pub user_type: String,
    pub value: String,
    pub temporary: bool,
}

impl CreateUserCredentialsRequest {
    pub fn new(password: &str) -> Self {
        CreateUserCredentialsRequest {
            user_type: "password".to_owned(),
            value: password.to_owned(),
            temporary: false,
        }
    }
}

pub struct UsersQuery {
    pub realm: String,
    pub username: String,
}

impl UsersQuery {
    pub fn new(realm: &str, username: &str) -> Self {
        UsersQuery {
            realm: realm.to_owned(),
            username: username.to_owned(),
        }
    }
}

#[derive(Deserialize)]
pub struct UserResponse {
    pub id: String,
    pub username: String,
    pub enabled: bool,
}

pub struct ClientsQuery {
    pub realm: String,
    pub client_id: String,
}

impl ClientsQuery {
    pub fn new(realm: &str, client_id: &str) -> Self {
        ClientsQuery {
            realm: realm.to_owned(),
            client_id: client_id.to_owned(),
        }
    }
}

#[derive(Deserialize)]
pub struct ClientResponse {
    pub id: String,
    #[serde(rename = "clientId")]
    pub client_id: String,
}

#[derive(Serialize)]
pub struct CreateRoleRequest {
    #[serde(skip)]
    pub realm: String,
    #[serde(skip)]
    pub client_uuid: String,
    pub name: String,
    pub description: String,
    pub composite: bool,
    #[serde(rename = "clientRole")]
    pub client_role: bool,
}

impl CreateRoleRequest {
    pub fn new(realm: &str, client_uuid: &str, name: &str, description: &str) -> Self {
        CreateRoleRequest {
            realm: realm.to_owned(),
            client_uuid: client_uuid.to_owned(),
            name: name.to_owned(),
            description: description.to_owned(),
            composite: false,
            client_role: true,
        }
    }
}

pub struct RoleQuery {
    pub realm: String,
    pub client_uuid: String,
    pub role_name: String,
}

impl RoleQuery {
    pub fn new(realm: &str, client_uuid: &str, role_name: &str) -> Self {
        RoleQuery {
            realm: realm.to_owned(),
            client_uuid: client_uuid.to_owned(),
            role_name: role_name.to_owned(),
        }
    }
}

#[derive(Deserialize)]
pub struct RoleResponse {
    pub id: String,
    pub name: String,
}

pub struct AssignRolesRequest {
    pub realm: String,
    pub user_uuid: String,
    pub client_uuid: String,
    pub assign_roles: Vec<AssignRoleRequest>,
}

impl AssignRolesRequest {
    pub fn new(
        realm: &str,
        user_uuid: &str,
        client_uuid: &str,
        assign_roles: &[AssignRoleRequest],
    ) -> Self {
        AssignRolesRequest {
            realm: realm.to_owned(),
            user_uuid: user_uuid.to_owned(),
            client_uuid: client_uuid.to_owned(),
            assign_roles: assign_roles.to_vec(),
        }
    }
}

#[derive(Serialize, Clone)]
pub struct AssignRoleRequest {
    pub id: String,
    pub name: String,
}

impl AssignRoleRequest {
    pub fn new(id: &str, name: &str) -> Self {
        AssignRoleRequest {
            id: id.to_owned(),
            name: name.to_owned(),
        }
    }
}

#[derive(Serialize)]
pub struct UpdateUsersEmailRequest {
    #[serde(skip)]
    pub realm: String,
    #[serde(skip)]
    pub user_uuid: String,
    pub email: String,
    #[serde(rename = "emailVerified")]
    pub email_verified: bool,
}

impl UpdateUsersEmailRequest {
    pub fn new_verified(realm: &str, user_uuid: &str, email: &str) -> Self {
        UpdateUsersEmailRequest {
            realm: realm.to_owned(),
            user_uuid: user_uuid.to_owned(),
            email: email.to_owned(),
            email_verified: true,
        }
    }
}
