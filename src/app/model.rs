use crate::api::{post::Post, user::User};

use super::messages::WindowState;

#[derive(Clone, Debug)]
pub struct LostThoughts {
    pub user: User,
    pub current_window: WindowState,
    pub logged_in: bool,
    pub debbug: bool,
    pub search: String,
    pub title: String,
    pub posts: Vec<Post>,
    pub search_result: Vec<Post>,
}
