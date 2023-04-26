mod book_util;
mod db_util;
mod libgen;
mod libgen_util;
mod menu;
mod models;
mod schema;
mod user;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_pool = db_util::init_connection_pool();
    let mut connection = db_util::get_connection(&db_pool);
    menu::main_loop(&mut connection).await
}
