use super::services::{
    authorization_implementation::DefaultAdminTokenProvider,
    credentials_implementation::EnvAdminCredentialProvider,
    host_implementation::EnvHostAddressProvider,
    management_implementation::DefaultKeycloakManagement,
    routes_implementation::DefaultAdminRoutes, seeding_implementation::DefaultKeycloakSeeding,
    watcher_implementation::DefaultKeycloakWatcher,
};

#[inline]
pub fn create_default_manager<'a>() -> &'a DefaultKeycloakManagement<
    'a,
    DefaultAdminTokenProvider<
        'a,
        DefaultAdminRoutes<'a, EnvHostAddressProvider<'a>>,
        EnvAdminCredentialProvider<'a>,
    >,
    DefaultAdminRoutes<'a, EnvHostAddressProvider<'a>>,
> {
    let host_provider = &EnvHostAddressProvider::new("KEYCLOAK_HOST");

    let credentials_provider =
        &EnvAdminCredentialProvider::new("KEYCLOAK_ADMIN_LOGIN", "KEYCLOAK_ADMIN_PASSWORD");

    let routes = &DefaultAdminRoutes::new(host_provider);

    let auth_provider = &DefaultAdminTokenProvider::new(routes, credentials_provider);

    let keycloak_manager = &DefaultKeycloakManagement::new(auth_provider, routes);

    keycloak_manager
}

pub fn create_default_watcher<'a>() -> &'a DefaultKeycloakWatcher<
    'a,
    DefaultAdminTokenProvider<
        'a,
        DefaultAdminRoutes<'a, EnvHostAddressProvider<'a>>,
        EnvAdminCredentialProvider<'a>,
    >,
> {
    &DefaultKeycloakWatcher::new(&DefaultAdminTokenProvider::new(
        &DefaultAdminRoutes::new(&EnvHostAddressProvider::new("KEYCLOAK_HOST")),
        &EnvAdminCredentialProvider::new("KEYCLOAK_ADMIN_LOGIN", "KEYCLOAK_ADMIN_PASSWORD"),
    ))
}

pub fn create_default_seeder<'a>() -> &'a DefaultKeycloakSeeding<
    'a,
    DefaultKeycloakManagement<
        'a,
        DefaultAdminTokenProvider<
            'a,
            DefaultAdminRoutes<'a, EnvHostAddressProvider<'a>>,
            EnvAdminCredentialProvider<'a>,
        >,
        DefaultAdminRoutes<'a, EnvHostAddressProvider<'a>>,
    >,
> {
    &DefaultKeycloakSeeding::new(create_default_manager())
}
