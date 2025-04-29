use utils::errors::AppErr;

pub struct KeycloakSeedingArguments {
    pub realm_name: String,
    pub client_name: String,
    pub client_secret: String,
    pub customer_role_name: String,
    pub vendor_role_name: String,
}

impl KeycloakSeedingArguments {
    pub fn new(
        realm_name: &str,
        client_name: &str,
        client_secret: &str,
        customer_role_name: &str,
        vendor_role_name: &str,
    ) -> Self {
        KeycloakSeedingArguments {
            realm_name: realm_name.to_string(),
            client_name: client_name.to_string(),
            client_secret: client_secret.to_string(),
            customer_role_name: customer_role_name.to_string(),
            vendor_role_name: vendor_role_name.to_string(),
        }
    }
}

pub trait KeycloakSeeding {
    fn seed(
        &self,
        args: KeycloakSeedingArguments,
    ) -> impl Future<Output = Result<(), AppErr>> + Send;
}
