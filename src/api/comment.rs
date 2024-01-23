use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Comment {
    pub author: String,
    pub id: String,
    pub reject: String,
    pub text: String,
}
#[allow(dead_code)]
impl Comment {
    pub fn new(author: String, id: String, reject: String, text: String) -> Self {
        Self {
            author,
            id,
            reject,
            text,
        }
    }
    //Setters
    pub fn set_author(&mut self, author: String) {
        self.author = author
    }
    pub fn set_id(&mut self, id: String) {
        self.id = id
    }
    pub fn set_reject(&mut self, reject: String) {
        self.reject = reject
    }
    pub fn set_text(&mut self, text: String) {
        self.text = text
    }
    //Getters
    pub fn get_author(&self) -> &str {
        &self.author
    }
    pub fn get_id(&self) -> &str {
        &self.id
    }
    pub fn get_reject(&self) -> &str {
        &self.reject
    }
    pub fn get_text(&self) -> &str {
        &self.text
    }
}
