use std::{env, sync::Arc};

use dotenvy::dotenv;
use lazy_static::lazy_static;
use reqwest::Client;
use serde_json::json;

use crate::{
    models::Book,
    user::{DbUser, NewUser, User},
};

lazy_static! {
    pub static ref BACKEND_URL: String = {
        dotenv().ok();
        env::var("BACKEND_URL").expect("BACKEND_URL must be set")
    };
}

pub async fn register_user(
    client: &Arc<Client>,
    new_user: &NewUser,
) -> Result<DbUser, Box<dyn std::error::Error>> {
    let response = client
        .post(format!("{}/users", *BACKEND_URL))
        .json(&new_user)
        .send()
        .await?;
    let response_body = response.text().await?;
    let db_user = serde_json::from_str(&response_body)?;
    Ok(db_user)
}

pub async fn login_user(
    client: &Arc<Client>,
    username: String,
    password: String,
) -> Result<User, Box<dyn std::error::Error>> {
    let credentials_json = json!(
        {
            "username": username,
            "password": password
        }
    );
    let response = client
        .post(format!("{}/users/login", *BACKEND_URL))
        .json(&credentials_json)
        .send()
        .await?;
    let response_body = response.text().await?;
    let user = serde_json::from_str(&response_body)?;
    Ok(user)
}

pub async fn create_book(
    client: &Arc<Client>,
    book: &Book,
    user_id: i32,
) -> Result<usize, Box<dyn std::error::Error>> {
    let response = client
        .post(format!("{}/users/{}/books", *BACKEND_URL, user_id))
        .json(&book)
        .send()
        .await?;
    let response_body = response.text().await?;
    let inserted_rows = serde_json::from_str(&response_body)?;
    Ok(inserted_rows)
}

pub async fn delete_book(
    client: &Arc<Client>,
    book_id: String,
) -> Result<usize, Box<dyn std::error::Error>> {
    let response = client
        .delete(format!("{}/books/{}", *BACKEND_URL, book_id))
        .send()
        .await?;
    let response_body = response.text().await?;
    let inserted_rows = serde_json::from_str(&response_body)?;
    Ok(inserted_rows)
}
