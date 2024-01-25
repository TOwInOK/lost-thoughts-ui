use super::comment::Comment;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Id {
    #[serde(rename = "$oid", alias = "$oid")]
    pub oid: String,
}

impl Id {
    fn new(oid: String) -> Self {
        Self { oid }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Post {
    #[serde(rename = "_id", alias = "_id")]
    pub id: Id,
    pub author: Vec<String>,
    pub date: u64,
    pub underlabel: String,
    pub label: String,
    pub text: String,
    pub footer: String,
    pub tags: Vec<String>,
    pub comments: Vec<Comment>,
}
#[allow(clippy::too_many_arguments)]
impl Post {
    pub fn new(
        id: String,
        author: Vec<String>,
        date: u64,
        underlabel: String,
        label: String,
        text: String,
        footer: String,
        tags: Vec<String>,
        comments: Vec<Comment>,
    ) -> Self {
        Self {
            id: Id::new(id),
            author,
            date,
            underlabel,
            label,
            text,
            footer,
            tags,
            comments,
        }
    }

    pub fn tags(&self) -> String {
        self.tags.join(",")
    }
}
#[allow(dead_code)]
//Get & Set
impl Post {
    // Геттеры
    pub fn get_label(&self) -> &str {
        &self.label
    }

    pub fn get_underlabel(&self) -> &str {
        &self.underlabel
    }

    pub fn get_text(&self) -> &str {
        &self.text
    }

    pub fn get_tags(&self) -> &Vec<String> {
        &self.tags
    }

    pub fn get_comments(&self) -> &Vec<Comment> {
        &self.comments
    }

    pub fn get_date(&self) -> &u64 {
        &self.date
    }

    pub fn get_author(&self) -> &Vec<String> {
        &self.author
    }

    pub fn get_id(&self) -> &str {
        &self.id.oid
    }

    pub fn get_footer(&self) -> &str {
        &self.footer
    }

    // Сеттеры
    pub fn set_label(&mut self, label: String) {
        self.label = label;
    }

    pub fn set_underlabel(&mut self, underlabel: String) {
        self.underlabel = underlabel;
    }

    pub fn set_text(&mut self, text: String) {
        self.text = text;
    }

    pub fn set_tags(&mut self, tags: Vec<String>) {
        self.tags = tags;
    }

    pub fn set_comments(&mut self, comments: Vec<Comment>) {
        self.comments = comments;
    }

    pub fn set_date(&mut self, date: u64) {
        self.date = date;
    }

    pub fn set_author(&mut self, author: Vec<String>) {
        self.author = author;
    }

    pub fn set_id(&mut self, id: String) {
        self.id.oid = id;
    }

    pub fn set_footer(&mut self, footer: String) {
        self.footer = footer;
    }
}
