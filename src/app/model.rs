use crate::api::{
    post::{Id, NewPost, Post},
    user::User,
};

use super::messages::WindowState;

#[derive(Clone, Debug)]
///Main Stru of this application
pub struct LostThoughts {
    //Arround
    pub title: String,
    pub current_window: WindowState,
    pub prevision_screen: WindowState,
    pub forvard_screen: WindowState,

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

    //Poster change screen
    pub local_post: LocalPost,
}
#[derive(Clone, Debug)]
pub struct LocalPost {
    id: String,
    label: String,
    under_label: String,
    date: u64,
    text: String,
    footer: String,
    tags: String,
    author: String,
}

impl LocalPost {
    fn convert_tags(tags: &String) -> Vec<String> {
        tags.split(',').map(|s| s.trim().to_string()).collect()
    }

    fn convert_author(author: &String) -> Vec<String> {
        author.split(',').map(|s| s.trim().to_string()).collect()
    }

    fn convert_id(id: &String) -> Option<Id> {
        Some(Id::new(id.clone()))
    }

    pub fn to_new_post(self) -> NewPost {
        NewPost::new(
            LocalPost::convert_id(self.get_id()),
            LocalPost::convert_author(self.get_author()),
            self.get_date(),
            self.get_under_label().clone(),
            self.get_label().clone(),
            self.get_text().clone(),
            self.get_footer().clone(),
            LocalPost::convert_tags(self.get_tags()),
        )
    }

    pub fn clear(&mut self) {
        self.id.clear();
        self.label.clear();
        self.under_label.clear();
        self.date = 0;
        self.text.clear();
        self.footer.clear();
        self.tags.clear();
        self.author.clear();
    }
    pub fn empty_new() -> Self {
        Self {
            id: String::new(),
            label: String::new(),
            under_label: String::new(),
            date: 0,
            text: String::new(),
            footer: String::new(),
            tags: String::new(),
            author: String::new(),
        }
    }
    pub fn is_empty_id(&self) -> bool {
        self.id.is_empty()
    }
}

impl LocalPost {
    // Геттеры
    pub fn get_id(&self) -> &String {
        &self.id
    }

    pub fn get_label(&self) -> &String {
        &self.label
    }

    pub fn get_under_label(&self) -> &String {
        &self.under_label
    }

    pub fn get_date(&self) -> u64 {
        self.date
    }

    pub fn get_text(&self) -> &String {
        &self.text
    }

    pub fn get_footer(&self) -> &String {
        &self.footer
    }

    pub fn get_tags(&self) -> &String {
        &self.tags
    }

    pub fn get_author(&self) -> &String {
        &self.author
    }

    // Сеттеры
    pub fn set_id(&mut self, id: String) {
        self.id = id;
    }

    pub fn set_label(&mut self, label: String) {
        self.label = label;
    }

    pub fn set_under_label(&mut self, under_label: String) {
        self.under_label = under_label;
    }

    pub fn set_date(&mut self, date: u64) {
        self.date = date;
    }

    pub fn set_text(&mut self, text: String) {
        self.text = text;
    }

    pub fn set_footer(&mut self, footer: String) {
        self.footer = footer;
    }

    pub fn set_tags(&mut self, tags: String) {
        self.tags = tags;
    }

    pub fn set_author(&mut self, author: String) {
        self.author = author;
    }
}
