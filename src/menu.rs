use diesel::MysqlConnection;

use crate::user::{login, register, User};

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
    DownloadBook,
    Exit,
}

impl std::fmt::Display for MainMenuOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MainMenuOption::SearchForBook => write!(f, "Search for a book"),
            MainMenuOption::ViewCollection => write!(f, "View your collection"),
            MainMenuOption::DownloadBook => write!(f, "Download a book from your collection"),
            MainMenuOption::Exit => write!(f, "Exit"),
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

pub async fn main_loop(connection: &mut MysqlConnection) -> Result<(), Box<dyn std::error::Error>> {
    loop {
        let mut user = login_menu(connection);

        if user != User::default() {
            main_menu(connection, &mut user).await?;
            break;
        }

        if confirm_exit() {
            println!("Later...");
            break;
        }
    }

    Ok(())
}

pub fn login_menu(conn: &mut MysqlConnection) -> User {
    loop {
        let options = vec![
            LoginMenuOption::Login,
            LoginMenuOption::Register,
            LoginMenuOption::Exit,
        ];
        let selection = inquire::Select::new("Select an option:", options).prompt();

        return match selection {
            Ok(selection) => match selection {
                LoginMenuOption::Exit => break User::default(),
                LoginMenuOption::Login => login(conn),
                LoginMenuOption::Register => register(conn),
            },
            Err(_) => break User::default(),
        };
    }
}

pub async fn main_menu(conn: &mut MysqlConnection, user: &mut User) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .expect("Could not build reqwest client");
    let client = std::sync::Arc::new(client);

    loop {
        let mirrors = crate::libgen_util::parse_mirrors();
        let mut mirror_handles = std::sync::Arc::new(mirrors)
            .spawn_get_working_mirrors_tasks(&client)
            .await;

        let options = vec![
            MainMenuOption::SearchForBook,
            MainMenuOption::ViewCollection,
            MainMenuOption::DownloadBook,
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
                        if let Err(e) = user.add_books(conn, books) {
                            eprintln!("Error adding books: {}", e);
                        }
                    } else {
                        eprintln!("Error searching for books");
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
