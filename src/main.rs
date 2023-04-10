mod book_util;
mod menu;
mod models;
mod user;
mod libgen;
mod libgen_util;

use inquire::Text;

use user::User;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let username = Text::new("Enter your username:").prompt();

    let mut user = User {
        id: 1,
        username: username.unwrap(),
        collection: vec![],
    };

    menu::menu(&mut user).await?;

    Ok(())
}
