use std::{fmt::Display, sync::Arc};

use futures::TryFutureExt;
use utils::errors::AppErr;

use super::{
    host::HostAddressProvider,
    routes::{AdminRoutes, Routes},
};

pub struct DefaultAdminRoutes<THost: HostAddressProvider> {
    provider: Arc<THost>,
}

impl<THost: HostAddressProvider> DefaultAdminRoutes<THost> {
    pub fn new(provider: Arc<THost>) -> Self {
        DefaultAdminRoutes { provider }
    }
}

impl<THost> AdminRoutes for DefaultAdminRoutes<THost>
where
    THost: HostAddressProvider + Send + Sync,
{
    async fn get_access_token_route(&self) -> Result<String, AppErr> {
        self.provider
            .get_host()
            .map_ok(|x| format!("{0}/realms/master/protocol/openid-connect/token", x))
            .await
    }

    async fn get_create_realm_route(&self) -> Result<String, AppErr> {
        self.provider
            .get_host()
            .map_ok(|x| format!("{0}/admin/realms", x))
            .await
    }

    async fn get_create_client_route(
        &self,
        realm: &(impl Display + Send + Sync),
    ) -> Result<String, AppErr> {
        let host = self.provider.get_host().await?;

        Ok(format!("{0}/admin/realms/{1}/clients", host, realm))
    }

    async fn get_create_user_route(
        &self,
        realm: &(impl Display + Send + Sync),
    ) -> Result<String, AppErr> {
        let host = self.provider.get_host().await?;

        Ok(format!("{0}/admin/realms/{1}/users", host, realm))
    }

    async fn get_users_query_route(
        &self,
        realm: &(impl Display + Send + Sync),
        username: &(impl Display + Send + Sync),
    ) -> Result<String, AppErr> {
        let host = self.provider.get_host().await?;

        Ok(format!(
            "{0}/admin/realms/{1}/users?username={2}",
            host, realm, username
        ))
    }

    async fn get_clients_query_route(
        &self,
        realm: &(impl Display + Send + Sync),
        client_id: &(impl Display + Send + Sync),
    ) -> Result<String, AppErr> {
        let host = self.provider.get_host().await?;

        Ok(format!(
            "{0}/admin/realms/{1}/clients?clientId={2}",
            host, realm, client_id
        ))
    }

    async fn get_create_role_route(
        &self,
        realm: &(impl Display + Send + Sync),
        client_uuid: &(impl Display + Send + Sync),
    ) -> Result<String, AppErr> {
        let host = self.provider.get_host().await?;

        Ok(format!(
            "{0}/admin/realms/{1}/clients/{2}/roles",
            host, realm, client_uuid
        ))
    }

    async fn get_role_query_route(
        &self,
        realm: &(impl Display + Send + Sync),
        client_uuid: &(impl Display + Send + Sync),
        role_name: &(impl Display + Send + Sync),
    ) -> Result<String, AppErr> {
        let host = self.provider.get_host().await?;

        Ok(format!(
            "{0}/admin/realms/{1}/clients/{2}/roles/{3}",
            host, realm, client_uuid, role_name
        ))
    }

    async fn get_assign_roles_query_route(
        &self,
        realm: &(impl Display + Send + Sync),
        user_uuid: &(impl Display + Send + Sync),
        client_uuid: &(impl Display + Send + Sync),
    ) -> Result<String, AppErr> {
        let host = self.provider.get_host().await?;

        Ok(format!(
            "{0}/admin/realms/{1}/users/{2}/role-mappings/clients/{3}",
            host, realm, user_uuid, client_uuid,
        ))
    }

    async fn get_update_user_route(
        &self,
        realm: &(impl Display + Send + Sync),
        user_uuid: &(impl Display + Send + Sync),
    ) -> Result<String, AppErr> {
        let host = self.provider.get_host().await?;

        Ok(format!(
            "{0}/admin/realms/{1}/users/{2}",
            host, realm, user_uuid
        ))
    }
}

pub struct DefaultRoutes<THost: HostAddressProvider> {
    provider: Arc<THost>,
}

impl<THost: HostAddressProvider> DefaultRoutes<THost> {
    pub fn new(provider: Arc<THost>) -> Self {
        DefaultRoutes { provider }
    }
}

impl<THost> Routes for DefaultRoutes<THost>
where
    THost: HostAddressProvider + Send + Sync,
{
    async fn get_auth_route(&self, realm: &impl Display) -> Result<String, AppErr> {
        let host = self.provider.get_host().await?;

        Ok(format!(
            "{host}/realms/{realm}/protocol/openid-connect/token"
        ))
    }
}
