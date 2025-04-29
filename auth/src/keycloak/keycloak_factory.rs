use std::sync::Arc;

use super::services::{
    authorization_implementation::DefaultAdminTokenProvider,
    credentials_implementation::EnvAdminCredentialProvider,
    host_implementation::EnvHostAddressProvider, management::KeycloakManagement,
    management_implementation::DefaultKeycloakManagement,
    routes_implementation::DefaultAdminRoutes,
};

pub fn create_default_manager() -> Arc<impl KeycloakManagement> {
    let host_provider = Arc::new(EnvHostAddressProvider::new("KEYCLOAK_HOST"));

    let credentials_provider = Arc::new(EnvAdminCredentialProvider::new(
        "KEYCLOAK_ADMIN_LOGIN",
        "KEYCLOAK_ADMIN_PASSWORD",
    ));

    let routes = Arc::new(DefaultAdminRoutes::new(host_provider));

    let auth_provider = Arc::new(DefaultAdminTokenProvider::new(
        routes.clone(),
        credentials_provider,
    ));

    let manager = Arc::new(DefaultKeycloakManagement::new(
        auth_provider,
        routes.clone(),
    ));

    manager
}
