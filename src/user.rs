use core::fmt;
use std::collections::HashMap;

use crate::models::Book;
use inquire::MultiSelect;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub collection: Vec<Book>,
}

impl User {
    pub fn select_books(
        &mut self,
        books: HashMap<String, Book>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let select_items = books.values().map(|book| book).collect::<Vec<_>>();
        let selected_books =
            MultiSelect::new("Select books to add to your collection:", select_items).prompt()?;
        for book in selected_books {
            if let Some(book) = books.get(&book.id) {
                println!("You selected: {}: {}", &book.id, book.volume_info);
                self.collection.push(book.clone());
            }
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
