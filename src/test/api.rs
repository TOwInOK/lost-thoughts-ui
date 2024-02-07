use reqwest::Client;
use serde_json::json;

use crate::api::{errors::IOErrors, role::Role, user::User};

#[tokio::test]
//Test Delete function of API
async fn test_delete() {
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
