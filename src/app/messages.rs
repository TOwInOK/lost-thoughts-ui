use crate::api::errors::IOErrors;
use crate::api::post::Post;
use reqwest::StatusCode;
use std::fmt;

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
    Signed(Result<StatusCode, IOErrors>),
    SignUp,
    Registered(Result<StatusCode, IOErrors>),
    SwitchWindow(WindowState),
    Change(Changers),
    Find(String),
    PostAdd(Result<Option<Vec<Post>>, IOErrors>),
    Switcher(Switch),
}
#[derive(Debug, Clone)]
pub enum Changers {
    PasswordChange(String),
    LoginChange(String),
    EmailChange(String),
    SearchChange(String),
}

#[derive(Debug, Clone)]
pub enum Switch {
    DebugPanelSwitch,
    ChangePasswordSwtich,
    ChangeEmailSwitch,
}
