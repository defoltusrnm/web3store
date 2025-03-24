use serde::Serialize;
use tokio_util::sync::CancellationToken;

use crate::utils::errors::AppErr;

use super::{
    authorization::AdminAccessTokenProvider, credentials::AdminCredentialProvider,
    host::HostAddressProvider, routes::AdminRoutes,
};

pub trait KeycloakManagement<TAuthorization, TRoutes, THost, TCredentials>
where
    TAuthorization: AdminAccessTokenProvider<TRoutes, THost, TCredentials>,
    TRoutes: AdminRoutes,
    THost: HostAddressProvider,
    TCredentials: AdminCredentialProvider,
{
    fn create_realm(
        request: CreateRealmRequest,
        cancellation_token: CancellationToken,
    ) -> impl Future<Output = Result<(), AppErr>>;
}

#[derive(Serialize)]
pub struct CreateRealmRequest {
    pub id: String,
    pub realm: String,
    #[serde(rename = "displayName")]
    pub display_name: String,
    pub enabled: bool,
    #[serde(rename = "sslRequired")]
    pub ssl_required: String,
    #[serde(rename = "registrationAllowed")]
    pub registration_allowed: bool,
    #[serde(rename = "loginWithEmailAllowed")]
    pub login_with_email_allowed: bool,
    #[serde(rename = "duplicateEmailsAllowed")]
    pub duplicate_emails_allowed: bool,
    #[serde(rename = "resetPasswordAllowed")]
    pub reset_password_allowed: bool,
    #[serde(rename = "editUsernameAllowed")]
    pub edit_username_allowed: bool,
    #[serde(rename = "bruteForceProtected")]
    pub brute_force_protected: bool,
}
