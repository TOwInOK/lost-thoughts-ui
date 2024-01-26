use crate::api::errors::IOErrors;
use crate::api::post::Post;
use crate::api::user::User;
use std::fmt::{self, write};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WindowState {
    Login,
    Register,
    AllPosts,
    Account,
    Poster(Post),
    Search,
    None,
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
            WindowState::None => write!(f, "None"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    SignIn,
    Signed(Result<Option<User>, IOErrors>),
    SignUp,
    Registered(Result<Option<User>, IOErrors>),
    SwitchWindow(WindowState),
    Change(Changers),
    Find(String),
    PostAdd(Result<Option<Vec<Post>>, IOErrors>),
    Switcher(Switch),
    Back,
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
