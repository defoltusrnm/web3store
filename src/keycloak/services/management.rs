use tokio_util::sync::CancellationToken;

use crate::utils::errors::AppErr;

use super::{
    queries::{clients::ClientsQuery, role::RoleQuery, users::UsersQuery},
    requests::{
        assign_roles::AssignRolesRequest, create_client::CreateClientRequest,
        create_realm::CreateRealmRequest, create_role::CreateRoleRequest,
        create_user::CreateUserRequest, update_users_email_request::UpdateUsersEmailRequest,
    },
    responses::{client::ClientResponse, role::RoleResponse, user::UserResponse},
};

pub trait KeycloakManagement {
    fn create_realm(
        &self,
        request: &CreateRealmRequest,
        cancellation_token: &CancellationToken,
    ) -> impl Future<Output = Result<(), AppErr>>;

    fn create_client(
        &self,
        request: &CreateClientRequest,
        cancellation_token: &CancellationToken,
    ) -> impl Future<Output = Result<(), AppErr>>;

    fn create_user(
        &self,
        request: &CreateUserRequest,
        cancellation_token: &CancellationToken,
    ) -> impl Future<Output = Result<(), AppErr>>;

    fn query_users(
        &self,
        request: &UsersQuery,
        cancellation_token: &CancellationToken,
    ) -> impl Future<Output = Result<Vec<UserResponse>, AppErr>>;

    fn query_clients(
        &self,
        request: &ClientsQuery,
        cancellation_token: &CancellationToken,
    ) -> impl Future<Output = Result<Vec<ClientResponse>, AppErr>>;

    fn create_role(
        &self,
        request: &CreateRoleRequest,
        cancellation_token: &CancellationToken,
    ) -> impl Future<Output = Result<(), AppErr>>;

    fn query_role(
        &self,
        request: &RoleQuery,
        cancellation_token: &CancellationToken,
    ) -> impl Future<Output = Result<RoleResponse, AppErr>>;

    fn assign_roles(
        &self,
        request: &AssignRolesRequest,
        cancellation_token: &CancellationToken,
    ) -> impl Future<Output = Result<(), AppErr>>;

    fn update_users_email(
        &self,
        request: &UpdateUsersEmailRequest,
        cancellation_token: &CancellationToken,
    ) -> impl Future<Output = Result<(), AppErr>>;
}
