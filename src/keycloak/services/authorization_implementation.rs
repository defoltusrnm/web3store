use std::collections::HashMap;

use http::StatusCode;
use reqwest::{Client, Response};
use tokio::select;
use tokio_util::sync::CancellationToken;

use crate::utils::errors::AppErr;

use super::{
    authorization::AdminAccessTokenProvider, credentials::AdminCredentialProvider,
    responses::access_token::AccessTokenResponse, routes::AdminRoutes,
};

pub struct DefaultAdminTokenProvider<'a, TRoutes, TAdminCredentialProvider>
where
    TRoutes: AdminRoutes,
    TAdminCredentialProvider: AdminCredentialProvider,
{
    routes: &'a TRoutes,
    credentials_provider: &'a TAdminCredentialProvider,
}

impl<'a, TRoutes, TAdminCredentialProvider>
    DefaultAdminTokenProvider<'a, TRoutes, TAdminCredentialProvider>
where
    TRoutes: AdminRoutes,
    TAdminCredentialProvider: AdminCredentialProvider,
{
    pub fn new(routes: &'a TRoutes, credentials_provider: &'a TAdminCredentialProvider) -> Self {
        DefaultAdminTokenProvider {
            routes,
            credentials_provider,
        }
    }
}

impl<'a, TRoutes, TAdminCredentialProvider> AdminAccessTokenProvider
    for DefaultAdminTokenProvider<'a, TRoutes, TAdminCredentialProvider>
where
    TRoutes: AdminRoutes,
    TAdminCredentialProvider: AdminCredentialProvider,
{
    async fn get_access_token(
        &self,
        cancellation_token: &CancellationToken,
    ) -> Result<AccessTokenResponse, AppErr> {
        let auth_route = self.routes.get_access_token_route().await?;
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
