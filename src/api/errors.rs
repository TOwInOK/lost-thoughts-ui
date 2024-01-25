use thiserror::Error;

#[derive(Debug, Error, Clone)]
pub enum IOErrors {
    #[error("Login error: `{0}`")]
    SingIn(String),
    #[error("Register error: `{0}`")]
    SingUp(String),
    #[error("Register error: `{0}`")]
    PostAdd(String),
}
