use super::comment::Comment;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Eq)]
pub struct Id {
    #[serde(rename = "$oid", alias = "$oid")]
    pub oid: String,
}

impl Id {
    pub fn new(oid: String) -> Self {
        Self { oid }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Eq)]
pub struct Post {
    #[serde(rename = "_id", alias = "_id")]
    pub id: Option<Id>,
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
            id: Some(Id::new(id)),
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

    pub fn get_tags_to_string(&self) -> String {
        self.tags.join(", ")
    }

    pub fn get_author_to_string(&self) -> String {
        self.author.join(", ")
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
        match &self.id {
            Some(e) => &e.oid,
            None => "",
        }
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
        self.id = Some(Id::new(id));
    }

    pub fn set_footer(&mut self, footer: String) {
        self.footer = footer;
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Eq)]
///This struct can be use for create new post or change exist post
///For editing existing post we need to use id. For editing id can't be `None`.
pub struct NewPost {
    #[serde(rename = "_id", alias = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<Id>,
    pub author: Vec<String>,
    pub date: u64,
    pub underlabel: String,
    pub label: String,
    pub text: String,
    pub footer: String,
    pub tags: Vec<String>,
}

impl NewPost {
    pub fn new(
        id: Option<Id>,
        author: Vec<String>,
        date: u64,
        underlabel: String,
        label: String,
        text: String,
        footer: String,
        tags: Vec<String>,
    ) -> Self {
        Self {
            id,
            author,
            date,
            underlabel,
            label,
            text,
            footer,
            tags,
        }
    }

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

    pub fn get_date(&self) -> &u64 {
        &self.date
    }

    pub fn get_author(&self) -> &Vec<String> {
        &self.author
    }

    pub fn get_id(&self) -> &str {
        match &self.id {
            Some(e) => &e.oid,
            None => "",
        }
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

    pub fn set_date(&mut self, date: u64) {
        self.date = date;
    }

    pub fn set_author(&mut self, author: Vec<String>) {
        self.author = author;
    }

    pub fn set_id(&mut self, id: String) {
        self.id = Some(Id::new(id));
    }

    pub fn set_footer(&mut self, footer: String) {
        self.footer = footer;
    }
}
