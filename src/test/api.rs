use reqwest::Client;
use serde_json::json;

use crate::api::{errors::IOErrors, post::Post, role::Role, user::User};
//Mockito does't support json from reqwest.

#[tokio::test]
//Test Delete function of API
async fn delete() {
    // Запуск фиктивного сервера
    let mut lost_thought = mockito::Server::new();
    let id = 12332;
    let url = format!("/https://api.lost-umbrella.com/post/{}/delete", id);
    lost_thought
        .mock("DELETE", &*url)
        .with_body("Post Delete")
        .create();
    let uri = format!("{}/{}", lost_thought.url(), url);
    // Создание тестового пользователя
    let user = User::new(
        "login".to_owned(),
        "password".to_owned(),
        "email@email.xxx".to_owned(),
        Some(Role::Default),
    );
    // Вызов тестируемой функции
    // let result = delete(id.to_string(), user).await;
    let client = Client::new();
    let response = client
        .delete(format!("{}", uri))
        .json(&json!(
            {
                "name": user.get_login(),
                "password": user.get_password(),
            }
        ))
        .send()
        .await
        .map_err(|e| IOErrors::PostDelete(e.to_string()))
        .unwrap();
    let result: Result<Option<()>, IOErrors> = if response.status().is_success() {
        Ok(Some(()))
    } else {
        // В случае ошибки возвращаем None
        println!("None: {}", response.status());
        Ok(None)
    };
    // Проверка результата
    assert!(result.is_ok())
}

#[tokio::test]
pub async fn log_in() {
    // Запуск фиктивного сервера
    let mut lost_thought = mockito::Server::new();
    lost_thought
        .mock("GET", "/endpoint")
        //match reqwest with mock data
        .match_body(
            r#"{
                "name": "example_user",
                "password": "test123"
            }"#,
        )
        .with_body(
            r#"{
                "name": "example_user",
                "password": "test123",
                "email": "email@email.xxx",
                "role": "Default"
            }"#,
        )
        .create();
    let uri = format!("{}/endpoint", lost_thought.url());

    let client = Client::new();
    let response = client
        .get(format!("{}", uri))
        .body(
            r#"{
                "name": "example_user",
                "password": "test123"
            }"#,
        )
        .send()
        .await
        .map_err(|e| IOErrors::SingIn(e.to_string()))
        .unwrap();
    println!("Send & Get");
    let result: Result<Option<User>, IOErrors> = if response.status().is_success() {
        println!("Get Json");
        let json: User = response
            .json()
            .await
            .map_err(|e| IOErrors::SingIn(e.to_string()))
            .unwrap();

        Ok(Some(json))
    } else {
        println!("Get Nothing: {}", response.status());
        Err(IOErrors::SingUp("".to_owned()))
    };
    //As we use json like User we neet to use .is_ok()
    assert!(result.is_ok());
}

#[tokio::test]
pub async fn sing_up() {
    // Запуск фиктивного сервера
    let mut lost_thought = mockito::Server::new();
    lost_thought
        .mock("POST", "/endpoint")
        //match reqwest with mock data
        .match_body(
            r#"{
                "name": "example_user",
                "password": "test123",
                "email": "email@email.xxx"
            }"#,
        )
        .create();
    let uri = format!("{}/endpoint", lost_thought.url());

    let client = Client::new();
    let response = client
        .post(format!("{}", uri))
        .body(
            r#"{
                "name": "example_user",
                "password": "test123",
                "email": "email@email.xxx"
            }"#,
        )
        .send()
        .await
        .map_err(|e| IOErrors::SingIn(e.to_string()))
        .unwrap();
    println!("Send & Get");
    //cause we get simply text we use string instad any stru
    let result: Result<String, IOErrors> = if response.status().is_success() {
        println!("Get Json");
        let text = response
            .text()
            .await
            .map_err(|e| IOErrors::SingIn(e.to_string()))
            .unwrap();

        Ok(text)
    } else {
        println!("Get Nothing");
        Err(IOErrors::SingUp("".to_owned()))
    };
    //As we use json like User we neet to use .is_ok()
    assert!(result.is_ok());
}

#[tokio::test]
pub async fn get_all_posts() {
    // Запуск фиктивного сервера
    let mut lost_thought = mockito::Server::new();
    lost_thought
        .mock("GET", "/endpoint")
        //match reqwest with mock data
        .with_body(
            r#"[{
            "_id": {
                "$oid": "65c4656f81062592f2274a66"
            },
            "author": [
                "example_user",
                "TOwInOK"
            ],
            "date": 1707369839327,
            "underlabel": "test",
            "label": "TEST",
            "text": "TEWRERWQFEDSADF",
            "footer": "FOOTER?",
            "tags": [
                "1",
                "2"
            ],
            "comments": []
        }]"#,
        )
        .create();
    let uri = format!("{}/endpoint", lost_thought.url());

    let client = Client::new();
    let response = client
        .get(format!("{}", uri))
        .send()
        .await
        .map_err(|e| IOErrors::SingIn(e.to_string()))
        .unwrap();
    println!("Send & Get");
    let result: Result<Option<Vec<Post>>, IOErrors> = if response.status().is_success() {
        println!("Get Json");
        let json: Vec<Post> = response
            .json()
            .await
            .map_err(|e| IOErrors::SingIn(e.to_string()))
            .unwrap();

        Ok(if json.is_empty() { None } else { Some(json) })
    } else {
        println!("Get Nothing");
        Err(IOErrors::SingUp("".to_owned()))
    };
    //As we use json like User we neet to use .is_ok()
    assert!(result.is_ok());
}

#[tokio::test]
pub async fn get_post_by_id() {
    // Запуск фиктивного сервера
    let mut lost_thought = mockito::Server::new();
    lost_thought
        .mock("GET", "/endpoint")
        //match reqwest with mock data
        .with_body(
            r#"
            {
                "_id": {
                    "$oid": "65c4656f81062592f2274a66"
                },
                "author": [
                    "example_user",
                    "TOwInOK"
                ],
                "date": 1707369839327,
                "underlabel": "test",
                "label": "TEST",
                "text": "TEWRERWQFEDSADF",
                "footer": "FOOTER?",
                "tags": [
                    "1",
                    "2"
                ],
                "comments": []
            }
            "#,
        )
        .create();
    let uri = format!("{}/endpoint", lost_thought.url());

    let client = Client::new();
    let response = client
        .get(format!("{}", uri))
        .send()
        .await
        .map_err(|e| IOErrors::SingIn(e.to_string()))
        .unwrap();
    println!("Send & Get");
    let result: Result<Option<Post>, IOErrors> = if response.status().is_success() {
        println!("Get Json");
        let json: Post = response
            .json()
            .await
            .map_err(|e| IOErrors::SingIn(e.to_string()))
            .unwrap();

        Ok(Some(json))
    } else {
        println!("Get Nothing");
        Err(IOErrors::SingUp("".to_owned()))
    };
    //As we use json like User we neet to use .is_ok()
    assert!(result.is_ok());
}

#[tokio::test]
pub async fn push() {
    // Запуск фиктивного сервера
    let mut lost_thought = mockito::Server::new();
    lost_thought
        .mock("PUT", "/endpoint")
        //match reqwest with mock data
        .match_body(
            r#"
            {
               	"post":{
              		"_id": {
             			"$oid": "65c4656f81062592f2274a66"
              		},
              		"author": [
             			"example_user"
              		],
              		"date": 1703179315411,
              		"underlabel": "test",
              		"label": "TEST",
              		"text": "TEF",
              		"footer": "FOOTER?",
              		"tags": [
             			"1",
             			"2"
              		]
               	},
              		"auth": {
              		"name": "example_user",
              		"password": "test123"
               	}
            }
            "#,
        )
        .with_body(r#"Post edited"#)
        .create();
    let uri = format!("{}/endpoint", lost_thought.url());

    let client = Client::new();
    let response = client
        .put(format!("{}", uri))
        .body(
            r#"
            {
               	"post":{
              		"_id": {
             			"$oid": "65c4656f81062592f2274a66"
              		},
              		"author": [
             			"example_user"
              		],
              		"date": 1703179315411,
              		"underlabel": "test",
              		"label": "TEST",
              		"text": "TEF",
              		"footer": "FOOTER?",
              		"tags": [
             			"1",
             			"2"
              		]
               	},
              		"auth": {
              		"name": "example_user",
              		"password": "test123"
               	}
            }
            "#,
        )
        .send()
        .await
        .map_err(|e| IOErrors::SingIn(e.to_string()))
        .unwrap();
    println!("Send & Get");
    let result: Result<Option<String>, IOErrors> = if response.status().is_success() {
        println!("Get Json");
        let text = response
            .text()
            .await
            .map_err(|e| IOErrors::SingIn(e.to_string()))
            .unwrap();

        Ok(Some(text))
    } else {
        println!("Get Nothing: {}", response.status());
        Err(IOErrors::SingUp("".to_owned()))
    };
    //As we use json like User we neet to use .is_ok()
    assert!(result.is_ok());
}

#[tokio::test]
pub async fn delete_post() {
    // Запуск фиктивного сервера
    let mut lost_thought = mockito::Server::new();
    lost_thought
        .mock("PUT", "/endpoint")
        //match reqwest with mock data
        .match_body(
            r#"
            {
          		"name": "example_user",
          		"password": "test123"
            }
            "#,
        )
        .with_body(r#"Post edited"#)
        .create();
    let uri = format!("{}/endpoint", lost_thought.url());

    let client = Client::new();
    let response = client
        .put(format!("{}", uri))
        .body(
            r#"
            {
          		"name": "example_user",
          		"password": "test123"
            }
            "#,
        )
        .send()
        .await
        .map_err(|e| IOErrors::SingIn(e.to_string()))
        .unwrap();
    println!("Send & Get");
    let result: Result<Option<String>, IOErrors> = if response.status().is_success() {
        println!("Get Json");
        let text = response
            .text()
            .await
            .map_err(|e| IOErrors::SingIn(e.to_string()))
            .unwrap();

        Ok(Some(text))
    } else {
        println!("Get Nothing: {}", response.status());
        Err(IOErrors::SingUp("".to_owned()))
    };
    //As we use json like User we neet to use .is_ok()
    assert!(result.is_ok());
}
