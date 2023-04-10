use inquire::Select;

use crate::{book_util::book_search, user::User};

#[derive(Debug)]
pub enum MenuOption {
    SearchForBook,
    ViewCollection,
    DownloadBook,
    Exit,
}

impl std::fmt::Display for MenuOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MenuOption::SearchForBook => write!(f, "Search for a book"),
            MenuOption::ViewCollection => write!(f, "View your collection"),
            MenuOption::DownloadBook => write!(f, "Download a book from your collection"),
            MenuOption::Exit => write!(f, "Exit"),
        }
    }
}

pub async fn menu(user: &mut User) -> Result<(), Box<dyn std::error::Error>> {
    loop {
        let options = vec![
            MenuOption::SearchForBook,
            MenuOption::ViewCollection,
            MenuOption::DownloadBook,
            MenuOption::Exit,
        ];
        let selection = Select::new(
            format!("Hello {}. Select an option:", user.username).as_str(),
            options,
        )
        .prompt();

        match selection {
            Ok(selection) => match selection {
                MenuOption::Exit => break,
                MenuOption::ViewCollection => println!("{}", user),
                MenuOption::SearchForBook => {
                    let books = book_search().await?;
                    user.add_books(books)?;
                }
                MenuOption::DownloadBook => {
                    user.download_books().await?;
                }
            },
            Err(e) => println!("Error: {}", e),
        }
    }

    Ok(())
}
