use core::fmt;
use std::{collections::HashMap, sync::Arc};

use crate::{libgen_util::libgen_book_download, models::Book, libgen::mirrors::Mirror};
use inquire::{MultiSelect, Select};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tokio::task::JoinHandle;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub collection: Vec<Book>,
}

impl User {
    pub fn add_books(
        &mut self,
        books: HashMap<String, Book>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let select_items = books.values().map(|book| book).collect::<Vec<_>>();
        let selected_books =
            MultiSelect::new("Select books to add to your collection:", select_items).prompt()?;
        for book in selected_books {
            if let Some(book) = books.get(&book.id) {
                // println!("You selected: {}: {}", &book.id, book.volume_info);
                self.collection.push(book.clone());
            }
        }

        Ok(())
    }

    pub async fn download_books(
        &mut self,
        client: Arc<Client>,
        mirror_handles: Vec<JoinHandle<Result<Vec<Mirror>, String>>>,
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
