use crate::utils::errors::AppErr;

use super::host::HostAddressProvider;

pub trait AdminRoutes {
    fn get_access_token_route<THost: HostAddressProvider>(
        provider: &THost,
    ) -> impl Future<Output = Result<String, AppErr>>;

    fn get_create_realm_route<THost: HostAddressProvider>(
        provider: &THost,
    ) -> impl Future<Output = Result<String, AppErr>>;

    fn get_create_client_route<THost: HostAddressProvider>(
        provider: &THost,
        realm: &str,
    ) -> impl Future<Output = Result<String, AppErr>>;

    fn get_create_user_route<THost: HostAddressProvider>(
        provider: &THost,
        realm: &str,
    ) -> impl Future<Output = Result<String, AppErr>>;

    fn get_users_query_route<THost: HostAddressProvider>(
        provider: &THost,
        realm: &str,
        username: &str,
    ) -> impl Future<Output = Result<String, AppErr>>;

    fn get_clients_query_route<THost: HostAddressProvider>(
        provider: &THost,
        realm: &str,
        client_id: &str,
    ) -> impl Future<Output = Result<String, AppErr>>;

    fn get_create_role_route<THost: HostAddressProvider>(
        provider: &THost,
        realm: &str,
        client_uuid: &str,
    ) -> impl Future<Output = Result<String, AppErr>>;

    fn get_role_query_route<THost: HostAddressProvider>(
        provider: &THost,
        realm: &str,
        client_uuid: &str,
        role_name: &str,
    ) -> impl Future<Output = Result<String, AppErr>>;
}

pub struct DefaultAdminRoutes;

impl AdminRoutes for DefaultAdminRoutes {
    async fn get_access_token_route<THost: HostAddressProvider>(
        provider: &THost,
    ) -> Result<String, AppErr> {
        provider
            .get_host()
            .await
            .map(|x| format!("{0}/realms/master/protocol/openid-connect/token", x))
    }

    async fn get_create_realm_route<THost: HostAddressProvider>(
        provider: &THost,
    ) -> Result<String, AppErr> {
        provider
            .get_host()
            .await
            .map(|x| format!("{0}/admin/realms", x))
    }

    async fn get_create_client_route<THost: HostAddressProvider>(
        provider: &THost,
        realm: &str,
    ) -> Result<String, AppErr> {
        let host = provider.get_host().await?;

        Ok(format!("{0}/admin/realms/{1}/clients", host, realm))
    }

    async fn get_create_user_route<THost: HostAddressProvider>(
        provider: &THost,
        realm: &str,
    ) -> Result<String, AppErr> {
        let host = provider.get_host().await?;

        Ok(format!("{0}/admin/realms/{1}/users", host, realm))
    }

    async fn get_users_query_route<THost: HostAddressProvider>(
        provider: &THost,
        realm: &str,
        username: &str,
    ) -> Result<String, AppErr> {
        let host = provider.get_host().await?;

        Ok(format!(
            "{0}/admin/realms/{1}/users?username={2}",
            host, realm, username
        ))
    }

    async fn get_clients_query_route<THost: HostAddressProvider>(
        provider: &THost,
        realm: &str,
        client_id: &str,
    ) -> Result<String, AppErr> {
        let host = provider.get_host().await?;

        Ok(format!(
            "{0}/admin/realms/{1}/clients?clientId={2}",
            host, realm, client_id
        ))
    }

    async fn get_create_role_route<THost: HostAddressProvider>(
        provider: &THost,
        realm: &str,
        client_uuid: &str,
    ) -> Result<String, AppErr> {
        let host = provider.get_host().await?;

        Ok(format!(
            "{0}/admin/realms/{1}/clients/{2}/roles",
            host, realm, client_uuid
        ))
    }

    async fn get_role_query_route<THost: HostAddressProvider>(
        provider: &THost,
        realm: &str,
        client_uuid: &str,
        role_name: &str,
    ) -> Result<String, AppErr> {
        let host = provider.get_host().await?;

        Ok(format!(
            "{0}/admin/realms/{1}/clients/{2}/roles/{3}",
            host, realm, client_uuid, role_name
        ))
    }
}
