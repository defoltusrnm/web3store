use crate::utils::errors::AppErr;

pub trait AdminRoutes {
    fn get_access_token_route(&self) -> impl Future<Output = Result<String, AppErr>>;

    fn get_create_realm_route(&self) -> impl Future<Output = Result<String, AppErr>>;

    fn get_create_client_route(&self, realm: &str) -> impl Future<Output = Result<String, AppErr>>;

    fn get_create_user_route(&self, realm: &str) -> impl Future<Output = Result<String, AppErr>>;

    fn get_users_query_route(
        &self,
        realm: &str,
        username: &str,
    ) -> impl Future<Output = Result<String, AppErr>>;

    fn get_clients_query_route(
        &self,
        realm: &str,
        client_id: &str,
    ) -> impl Future<Output = Result<String, AppErr>>;

    fn get_create_role_route(
        &self,
        realm: &str,
        client_uuid: &str,
    ) -> impl Future<Output = Result<String, AppErr>>;

    fn get_role_query_route(
        &self,
        realm: &str,
        client_uuid: &str,
        role_name: &str,
    ) -> impl Future<Output = Result<String, AppErr>>;

    fn get_assign_roles_query_route(
        &self,
        realm: &str,
        user_uuid: &str,
        client_uuid: &str,
    ) -> impl Future<Output = Result<String, AppErr>>;

    fn get_update_user_route(
        &self,
        realm: &str,
        user_uuid: &str,
    ) -> impl Future<Output = Result<String, AppErr>>;
}

pub trait Routes {
    fn get_auth_route(&self, realm: &str) -> impl Future<Output = Result<String, AppErr>>;
}
