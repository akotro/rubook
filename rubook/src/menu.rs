use std::sync::Arc;

use reqwest::Client;
use rubook_lib::{
    backend_util::{delete_user, get_mirrors},
    user::{login, register, User}, libgen::mirrors::{MirrorList, Mirror},
};

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
    DeleteAccount,
    ReturnToLogin,
    Exit,
}

impl std::fmt::Display for MainMenuOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MainMenuOption::SearchForBook => write!(f, "Search for a book"),
            MainMenuOption::ViewCollection => write!(f, "View your collection"),
            MainMenuOption::DeleteBooks => write!(f, "Delete books from your collection"),
            MainMenuOption::DownloadBook => write!(f, "Download a book from your collection"),
            MainMenuOption::DeleteAccount => write!(f, "Delete your account"),
            MainMenuOption::ReturnToLogin => write!(f, "Return to login menu"),
            MainMenuOption::Exit => write!(f, "Exit"),
        }
    }
}

fn confirm(message: &str) -> bool {
    let retry = inquire::Confirm::new(message)
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

pub async fn main_loop(client: Arc<Client>) -> Result<(), Box<dyn std::error::Error>> {
    let mut exit_program = false;

    loop {
        if exit_program {
            break;
        }

        if let Some(mut user) = login_menu(&client).await {
            let mirrors = get_mirrors(&client, &user.token).await?;
            if main_menu(client.clone(), &mut user, mirrors).await? {
                exit_program = true;
            }
        } else if confirm("Do you really want to exit?") {
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
                    loop {
                        if let Some(user) = login(client).await {
                            return Some(user);
                        } else if !confirm("Try again?") {
                            break None::<User>;
                        }
                    }
                }
                LoginMenuOption::Register => {
                    loop {
                        if let Some(user) = register(client).await {
                            return Some(user);
                        } else if !confirm("Try again?") {
                            break None::<User>;
                        }
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
    mirrors: Vec<Mirror>,
) -> Result<bool, Box<dyn std::error::Error>> {
    loop {
        let mirror_list = MirrorList::new(mirrors.clone());
        let mut mirror_handles = std::sync::Arc::new(mirror_list)
            .spawn_get_working_mirrors_tasks(&client)
            .await;

        let options = vec![
            MainMenuOption::SearchForBook,
            MainMenuOption::ViewCollection,
            MainMenuOption::DownloadBook,
            MainMenuOption::DeleteBooks,
            MainMenuOption::DeleteAccount,
            MainMenuOption::ReturnToLogin,
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
                    if confirm("Do you really want to exit?") {
                        println!("Later...");
                        return Ok(true);
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
                MainMenuOption::DeleteAccount => {
                    if confirm("Do you really want to delete your account?") {
                        if let Err(e) = delete_user(&client, &user.token, &user.id).await {
                            eprintln!("Error: {}", e);
                        }
                        *user = User::default();
                        break;
                    }
                }
                MainMenuOption::ReturnToLogin => return Ok(false)
            },
            Err(e) => eprintln!("Error: {}", e),
        }
    }

    Ok(true)
}
