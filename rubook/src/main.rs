use rubook_lib::db_util;

mod book_util;
mod menu;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_pool = db_util::init_database();
    let mut connection = db_util::get_connection(&db_pool);
    menu::main_loop(&mut connection).await
}
