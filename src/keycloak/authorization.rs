use std::collections::HashMap;

use derive_more::Display;
use http::StatusCode;
use reqwest::{Client, Response};
use serde::Deserialize;
use tokio::select;
use tokio_util::sync::CancellationToken;

use super::{
    credentials::AdminCredentialProvider,
    host::HostAddressProvider,
    routes::{AdminRoutes, DefaultAdminRoutes},
};
use crate::utils::errors::AppErr;

pub trait AdminAccessTokenProvider<TRoutes, THostProvider, TAdminCredentialProvider>
where
    TRoutes: AdminRoutes,
    THostProvider: HostAddressProvider,
    TAdminCredentialProvider: AdminCredentialProvider,
{
    fn get_access_token(
        host_provider: THostProvider,
        credential_provider: TAdminCredentialProvider,
        cancellation_token: CancellationToken,
    ) -> impl Future<Output = Result<AccessTokenResponse, AppErr>>;
}

pub struct DefaultAdminTokenProvider;

impl<THostProvider, TAdminCredentialProvider>
    AdminAccessTokenProvider<DefaultAdminRoutes, THostProvider, TAdminCredentialProvider>
    for DefaultAdminTokenProvider
where
    THostProvider: HostAddressProvider,
    TAdminCredentialProvider: AdminCredentialProvider,
{
    async fn get_access_token(
        host_provider: THostProvider,
        credential_provider: TAdminCredentialProvider,
        cancellation_token: CancellationToken,
    ) -> Result<AccessTokenResponse, AppErr> {
        let auth_route = DefaultAdminRoutes::get_access_token_route(host_provider).await?;
        let login = credential_provider.get_login().await?;
        let password = credential_provider.get_password().await?;

        let mut form_data = HashMap::new();
        form_data.insert("client_id", "admin-cli");
        form_data.insert("username", &login);
        form_data.insert("password", &password);
        form_data.insert("grant_type", "password");

        let auth_response = select! {
            resp = Client::new()
            .post(&auth_route)
            .form(&form_data)
            .send() => resp.map_err(|err| AppErr::from_owned(format!("admin auth call err: {err}"))),
            _ = cancellation_token.cancelled() => Result::<Response, AppErr>::Err(AppErr::from("auth call cancelled"))

        }?;

        match auth_response.status() {
            StatusCode::OK => select! {
                body = auth_response.json::<AccessTokenResponse>() => body.map_err(|err| AppErr::from_owned(format!("reading body err: {err}"))) ,
                _ = cancellation_token.cancelled() => Result::<AccessTokenResponse, AppErr>::Err(AppErr::from("body read cancelled"))
            },
            _ => Err(AppErr::from("da fuq")),
        }
    }
}

#[derive(Deserialize, Display)]
pub struct AccessTokenResponse {
    pub access_token: String,
}
