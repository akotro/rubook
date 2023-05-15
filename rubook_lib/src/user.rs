use core::fmt;
use std::{collections::HashMap, sync::Arc};

use inquire::{min_length, MultiSelect, Password, PasswordDisplayMode, Select, Text};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tokio::task::JoinHandle;

use crate::{
    backend_util, libgen::mirrors::Mirror, libgen_util::libgen_book_download, models::Book,
};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub token: String,
    pub username: String,
    pub password: String,
    pub collection: Vec<Book>,
}

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        println!("\n{}'s collection:\n", self.username);
        if !self.collection.is_empty() {
            self.collection
                .iter()
                .try_for_each(|book| write!(f, "{}\n", book))
        } else {
            write!(f, "No books in your collection yet")
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct UserClaims {
    pub sub: String,
    pub exp: usize,
}

pub async fn register(client: &Arc<Client>) -> Option<User> {
    let username = Text::new("Enter your username:")
        .prompt()
        .expect("Failed to get username");
    let password = Password::new("Enter your password: ")
        .with_display_mode(PasswordDisplayMode::Masked)
        .with_validator(min_length!(8))
        .prompt()
        .expect("Failed to get password");

    match backend_util::register_user(client, username, password).await {
        Ok(db_user) => {
            println!("User '{}' created", db_user.username);
            Some(User {
                id: db_user.id,
                token: db_user.token,
                username: db_user.username,
                password: db_user.password,
                collection: vec![],
            })
        }
        Err(_) => {
            println!("Failed to create user");
            None
        }
    }
}

pub async fn login(client: &Arc<Client>) -> Option<User> {
    let username = Text::new("Enter your username:")
        .prompt()
        .expect("Failed to get username");
    let password = Password::new("Enter your password: ")
        .with_display_mode(PasswordDisplayMode::Masked)
        .without_confirmation()
        .prompt()
        .expect("Failed to get password");

    if let Ok(user) = backend_util::login_user(client, username, password).await {
        println!("Welcome back, {}", user.username);
        Some(user)
    } else {
        println!("Failed to login");
        None
    }
}

impl User {
    pub fn view_collection(&self) -> Result<(), Box<dyn std::error::Error>> {
        let selected_book =
            Select::new("Select books to view:", self.collection.clone()).prompt()?;
        selected_book.print_book_info()?;

        Ok(())
    }

    pub async fn add_books(
        &mut self,
        client: &Arc<Client>,
        books: HashMap<String, Book>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let select_items = books.values().map(|book| book).collect::<Vec<_>>();
        let selected_books =
            MultiSelect::new("Select books to add to your collection:", select_items).prompt()?;

        let mut create_book_futures = Vec::new();
        for book in selected_books {
            if let Some(book) = books.get(&book.id) {
                // println!("You selected: {}: {}", &book.id, book.volume_info);
                self.collection.push(book.clone());
                create_book_futures.push(backend_util::create_book(
                    client,
                    self.token.as_str(),
                    &book,
                    &self.id,
                ))
            }
        }
        let create_book_results = futures::future::join_all(create_book_futures).await;
        for result in create_book_results {
            result?;
        }

        Ok(())
    }

    pub async fn delete_books(
        &mut self,
        client: &Arc<Client>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if !self.collection.is_empty() {
            let books_to_delete = MultiSelect::new(
                "Select books to delete from your collection:",
                self.collection.clone(),
            )
            .prompt()?;

            self.collection
                .retain(|book| !books_to_delete.contains(book));

            let delete_book_futures = books_to_delete
                .iter()
                .map(|book| {
                    backend_util::delete_book(
                        client,
                        self.token.as_str(),
                        &self.id,
                        book.id.clone(),
                    )
                })
                .collect::<Vec<_>>();
            let delete_book_results = futures::future::join_all(delete_book_futures).await;
            for result in delete_book_results {
                result?;
            }
        } else {
            println!("No books in your collection to download");
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
