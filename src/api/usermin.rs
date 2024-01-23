use super::role::{default_role, Role};
use serde::{Deserialize, Serialize};
#[derive(Clone, Serialize, Deserialize, Debug)]
struct UserMin {
    login: String,
    #[serde(default = "default_role")]
    role: Role,
}
#[allow(dead_code)]
impl UserMin {
    pub fn new(login: String, role: Role) -> Self {
        Self { login, role }
    }
}
