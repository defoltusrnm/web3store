use tokio_util::sync::CancellationToken;
use utils::errors::AppErr;

use crate::keycloak::services::responses::access_token::AccessTokenResponse;

pub trait AdminAccessTokenProvider {
    fn get_access_token(&self) -> impl Future<Output = Result<AccessTokenResponse, AppErr>>;
    fn get_access_token_with_cancel(
        &self,
        cancellation_token: &CancellationToken,
    ) -> impl Future<Output = Result<AccessTokenResponse, AppErr>>;
}
