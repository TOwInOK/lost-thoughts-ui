use super::comment::Comment;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Post {
    title: String,
    under_title: String,
    body: String,
    tag: Vec<String>,
    comments: Vec<Comment>,
}

impl Post {
    pub fn new(
        title: String,
        under_title: String,
        body: String,
        tag: Vec<String>,
        comments: Vec<Comment>,
    ) -> Self {
        Self {
            title,
            under_title,
            body,
            tag,
            comments,
        }
    }

    pub fn tag(&self) -> String {
        self.tag.join(",")
    }
}
#[allow(dead_code)]
//Get & Set
impl Post {
    // Геттеры
    pub fn get_title(&self) -> &str {
        &self.title
    }

    pub fn get_under_title(&self) -> &str {
        &self.under_title
    }

    pub fn get_body(&self) -> &str {
        &self.body
    }

    pub fn get_tags(&self) -> &Vec<String> {
        &self.tag
    }

    pub fn get_comments(&self) -> &Vec<Comment> {
        &self.comments
    }

    // Сеттеры
    pub fn set_title(&mut self, title: String) {
        self.title = title;
    }

    pub fn set_under_title(&mut self, under_title: String) {
        self.under_title = under_title;
    }

    pub fn set_body(&mut self, body: String) {
        self.body = body;
    }

    pub fn set_tags(&mut self, tags: Vec<String>) {
        self.tag = tags;
    }

    pub fn set_comments(&mut self, comments: Vec<Comment>) {
        self.comments = comments;
    }
}
