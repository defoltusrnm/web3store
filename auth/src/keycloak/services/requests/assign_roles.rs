use std::fmt::Display;

use serde::Serialize;

pub struct AssignRolesRequest {
    pub realm: String,
    pub user_uuid: String,
    pub client_uuid: String,
    pub assign_roles: Vec<AssignRoleRequest>,
}

impl AssignRolesRequest {
    pub fn new(
        realm: &impl Display,
        user_uuid: &impl Display,
        client_uuid: &impl Display,
        assign_roles: &[AssignRoleRequest],
    ) -> Self {
        AssignRolesRequest {
            realm: realm.to_string(),
            user_uuid: user_uuid.to_string(),
            client_uuid: client_uuid.to_string(),
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
    pub fn new(id: &impl Display, name: &impl Display) -> Self {
        AssignRoleRequest {
            id: id.to_string(),
            name: name.to_string(),
        }
    }
}
