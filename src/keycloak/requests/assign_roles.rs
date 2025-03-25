use serde::Serialize;

pub struct AssignRolesRequest {
    pub realm: String,
    pub user_uuid: String,
    pub client_uuid: String,
    pub assign_roles: Vec<AssignRoleRequest>,
}

impl AssignRolesRequest {
    pub fn new(
        realm: &str,
        user_uuid: &str,
        client_uuid: &str,
        assign_roles: &[AssignRoleRequest],
    ) -> Self {
        AssignRolesRequest {
            realm: realm.to_owned(),
            user_uuid: user_uuid.to_owned(),
            client_uuid: client_uuid.to_owned(),
            assign_roles: assign_roles.to_vec(),
        }
    }
}

#[derive(Serialize, Clone)]
pub struct AssignRoleRequest {
    pub id: String,
    pub name: String,
}

impl AssignRoleRequest {
    pub fn new(id: &str, name: &str) -> Self {
        AssignRoleRequest {
            id: id.to_owned(),
            name: name.to_owned(),
        }
    }
}
