mod db_util;

use actix_web::{
    delete, get,
    middleware::Logger,
    post, put,
    web::{self, Data},
    App, HttpResponse, HttpServer,
};
use env_logger::Env;
use rubook_lib::{
    models::Book,
    user::{NewUser, User},
};

use crate::db_util::{
    create_book, create_user, delete_book, delete_user, get_book_by_id, get_books_by_user_id,
    get_connection, get_user_by_credentials, get_user_by_id, get_users, update_book, update_user,
    MySqlPool,
};

#[get("/users")]
async fn get_users_route(pool: web::Data<MySqlPool>) -> HttpResponse {
    let mut conn = get_connection(&pool);
    let result = get_users(&mut conn);
    match result {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(error) => HttpResponse::InternalServerError().body(error.to_string()),
    }
}

#[post("/users")]
async fn create_user_route(
    pool: web::Data<MySqlPool>,
    new_user: web::Json<NewUser>,
) -> HttpResponse {
    let mut conn = get_connection(&pool);
    let result = create_user(&mut conn, &new_user.0);
    match result {
        Ok(user) => HttpResponse::Created().json(user),
        Err(error) => HttpResponse::InternalServerError().body(error.to_string()),
    }
}

#[get("/users/{id}")]
async fn get_user_by_id_route(pool: web::Data<MySqlPool>, id: web::Path<i32>) -> HttpResponse {
    let mut conn = get_connection(&pool);
    let result = get_user_by_id(&mut conn, *id);
    match result {
        Ok(user) => HttpResponse::Found().json(user),
        Err(error) => HttpResponse::InternalServerError().body(error.to_string()),
    }
}

#[post("/users/login")]
async fn get_user_by_credentials_route(
    pool: web::Data<MySqlPool>,
    credentials: web::Json<NewUser>,
) -> HttpResponse {
    let mut conn = get_connection(&pool);
    let result = get_user_by_credentials(&mut conn, &credentials.username, &credentials.password);
    match result {
        Ok(user) => HttpResponse::Found().json(user),
        Err(error) => HttpResponse::InternalServerError().body(error.to_string()),
    }
}

#[put("/users/{id}")]
async fn update_user_route(
    pool: web::Data<MySqlPool>,
    id: web::Path<i32>,
    user: web::Json<User>,
) -> HttpResponse {
    let mut conn = get_connection(&pool);
    let result = update_user(&mut conn, *id, &user.0);
    match result {
        Ok(rows) => HttpResponse::Ok().json(rows),
        Err(error) => HttpResponse::InternalServerError().body(error.to_string()),
    }
}

#[delete("/users/{id}")]
async fn delete_user_route(pool: web::Data<MySqlPool>, id: web::Path<i32>) -> HttpResponse {
    let mut conn = get_connection(&pool);
    let result = delete_user(&mut conn, *id);
    match result {
        Ok(rows) => HttpResponse::Ok().json(rows),
        Err(error) => HttpResponse::InternalServerError().body(error.to_string()),
    }
}

#[post("/users/{user_id}/books")]
async fn create_book_route(
    pool: web::Data<MySqlPool>,
    user_id: web::Path<i32>,
    book: web::Json<Book>,
) -> HttpResponse {
    let mut conn = get_connection(&pool);
    let result = create_book(&mut conn, &book.0, *user_id);
    match result {
        Ok(rows) => HttpResponse::Ok().json(rows),
        Err(error) => HttpResponse::InternalServerError().body(error.to_string()),
    }
}

#[get("/users/{user_id}/books")]
async fn get_books_by_user_id_route(
    pool: web::Data<MySqlPool>,
    user_id: web::Path<i32>,
) -> HttpResponse {
    let mut conn = get_connection(&pool);
    let result = get_books_by_user_id(&mut conn, *user_id);
    match result {
        Ok(books) => HttpResponse::Ok().json(books),
        Err(error) => HttpResponse::InternalServerError().body(error.to_string()),
    }
}

#[get("/books/{id}")]
async fn get_book_by_id_route(pool: web::Data<MySqlPool>, id: web::Path<String>) -> HttpResponse {
    let mut conn = get_connection(&pool);
    let result = get_book_by_id(&mut conn, &id);
    match result {
        Ok(book) => HttpResponse::Ok().json(book),
        Err(error) => HttpResponse::InternalServerError().body(error.to_string()),
    }
}

#[put("/users/{user_id}/books/{book_id}")]
async fn update_book_route(
    pool: web::Data<MySqlPool>,
    user_id: web::Path<i32>,
    book_id: web::Path<String>,
    book: web::Json<Book>,
) -> HttpResponse {
    let mut conn = get_connection(&pool);
    let result = update_book(&mut conn, &book_id, &book.0, *user_id);
    match result {
        Ok(rows) => HttpResponse::Ok().json(rows),
        Err(error) => HttpResponse::InternalServerError().body(error.to_string()),
    }
}

#[delete("/books/{id}")]
async fn delete_book_route(pool: web::Data<MySqlPool>, id: web::Path<String>) -> HttpResponse {
    let mut conn = get_connection(&pool);
    let result = delete_book(&mut conn, &id);
    match result {
        Ok(rows) => HttpResponse::Ok().json(rows),
        Err(error) => HttpResponse::InternalServerError().body(error.to_string()),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let db_pool = db_util::init_database();

    HttpServer::new(move || {
        App::new().service(
            web::scope("rubook")
                .wrap(Logger::default())
                .wrap(Logger::new("%a %{User-Agent}i"))
                .app_data(Data::new(db_pool.clone()))
                .service(get_users_route)
                .service(create_user_route)
                .service(get_user_by_id_route)
                .service(get_user_by_credentials_route)
                .service(update_user_route)
                .service(delete_user_route)
                .service(get_books_by_user_id_route)
                .service(create_book_route)
                .service(get_book_by_id_route)
                .service(update_book_route)
                .service(delete_book_route),
        )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
