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

pub async fn menu(user: &mut crate::user::User) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .expect("Could not build reqwest client");
    let client = std::sync::Arc::new(client);

    let mirrors = crate::libgen_util::parse_mirrors();
    let mut mirror_handles = std::sync::Arc::new(mirrors)
        .spawn_get_working_mirrors_tasks(&client)
        .await;

    loop {
        let options = vec![
            MenuOption::SearchForBook,
            MenuOption::ViewCollection,
            MenuOption::DownloadBook,
            MenuOption::Exit,
        ];
        let selection = inquire::Select::new(
            format!("Hello {}. Select an option:", user.username).as_str(),
            options,
        )
        .prompt();

        match selection {
            Ok(selection) => match selection {
                MenuOption::Exit => break,
                MenuOption::ViewCollection => println!("{}", user),
                MenuOption::SearchForBook => {
                    if let Ok(books) = crate::book_util::book_search().await {
                        if let Err(e) = user.add_books(books) {
                            eprintln!("Error adding books: {}", e);
                        }
                    } else {
                        eprintln!("Error searching for books");
                    }
                }
                MenuOption::DownloadBook => {
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
