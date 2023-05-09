use std::sync::Arc;

use reqwest::Client;
use serde_json::json;

use crate::{
    models::{ApiResponse, Book},
    user::User,
};

// lazy_static! {
//     pub static ref BACKEND_URL: String = {
//         dotenv().ok();
//         env::var("BACKEND_URL").expect("BACKEND_URL must be set")
//     };
// }

pub static BACKEND_URL: &str = "http://64.226.108.119:8080/rubook";

pub async fn register_user(
    client: &Arc<Client>,
    username: String,
    password: String,
) -> Result<User, Box<dyn std::error::Error>> {
    let new_user_json = json!(
        {
            "id": "",
            "username": username,
            "password": password
        }
    );
    let response = client
        .post(format!("{}/auth/register", BACKEND_URL))
        .json(&new_user_json)
        .send()
        .await?;
    let response_body = response.text().await?;
    ApiResponse::<User>::from_response_body(&response_body)
}

pub async fn login_user(
    client: &Arc<Client>,
    username: String,
    password: String,
) -> Result<User, Box<dyn std::error::Error>> {
    let credentials_json = json!(
        {
            "id": "",
            "username": username,
            "password": password
        }
    );
    let response = client
        .post(format!("{}/auth/login", BACKEND_URL))
        .json(&credentials_json)
        .send()
        .await?;
    let response_body = response.text().await?;
    ApiResponse::<User>::from_response_body(&response_body)
}

pub async fn delete_user(
    client: &Arc<Client>,
    token: &str,
    user_id: &str,
) -> Result<usize, Box<dyn std::error::Error>> {
    let response = client
        .delete(format!("{}/users/{}", BACKEND_URL, user_id))
        .bearer_auth(token)
        .send()
        .await?;
    let response_body = response.text().await?;
    ApiResponse::<usize>::from_response_body(&response_body)
}

pub async fn create_book(
    client: &Arc<Client>,
    token: &str,
    book: &Book,
    user_id: &str,
) -> Result<usize, Box<dyn std::error::Error>> {
    let response = client
        .post(format!("{}/users/{}/books", BACKEND_URL, user_id))
        .bearer_auth(token)
        .json(&book)
        .send()
        .await?;
    let response_body = response.text().await?;
    ApiResponse::<usize>::from_response_body(&response_body)
}

pub async fn delete_book(
    client: &Arc<Client>,
    token: &str,
    user_id: &str,
    book_id: String,
) -> Result<usize, Box<dyn std::error::Error>> {
    let response = client
        .delete(format!(
            "{}/users/{}/books/{}",
            BACKEND_URL, user_id, book_id
        ))
        .bearer_auth(token)
        .send()
        .await?;
    let response_body = response.text().await?;
    ApiResponse::<usize>::from_response_body(&response_body)
}
