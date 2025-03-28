use tokio_util::sync::CancellationToken;

use crate::{
    keycloak::services::responses::access_token::AccessTokenResponse, utils::errors::AppErr,
};

pub trait AdminAccessTokenProvider {
    fn get_access_token(&self) -> impl Future<Output = Result<AccessTokenResponse, AppErr>>;
    fn get_access_token_with_cancel(
        &self,
        cancellation_token: &CancellationToken,
    ) -> impl Future<Output = Result<AccessTokenResponse, AppErr>>;
}
