use utils::errors::AppErr;

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
