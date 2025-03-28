use std::collections::HashMap;

use reqwest::{Client, Response};
use tokio::select;
use tokio_util::sync::CancellationToken;
use utils::{errors::AppErr, http::ResponseExtended};

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
    async fn get_access_token_with_cancel(
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

        let token = auth_response
            .ensure_success_json::<AccessTokenResponse>()
            .await?;
        Ok(token)
    }

    async fn get_access_token(&self) -> Result<AccessTokenResponse, AppErr> {
        self.get_access_token_with_cancel(&CancellationToken::new())
            .await
    }
}
