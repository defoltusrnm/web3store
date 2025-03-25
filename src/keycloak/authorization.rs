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

pub trait AdminAccessTokenProvider {
    fn get_access_token<TRoutes>(
        &self,
        cancellation_token: &CancellationToken,
    ) -> impl Future<Output = Result<AccessTokenResponse, AppErr>>
    where
        TRoutes: AdminRoutes;
}

pub struct DefaultAdminTokenProvider<'a, THostProvider, TAdminCredentialProvider>
where
    THostProvider: HostAddressProvider,
    TAdminCredentialProvider: AdminCredentialProvider,
{
    host_provider: &'a THostProvider,
    credentials_provider: &'a TAdminCredentialProvider,
}

impl<THostProvider, TAdminCredentialProvider>
    DefaultAdminTokenProvider<THostProvider, TAdminCredentialProvider>
where
    THostProvider: HostAddressProvider,
    TAdminCredentialProvider: AdminCredentialProvider,
{
    pub fn new(
        host_provider: THostProvider,
        credentials_provider: TAdminCredentialProvider,
    ) -> Self {
        DefaultAdminTokenProvider {
            host_provider,
            credentials_provider,
        }
    }
}

impl<THostProvider, TAdminCredentialProvider> AdminAccessTokenProvider
    for DefaultAdminTokenProvider<THostProvider, TAdminCredentialProvider>
where
    THostProvider: HostAddressProvider,
    TAdminCredentialProvider: AdminCredentialProvider,
{
    async fn get_access_token<TRoutes: AdminRoutes>(
        &self,
        cancellation_token: &CancellationToken,
    ) -> Result<AccessTokenResponse, AppErr> {
        let auth_route = TRoutes::get_access_token_route(&self.host_provider).await?;
        let login = self.credentials_provider.get_login().await?;
        let password = self.credentials_provider.get_password().await?;

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
