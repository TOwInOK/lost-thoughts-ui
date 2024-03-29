use std::fmt::Display;

use super::role::{default_role, Role};
use serde::{Deserialize, Serialize};
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct User {
    #[serde(rename = "name", alias = "name")]
    login: String,
    password: String,
    email: String,
    #[serde(default = "default_role")]
    role: Role,
}
#[allow(dead_code)]
impl User {
    pub fn new(login: String, password: String, email: String, role: Option<Role>) -> Self {
        User {
            login,
            password,
            email,
            role: role.unwrap_or_else(default_role),
        }
    }

    pub fn get_login(&self) -> &str {
        &self.login
    }

    pub fn set_login(&mut self, value: String) {
        self.login = value;
    }

    pub fn get_password(&self) -> &str {
        &self.password
    }

    pub fn set_password(&mut self, value: String) {
        self.password = value;
    }

    pub fn get_email(&self) -> &str {
        &self.email
    }

    pub fn set_email(&mut self, value: String) {
        self.email = value;
    }

    pub fn get_role(&self) -> &Role {
        &self.role
    }

    pub fn set_role(&mut self, value: Role) {
        self.role = value;
    }

    //other

    pub fn login_is_empty(&self) -> bool {
        self.login.is_empty()
    }

    ///just push post and set self on it
    pub fn as_a_plain(&mut self, post: User) {
        self.login = post.login;
        self.password = post.password;
        self.email = post.email;
        self.role = post.role;
    }
}

impl Display for Role {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Role::Admin => write!(f, "Admin"),
            Role::Paid => write!(f, "Paid"),
            Role::Default => write!(f, "Default"),
        }
    }
}
