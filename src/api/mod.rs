pub mod comment;
pub mod post;
pub mod role;
pub mod user;
pub mod usermin;
use post::*;
use reqwest::{Client, StatusCode};
use serde_json::json;
use user::*;

pub async fn log_in(user: User) -> Result<StatusCode, String> {
    let client = Client::new();
    client
        .get(format!(
            "https://api.lost-umbrella.com/user/{}/settings",
            user.get_login()
        ))
        .json(&json!({
            "name": user.get_login(),
            "password": user.get_password(),
        }))
        .send()
        .await
        .map(|e| {
            println!("{:#?}", &e);
            e.status()
        })
        .map_err(|e| e.to_string())
}

pub async fn sign_up(user: &User) -> Result<StatusCode, String> {
    let client = Client::new();
    client
        .post("https://api.lost-umbrella.com/user/create".to_string())
        .json(&json!({
            "name": user.get_login(),
            "password": user.get_password(),
            "email": user.get_email()
        }))
        .send()
        .await
        .map(|e| {
            println!("{:#?}", &e);
            e.status()
        })
        .map_err(|e| e.to_string())
}

pub async fn get_all_posts() -> Result<Option<Vec<Post>>, String> {
    let client = Client::new();
    let response = client
        .get(format!("https://api.lost-umbrella.com/posts/page/all"))
        .send()
        .await
        .map_err(|e| e.to_string())?;
    let results = response
        .json::<Vec<Post>>()
        .await
        .map_err(|e| e.to_string())?;

    // Если результаты пусты, вернем None, иначе Some(results)
    Ok(if results.is_empty() {
        None
    } else {
        Some(results)
    })
}

pub async fn get_page(page: u32) -> Result<Vec<Post>, String> {
    todo!()
}

pub async fn search(user: User) -> Result<StatusCode, String> {
    todo!()
}
