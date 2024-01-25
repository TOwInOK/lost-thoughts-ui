use crate::api::{post::Post, user::User};

use super::messages::WindowState;

#[derive(Clone, Debug)]
///Main Stru of this application
pub struct LostThoughts {
    //Arround
    pub title: String,
    pub current_window: WindowState,

    //AnyStru
    pub user: User,
    pub posts: Vec<Post>,
    pub search_result: Vec<Post>,

    //bools
    pub logged_in: bool,
    pub debbug: bool,

    //may changable
    pub search: String,
    pub password: String,
    pub password_repit: String,
}