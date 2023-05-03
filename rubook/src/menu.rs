use std::sync::Arc;

use reqwest::Client;
use rubook_lib::{user::{login, register, User}, libgen_util::parse_mirrors};

#[derive(Debug)]
pub enum LoginMenuOption {
    Login,
    Register,
    Exit,
}

impl std::fmt::Display for LoginMenuOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LoginMenuOption::Login => write!(f, "Login"),
            LoginMenuOption::Register => write!(f, "Register"),
            LoginMenuOption::Exit => write!(f, "Exit"),
        }
    }
}

#[derive(Debug)]
pub enum MainMenuOption {
    SearchForBook,
    ViewCollection,
    DeleteBooks,
    DownloadBook,
    Exit,
}

impl std::fmt::Display for MainMenuOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MainMenuOption::SearchForBook => write!(f, "Search for a book"),
            MainMenuOption::ViewCollection => write!(f, "View your collection"),
            MainMenuOption::DeleteBooks => write!(f, "Delete books from your collection"),
            MainMenuOption::DownloadBook => write!(f, "Download a book from your collection"),
            MainMenuOption::Exit => write!(f, "Exit"),
        }
    }
}

fn confirm_retry() -> bool {
    let retry = inquire::Confirm::new("Try again?")
        .with_default(false)
        .prompt();
    match retry {
        Ok(true) => true,
        Ok(false) => false,
        Err(_) => {
            println!("Error, try again later");
            false
        }
    }
}

fn confirm_exit() -> bool {
    let really_exit = inquire::Confirm::new("Do you really want to exit?")
        .with_default(false)
        .prompt();
    match really_exit {
        Ok(true) => true,
        Ok(false) => false,
        Err(_) => {
            println!("Error, try again later");
            true
        }
    }
}

pub async fn main_loop(client: Arc<Client>) -> Result<(), Box<dyn std::error::Error>> {
    loop {
        if let Some(mut user) = login_menu(&client).await {
            main_menu(client, &mut user).await?;
            break;
        }

        if confirm_exit() {
            println!("Later...");
            break;
        }
    }

    Ok(())
}

pub async fn login_menu(client: &Arc<Client>) -> Option<User> {
    loop {
        let options = vec![
            LoginMenuOption::Login,
            LoginMenuOption::Register,
            LoginMenuOption::Exit,
        ];
        let selection = inquire::Select::new("Select an option:", options).prompt();

        match selection {
            Ok(selection) => match selection {
                LoginMenuOption::Exit => break None,
                LoginMenuOption::Login => {
                    if let Some(user) = login(client).await {
                        break Some(user);
                    } else if !confirm_retry() {
                        break None;
                    }
                }
                LoginMenuOption::Register => {
                    if let Some(user) = register(client).await {
                        break Some(user);
                    } else if !confirm_retry() {
                        break None;
                    }
                }
            },
            Err(_) => break None,
        };
    }
}

pub async fn main_menu(
    client: Arc<Client>,
    user: &mut User,
) -> Result<(), Box<dyn std::error::Error>> {
    loop {
        let mirrors = parse_mirrors();
        let mut mirror_handles = std::sync::Arc::new(mirrors)
            .spawn_get_working_mirrors_tasks(&client)
            .await;

        let options = vec![
            MainMenuOption::SearchForBook,
            MainMenuOption::ViewCollection,
            MainMenuOption::DownloadBook,
            MainMenuOption::DeleteBooks,
            MainMenuOption::Exit,
        ];
        let selection = inquire::Select::new(
            format!("Hello {}. Select an option:", user.username).as_str(),
            options,
        )
        .prompt();

        match selection {
            Ok(selection) => match selection {
                MainMenuOption::Exit => {
                    if confirm_exit() {
                        println!("Later...");
                        break;
                    }
                }
                MainMenuOption::ViewCollection => println!("{}", user),
                MainMenuOption::SearchForBook => {
                    if let Ok(books) = crate::book_util::book_search().await {
                        if let Err(e) = user.add_books(&client, books).await {
                            eprintln!("Error adding books: {}", e);
                        }
                    } else {
                        eprintln!("Error searching for books");
                    }
                }
                MainMenuOption::DeleteBooks => {
                    if let Err(e) = user.delete_books(&client).await {
                        eprintln!("Error deleting books: {}", e);
                    }
                }
                MainMenuOption::DownloadBook => {
                    if let Err(e) = user.download_books(&client, &mut mirror_handles).await {
                        eprintln!("Error downloading books: {}", e);
                    }
                }
            },
            Err(e) => eprintln!("Error: {}", e),
        }
    }

    Ok(())
}
