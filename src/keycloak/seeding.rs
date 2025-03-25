use tokio_util::sync::CancellationToken;

use crate::{
    keycloak::{
        queries::clients::ClientsQuery,
        requests::{
            create_client::CreateClientRequest, create_realm::CreateRealmRequest,
            create_role::CreateRoleRequest,
        },
    },
    utils::errors::AppErr,
};

use super::management::KeycloakManagement;

pub struct KeycloakSeedingArguments<'a> {
    pub realm_name: &'a str,
    pub client_name: &'a str,
    pub client_secret: &'a str,
    pub customer_role_name: &'a str,
    pub vendor_role_name: &'a str,
}

impl<'a> KeycloakSeedingArguments<'a> {
    pub fn new(
        realm_name: &'a str,
        client_name: &'a str,
        client_secret: &'a str,
        customer_role_name: &'a str,
        vendor_role_name: &'a str,
    ) -> Self {
        KeycloakSeedingArguments {
            realm_name,
            client_name,
            client_secret,
            customer_role_name,
            vendor_role_name,
        }
    }
}

pub trait KeycloakSeeding {
    fn seed<'a>(
        &self,
        args: KeycloakSeedingArguments<'a>,
    ) -> impl Future<Output = Result<(), AppErr>>;
}

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
            .create_realm(
                &CreateRealmRequest::new(args.realm_name),
                &CancellationToken::new(),
            )
            .await?;

        log::info!("realm created");

        self.manager
            .create_client(
                &CreateClientRequest::new(args.client_name, args.realm_name, args.client_secret),
                &CancellationToken::new(),
            )
            .await?;

        log::info!("client created");

        let clients = self
            .manager
            .query_clients(
                &ClientsQuery::new(args.realm_name, args.client_name),
                &CancellationToken::new(),
            )
            .await?;

        let client = clients
            .get(0)
            .map(Result::Ok)
            .unwrap_or(Result::Err(AppErr::from("cannot get client from payload")))?;

        log::info!("got client: {0}, {1}", client.id, client.client_id);

        self.manager
            .create_role(
                &CreateRoleRequest::new(args.realm_name, &client.id, args.customer_role_name, ""),
                &CancellationToken::new(),
            )
            .await?;

        log::info!("customer role created");

        self.manager
            .create_role(
                &CreateRoleRequest::new(args.realm_name, &client.id, args.vendor_role_name, ""),
                &CancellationToken::new(),
            )
            .await?;

        log::info!("vendor role created");

        Ok(())
    }
}
