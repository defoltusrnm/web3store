use tokio_util::sync::CancellationToken;

use crate::{keycloak::services::responses::access_token::AccessTokenResponse, utils::errors::AppErr};

pub trait AdminAccessTokenProvider {
    fn get_access_token(
        &self,
        cancellation_token: &CancellationToken,
    ) -> impl Future<Output = Result<AccessTokenResponse, AppErr>>;
}
