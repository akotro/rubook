use core::fmt;
use std::{collections::HashMap, sync::Arc};

use diesel::{MysqlConnection, Queryable};
use inquire::{min_length, MultiSelect, Password, PasswordDisplayMode, Select, Text};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tokio::task::JoinHandle;

use crate::{
    db_util::{self, create_user, get_user_by_credentials, NewUser},
    libgen::mirrors::Mirror,
    libgen_util::libgen_book_download,
    models::Book,
};

pub fn register(conn: &mut MysqlConnection) -> Option<User> {
    let username = Text::new("Enter your username:")
        .prompt()
        .expect("Failed to get username");
    let password = Password::new("Enter your password: ")
        .with_display_mode(PasswordDisplayMode::Masked)
        .with_validator(min_length!(8))
        .prompt()
        .expect("Failed to get password");

    let new_user = NewUser {
        username: username.as_str(),
        password: password.as_str(),
    };

    if let Ok(db_user) = create_user(conn, &new_user) {
        println!("User created: {}", db_user.username);
        Some(User {
            id: db_user.id,
            username: db_user.username,
            password: db_user.password,
            collection: vec![],
        })
    } else {
        println!("Failed to create user");
        None
    }
}

pub fn login(conn: &mut MysqlConnection) -> Option<User> {
    let username = Text::new("Enter your username:")
        .prompt()
        .expect("Failed to get username");
    let password = Password::new("Enter your password: ")
        .with_display_mode(PasswordDisplayMode::Masked)
        .without_confirmation()
        .prompt()
        .expect("Failed to get password");

    if let Ok(user) = get_user_by_credentials(conn, username.as_str(), password.as_str()) {
        println!("Welcome back, {}", user.username);
        Some(user)
    } else {
        println!("Failed to login");
        None
    }
}

impl User {
    pub fn add_books(
        &mut self,
        conn: &mut MysqlConnection,
        books: HashMap<String, Book>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let select_items = books.values().map(|book| book).collect::<Vec<_>>();
        let selected_books =
            MultiSelect::new("Select books to add to your collection:", select_items).prompt()?;
        for book in selected_books {
            if let Some(book) = books.get(&book.id) {
                // println!("You selected: {}: {}", &book.id, book.volume_info);
                self.collection.push(book.clone());
                db_util::create_book(conn, &book, self.id)?;
            }
        }

        Ok(())
    }

    pub async fn download_books(
        &mut self,
        client: &Arc<Client>,
        mirror_handles: &mut Vec<JoinHandle<Result<Vec<Mirror>, String>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if !self.collection.is_empty() {
            let selected_book =
                Select::new("Select books to download:", self.collection.clone()).prompt()?;

            libgen_book_download(selected_book, client, mirror_handles).await?;
        } else {
            println!("No books in your collection to download");
        }

        Ok(())
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub collection: Vec<Book>,
}

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if !self.collection.is_empty() {
            self.collection
                .iter()
                .try_for_each(|book| write!(f, "{}\n", book))
        } else {
            write!(f, "No books in your collection yet")
        }
    }
}

#[derive(Queryable)]
pub struct DbUser {
    pub id: i32,
    pub username: String,
    pub password: String,
}
