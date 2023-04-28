use rubook_lib::models::{Book, Response};

use std::{collections::HashMap, env};

use dotenvy::dotenv;
use inquire::Text;

pub async fn book_search() -> Result<HashMap<String, Book>, Box<dyn std::error::Error>> {
    dotenv().ok();
    let google_api_key = env::var("GOOGLE_API_KEY").expect("GOOGLE_API_KEY not set");

    let mut books = HashMap::new();

    let book_query = Text::new("Search for a book:").prompt();

    match book_query {
        Ok(book_query) => {
            println!("Searching for: {}", book_query.trim());

            let url = format!(
                "https://www.googleapis.com/books/v1/volumes?q={}&key={}",
                book_query, google_api_key
            );

            let response_text = reqwest::get(&url).await?.text().await?;
            // println!("response json: {}", response_text);
            let response = serde_json::from_str::<Response>(&response_text);

            match response {
                Ok(response) => {
                    if let Some(error) = response.error {
                        println!("Serialization error from google api: {}", error.message);
                    } else if let Some(items) = response.items {
                        println!("SHADOW WIZARD MONEY GANG");
                        for book in items {
                            books.insert(book.id.clone(), book.clone());
                        }
                    } else {
                        println!("No items found");
                    }
                }
                Err(e) => println!("Serialization error: {}", e),
            }
        }
        Err(e) => println!("Error: {}", e),
    }

    Ok(books)
}
