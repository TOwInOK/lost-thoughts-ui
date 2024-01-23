use reqwest::StatusCode;
use std::fmt;

use crate::api::post::Post;

#[derive(Debug, Clone)]
pub enum WindowState {
    Login,
    Register,
    AllPosts,
    Account,
    Poster(Post),
    Search,
}

impl fmt::Display for WindowState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            WindowState::Login => write!(f, "Login"),
            WindowState::Register => write!(f, "Register"),
            WindowState::AllPosts => write!(f, "AllPosts"),
            WindowState::Account => write!(f, "Account"),
            WindowState::Poster(_) => write!(f, "Poster"),
            WindowState::Search => write!(f, "Search"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    SignIn,
    Signed(Result<StatusCode, String>),
    SignUp,
    Registered(Result<StatusCode, String>),
    SwitchWindow(WindowState),
    Change(Changers),
    Find(String),
    PostAdd(Vec<Post>),
    DebugSwitch,
}
#[derive(Debug, Clone)]
pub enum Changers {
    PasswordChange(String),
    LoginChange(String),
    EmailChange(String),
    SearchChange(String),
}
