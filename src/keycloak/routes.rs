use crate::utils::errors::AppErr;

use super::host::HostAddressProvider;

pub trait AdminRoutes {
    fn get_access_token_route<THost: HostAddressProvider>(
        provider: THost,
    ) -> impl Future<Output = Result<String, AppErr>>;
}

pub struct DefaultAdminRoutes;

impl AdminRoutes for DefaultAdminRoutes {
    async fn get_access_token_route<THost: HostAddressProvider>(
        provider: THost,
    ) -> Result<String, AppErr> {
        provider.get_host().await.map(|x| format!("{0}/realms/master/protocol/openid-connect/token", x))
    }
}
