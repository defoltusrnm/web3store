use std::fmt::Display;

use utils::errors::AppErr;

pub trait AdminRoutes {
    fn get_access_token_route(&self) -> impl Future<Output = Result<String, AppErr>> + Send;

    fn get_create_realm_route(&self) -> impl Future<Output = Result<String, AppErr>> + Send;

    fn get_create_client_route(
        &self,
        realm: &(impl Display + Send + Sync),
    ) -> impl Future<Output = Result<String, AppErr>> + Send;

    fn get_create_user_route(
        &self,
        realm: &(impl Display + Send + Sync),
    ) -> impl Future<Output = Result<String, AppErr>> + Send;

    fn get_users_query_route(
        &self,
        realm: &(impl Display + Send + Sync),
        username: &(impl Display + Send + Sync),
    ) -> impl Future<Output = Result<String, AppErr>> + Send;

    fn get_clients_query_route(
        &self,
        realm: &(impl Display + Send + Sync),
        client_id: &(impl Display + Send + Sync),
    ) -> impl Future<Output = Result<String, AppErr>> + Send;

    fn get_create_role_route(
        &self,
        realm: &(impl Display + Send + Sync),
        client_uuid: &(impl Display + Send + Sync),
    ) -> impl Future<Output = Result<String, AppErr>> + Send;

    fn get_role_query_route(
        &self,
        realm: &(impl Display + Send + Sync),
        client_uuid: &(impl Display + Send + Sync),
        role_name: &(impl Display + Send + Sync),
    ) -> impl Future<Output = Result<String, AppErr>> + Send;

    fn get_assign_roles_query_route(
        &self,
        realm: &(impl Display + Send + Sync),
        user_uuid: &(impl Display + Send + Sync),
        client_uuid: &(impl Display + Send + Sync),
    ) -> impl Future<Output = Result<String, AppErr>> + Send;

    fn get_update_user_route(
        &self,
        realm: &(impl Display + Send + Sync),
        user_uuid: &(impl Display + Send + Sync),
    ) -> impl Future<Output = Result<String, AppErr>> + Send;
}

pub trait Routes {
    fn get_auth_route(
        &self,
        realm: &(impl Display + Send + Sync),
    ) -> impl Future<Output = Result<String, AppErr>> + Send;
}
