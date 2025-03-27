use tokio_util::sync::CancellationToken;

use crate::{
    keycloak::services::{
        queries::clients::ClientsQuery,
        requests::{
            create_client::CreateClientRequest, create_realm::CreateRealmRequest,
            create_role::CreateRoleRequest,
        },
    },
    utils::errors::AppErr,
};

use super::{
    management::KeycloakManagement,
    seeding::{KeycloakSeeding, KeycloakSeedingArguments},
};

pub struct DefaultKeycloakSeeding<'a, TManager: KeycloakManagement> {
    manager: &'a TManager,
}

impl<'a, TManager: KeycloakManagement> DefaultKeycloakSeeding<'a, TManager> {
    pub fn new(manager: &'a TManager) -> Self {
        DefaultKeycloakSeeding { manager }
    }
}

impl<'a, TManager: KeycloakManagement> KeycloakSeeding for DefaultKeycloakSeeding<'a, TManager> {
    async fn seed<'b>(&self, args: KeycloakSeedingArguments<'b>) -> Result<(), AppErr> {
        self.manager
            .create_realm(&CreateRealmRequest::new(args.realm_name))
            .await?;

        log::info!("realm created");

        self.manager
            .create_client(&CreateClientRequest::new(
                args.client_name,
                args.realm_name,
                args.client_secret,
            ))
            .await?;

        log::info!("client created");

        let clients = self
            .manager
            .query_clients(&ClientsQuery::new(args.realm_name, args.client_name))
            .await?;

        let client = clients
            .get(0)
            .map(Result::Ok)
            .unwrap_or(Result::Err(AppErr::from("cannot get client from payload")))?;

        log::info!("got client: {0}, {1}", client.id, client.client_id);

        self.manager
            .create_role(&CreateRoleRequest::new(
                args.realm_name,
                &client.id,
                args.customer_role_name,
                "",
            ))
            .await?;

        log::info!("customer role created");

        self.manager
            .create_role(&CreateRoleRequest::new(
                args.realm_name,
                &client.id,
                args.vendor_role_name,
                String::new().as_str(),
            ))
            .await?;

        log::info!("vendor role created");

        Ok(())
    }
}
