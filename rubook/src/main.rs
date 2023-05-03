mod book_util;
mod menu;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .expect("Could not build reqwest client");
    let client = std::sync::Arc::new(client);
    // let db_pool = db_util::init_database();
    // let mut connection = db_util::get_connection(&db_pool);
    menu::main_loop(client).await
}
