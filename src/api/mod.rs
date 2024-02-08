pub mod comment;
pub mod errors;
pub mod post;
pub mod role;
pub mod user;
pub mod usermin;
use errors::IOErrors;
use post::*;
use reqwest::Client;
use serde::Deserialize;
use serde_json::json;
use user::*;

pub async fn log_in(user: User) -> Result<Option<User>, IOErrors> {
    let client = Client::new();
    let response = client
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
        .map_err(|e| IOErrors::SingIn(e.to_string()))?;
    println!("Send & Get");
    if response.status().is_success() {
        println!("Get Json");
        let json: User = response
            .json()
            .await
            .map_err(|e| IOErrors::SingIn(e.to_string()))?;

        Ok(Some(json))
    } else {
        println!("Get Nothing");
        Ok(None)
    }
}

pub async fn sign_up(user: User) -> Result<(), IOErrors> {
    let client = Client::new();
    let response = client
        .post("https://api.lost-umbrella.com/user/create".to_string())
        .json(&json!({
            "name": user.get_login(),
            "password": user.get_password(),
            "email": user.get_email()
        }))
        .send()
        .await
        .map_err(|e| IOErrors::SingUp(e.to_string()))?;
    println!("Send & Get");
    if response.status().is_success() {
        let _text = response
            .text()
            .await
            .map_err(|e| IOErrors::SingUp(e.to_string()))?;
        println!("Get Json");
        Ok(())
    } else {
        println!("Get Nothing");
        Err(IOErrors::SingUp("".to_owned()))
    }
}

pub async fn get_all_posts() -> Result<Option<Vec<Post>>, IOErrors> {
    println!("Start medot GetAllPosts");
    let client = Client::new();
    let response: reqwest::Response = client
        .get("https://api.lost-umbrella.com/post/page/all")
        .send()
        .await
        .map_err(|e| IOErrors::PostAdd(e.to_string()))?;
    println!("Send & Get");

    // Получаем ответ сервера в виде JSON
    let json: Vec<Post> = response
        .json()
        .await
        .map_err(|e| IOErrors::PostAdd(e.to_string()))?;
    println!("Get Json");

    // Если результаты пусты, вернем None, иначе Some(results)
    Ok(if json.is_empty() { None } else { Some(json) })
}

///Get 10 post if 0 or 1
///
///1 number = 10 posts(`Some(Vec<Post>)`) in 1 reqwest
pub async fn get_page(page: u32) -> Result<Option<Vec<Post>>, IOErrors> {
    println!("Start medot GetPage");
    let client = Client::new();
    let response: reqwest::Response = client
        .get(format!("https://api.lost-umbrella.com/post/page/{}", page))
        .send()
        .await
        .map_err(|e| IOErrors::PostAdd(e.to_string()))?;
    println!("Send & Get");

    // Получаем ответ сервера в виде JSON
    let json: Vec<Post> = response
        .json()
        .await
        .map_err(|e| IOErrors::PostAdd(e.to_string()))?;
    println!("Get Json");

    // Если результаты пусты, вернем None, иначе Some(results)
    Ok(if json.is_empty() { None } else { Some(json) })
}

pub async fn search(find: String, page: u32) -> Result<Option<Vec<Post>>, IOErrors> {
    println!("Start medot GetAllPosts");
    let client = Client::new();
    let response = client
        .get(format!(
            "https://api.lost-umbrella.com/search/vague/{}/{}",
            &find, &page
        ))
        .send()
        .await
        .map_err(|e| IOErrors::PostAdd(e.to_string()))?;
    println!("{:#?}", response);
    println!("Send & Get");
    let json: Vec<Post> = response
        .json::<Vec<Post>>()
        .await
        .map_err(|e| IOErrors::PostAdd(e.to_string()))?;
    println!("Get Json");
    println!("Json: {:#?}", &json);
    // Если результаты пусты, вернем None, иначе Some(results)
    Ok(if json.is_empty() { None } else { Some(json) })
}

pub async fn get_post_by_id(post_id: String) -> Result<Option<Post>, IOErrors> {
    println!("Start method get_post_by_id");
    println!("Post id is: {:#?}", &post_id);
    let client = Client::new();
    let response = client
        .get(format!("https://api.lost-umbrella.com/post/{}", post_id))
        .send()
        .await
        .map_err(|e| IOErrors::PostAdd(e.to_string()))?;
    println!("Send & Get");

    if response.status().is_success() {
        // Получаем ответ сервера в виде JSON
        let json: Post = response
            .json()
            .await
            .map_err(|e| IOErrors::PostAdd(e.to_string()))?;
        println!("Get Json");

        // Возвращаем полученный пост
        Ok(Some(json))
    } else {
        // В случае ошибки возвращаем None
        Ok(None)
    }
}

pub async fn push(mut post: NewPost, user: User) -> Result<Option<String>, IOErrors> {
    if post.get_id().is_empty() {
        //Create new post
        post.id = None;
        let client = Client::new();
        let response = client
            .post("https://api.lost-umbrella.com/post/create")
            .json(&json!(
                {
                    "post": &post,
                    "auth":  {
                        "name": user.get_login(),
                        "password": user.get_password(),
                    }
                }
            ))
            .send()
            .await
            .map_err(|e| IOErrors::PostPush(e.to_string()))?;
        println!("Send & Get");

        if response.status().is_success() {
            println!("Success post");
            let server_response: ServerResponse = response
                .json::<ServerResponse>()
                .await
                .map_err(|e| IOErrors::PostPush(e.to_string()))?;
            Ok(Some(server_response.inserte_id.oid))
        } else {
            println!("Post Nothing: Status: {}", response.status());
            Ok(None)
        }
    } else {
        //Change exist post
        let client = Client::new();
        let response = client
            .put("https://api.lost-umbrella.com/post/edit")
            .json(&json!(
                {
                "post": &post,
                "auth":  {
                    "name": user.get_login(),
                    "password": user.get_password(),
                }
            }
            ))
            .send()
            .await
            .map_err(|e| IOErrors::PostPush(e.to_string()))?;
        println!("Send & Get");

        if response.status().is_success() {
            println!("Seccess put");
            Ok(Some(
                response
                    .text()
                    .await
                    .map(|_| post.get_id().to_owned())
                    .map_err(|e| IOErrors::PostPush(e.to_string()))?,
            ))
        } else {
            println!("Put Nothing: Status: {}", response.status());
            Ok(None)
        }
    }
}

pub async fn delete(post_id: String, user: User) -> Result<Option<()>, IOErrors> {
    println!("Start method remove");
    println!("Post id is: {:#?}", &post_id);
    let client = Client::new();
    let response = client
        .delete(format!(
            "https://api.lost-umbrella.com/post/{}/delete",
            post_id
        ))
        .json(&json!(
            {
                "name": user.get_login(),
                "password": user.get_password(),
            }
        ))
        .send()
        .await
        .map_err(|e| IOErrors::PostDelete(e.to_string()))?;
    println!("Send & Get");

    if response.status().is_success() {
        // Возвращаем
        println!("Ok: {}", response.status());
        Ok(Some(()))
    } else {
        // В случае ошибки возвращаем None
        println!("None: {}", response.status());
        Ok(None)
    }
}

#[derive(Debug, Deserialize)]
struct ServerResponse {
    #[serde(alias = "insertedId")]
    pub inserte_id: Id,
}
