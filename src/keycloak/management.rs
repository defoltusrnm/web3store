use http::StatusCode;
use reqwest::{Client, Response};
use serde::Serialize;
use tokio::select;
use tokio_util::sync::CancellationToken;

use crate::utils::errors::AppErr;

use super::{
    authorization::AdminAccessTokenProvider,
    host::HostAddressProvider,
    routes::AdminRoutes,
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
    ) -> Result<(), AppErr> {
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
        match status {
            StatusCode::OK => Ok(()),
            StatusCode::CREATED => Ok(()),
            _ => Err(AppErr::from_owned(format!(
                "create client status code: {status}"
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
    pub credentials: CreateUserCredentialsRequest,
}

impl CreateUserRequest {
    pub fn new(realm: &str, username: &str, password: &str) -> Self {
        CreateUserRequest {
            realm: realm.to_owned(),
            username: username.to_owned(),
            enabled: true,
            credentials: CreateUserCredentialsRequest::new(password),
        }
    }
}

#[derive(Serialize)]
pub struct CreateUserCredentialsRequest {
    pub r#type: String,
    pub value: String,
    pub temporary: bool,
}

impl CreateUserCredentialsRequest {
    pub fn new(password: &str) -> Self {
        CreateUserCredentialsRequest {
            r#type: "password".to_owned(),
            value: password.to_owned(),
            temporary: false,
        }
    }
}
