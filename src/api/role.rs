use serde::{Deserialize, Serialize};

/// Get default role if role is none
pub fn default_role() -> Role {
    Role::Default
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Role {
    Admin,
    Paid,
    Default,
}
