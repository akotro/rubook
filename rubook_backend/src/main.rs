mod auth;
mod db_util;
mod db_models;
mod schema;

use std::env;

use actix_web::{
    delete, get,
    middleware::Logger,
    post, put,
    web::{self, Data},
    App, HttpRequest, HttpResponse, HttpServer,
};
use auth::validate_token;
use dotenvy::dotenv;
use env_logger::Env;
use rubook_lib::{
    models::{ApiResponse, Book},
    user::User,
};
use uuid::Uuid;

use crate::{
    auth::{generate_password_hash, generate_token, validate_password},
    db_util::{
        create_book, create_user, delete_book, delete_user, get_book_by_id, get_books_by_user_id,
        get_connection, get_user_by_credentials, get_user_by_id, get_users, update_user, MySqlPool,
    }, db_models::NewUser,
};

const JWT_SECRET: &str = "JWT_SECRET";

#[post("/register")]
async fn register_user_route(
    pool: web::Data<MySqlPool>,
    req: HttpRequest,
    mut new_user: web::Json<NewUser>,
) -> HttpResponse {
    new_user.0.id = Uuid::new_v4().to_string();
    let username = new_user.0.username.clone();

    let hashed_password = match generate_password_hash(new_user.0.password.clone()) {
        Ok(password) => password,
        Err(error) => {
            return HttpResponse::InternalServerError()
                .json(ApiResponse::<()>::error(error.to_string()))
        }
    };
    new_user.0.password = hashed_password;

    let result = web::block(move || {
        let mut conn = get_connection(&pool);
        create_user(&mut conn, &new_user.0)
    })
    .await;

    match result {
        Ok(users_result) => match users_result {
            Ok(db_user) => {
                let token = generate_token(&req, username);
                HttpResponse::Created().json(ApiResponse::success(User {
                    id: db_user.id,
                    username: db_user.username,
                    password: db_user.password,
                    token: token.clone(),
                    collection: Vec::new(),
                }))
            }
            Err(error) => HttpResponse::InternalServerError()
                .json(ApiResponse::<()>::error(error.to_string())),
        },
        Err(error) => {
            HttpResponse::InternalServerError().json(ApiResponse::<()>::error(error.to_string()))
        }
    }
}

#[post("/login")]
async fn login_user_route(
    pool: web::Data<MySqlPool>,
    req: HttpRequest,
    credentials: web::Json<NewUser>,
) -> HttpResponse {
    let username = credentials.0.username.clone();
    let password = credentials.0.password.clone();

    let result = web::block(move || {
        let mut conn = get_connection(&pool);
        get_user_by_credentials(&mut conn, &credentials.0.username.clone())
    })
    .await;

    match result {
        Ok(users_result) => match users_result {
            Ok(mut user) => {
                let is_valid_password = validate_password(&user.password, &password);
                if is_valid_password {
                    let token = generate_token(&req, username);
                    user.token = token;
                    HttpResponse::Found().json(ApiResponse::success(user))
                } else {
                    HttpResponse::Unauthorized()
                        .json(ApiResponse::<()>::error("Invalid credentials".to_string()))
                }
            }
            Err(error) => HttpResponse::InternalServerError()
                .json(ApiResponse::<()>::error(error.to_string())),
        },
        Err(error) => {
            HttpResponse::InternalServerError().json(ApiResponse::<()>::error(error.to_string()))
        }
    }
}

#[get("/users")]
async fn get_users_route(pool: web::Data<MySqlPool>, req: HttpRequest) -> HttpResponse {
    if let Err(err) = validate_token(&req) {
        return err;
    }

    let result = web::block(move || {
        let mut conn = get_connection(&pool);
        get_users(&mut conn)
    })
    .await;
    match result {
        Ok(users_result) => match users_result {
            Ok(users) => HttpResponse::Ok().json(ApiResponse::success(users)),
            Err(error) => HttpResponse::InternalServerError()
                .json(ApiResponse::<()>::error(error.to_string())),
        },
        Err(error) => {
            HttpResponse::InternalServerError().json(ApiResponse::<()>::error(error.to_string()))
        }
    }
}

#[get("/users/{id}")]
async fn get_user_by_id_route(
    pool: web::Data<MySqlPool>,
    req: HttpRequest,
    id: web::Path<String>,
) -> HttpResponse {
    if let Err(err) = validate_token(&req) {
        return err;
    }

    let result = web::block(move || {
        let mut conn = get_connection(&pool);
        get_user_by_id(&mut conn, &id)
    })
    .await;
    match result {
        Ok(users_result) => match users_result {
            Ok(user) => HttpResponse::Found().json(ApiResponse::success(user)),
            Err(error) => HttpResponse::InternalServerError()
                .json(ApiResponse::<()>::error(error.to_string())),
        },
        Err(error) => {
            HttpResponse::InternalServerError().json(ApiResponse::<()>::error(error.to_string()))
        }
    }
}

#[put("/users/{id}")]
async fn update_user_route(
    pool: web::Data<MySqlPool>,
    req: HttpRequest,
    id: web::Path<String>,
    user: web::Json<User>,
) -> HttpResponse {
    if let Err(err) = validate_token(&req) {
        return err;
    }

    let result = web::block(move || {
        let mut conn = get_connection(&pool);
        update_user(&mut conn, &id, &user.0)
    })
    .await;
    match result {
        Ok(users_result) => match users_result {
            Ok(rows) => HttpResponse::Ok().json(ApiResponse::success(rows)),
            Err(error) => HttpResponse::InternalServerError()
                .json(ApiResponse::<()>::error(error.to_string())),
        },
        Err(error) => {
            HttpResponse::InternalServerError().json(ApiResponse::<()>::error(error.to_string()))
        }
    }
}

#[delete("/users/{id}")]
async fn delete_user_route(
    pool: web::Data<MySqlPool>,
    req: HttpRequest,
    id: web::Path<String>,
) -> HttpResponse {
    if let Err(err) = validate_token(&req) {
        return err;
    }

    let result = web::block(move || {
        let mut conn = get_connection(&pool);
        delete_user(&mut conn, &id)
    })
    .await;
    match result {
        Ok(users_result) => match users_result {
            Ok(rows) => HttpResponse::Ok().json(ApiResponse::success(rows)),
            Err(error) => HttpResponse::InternalServerError()
                .json(ApiResponse::<()>::error(error.to_string())),
        },
        Err(error) => {
            HttpResponse::InternalServerError().json(ApiResponse::<()>::error(error.to_string()))
        }
    }
}

#[post("/users/{user_id}/books")]
async fn create_book_route(
    pool: web::Data<MySqlPool>,
    req: HttpRequest,
    user_id: web::Path<String>,
    book: web::Json<Book>,
) -> HttpResponse {
    if let Err(err) = validate_token(&req) {
        return err;
    }

    let result = web::block(move || {
        let mut conn = get_connection(&pool);
        create_book(&mut conn, &book.0, &user_id)
    })
    .await;
    match result {
        Ok(users_result) => match users_result {
            Ok(rows) => HttpResponse::Ok().json(ApiResponse::success(rows)),
            Err(error) => HttpResponse::InternalServerError()
                .json(ApiResponse::<()>::error(error.to_string())),
        },
        Err(error) => {
            HttpResponse::InternalServerError().json(ApiResponse::<()>::error(error.to_string()))
        }
    }
}

#[get("/users/{user_id}/books")]
async fn get_books_by_user_id_route(
    pool: web::Data<MySqlPool>,
    req: HttpRequest,
    user_id: web::Path<String>,
) -> HttpResponse {
    if let Err(err) = validate_token(&req) {
        return err;
    }

    let result = web::block(move || {
        let mut conn = get_connection(&pool);
        get_books_by_user_id(&mut conn, &user_id)
    })
    .await;
    match result {
        Ok(users_result) => match users_result {
            Ok(books) => HttpResponse::Ok().json(ApiResponse::success(books)),
            Err(error) => HttpResponse::InternalServerError()
                .json(ApiResponse::<()>::error(error.to_string())),
        },
        Err(error) => {
            HttpResponse::InternalServerError().json(ApiResponse::<()>::error(error.to_string()))
        }
    }
}

#[get("/books/{id}")]
async fn get_book_by_id_route(
    pool: web::Data<MySqlPool>,
    req: HttpRequest,
    id: web::Path<String>,
) -> HttpResponse {
    if let Err(err) = validate_token(&req) {
        return err;
    }

    let result = web::block(move || {
        let mut conn = get_connection(&pool);
        get_book_by_id(&mut conn, &id)
    })
    .await;
    match result {
        Ok(users_result) => match users_result {
            Ok(book) => HttpResponse::Ok().json(ApiResponse::success(book)),
            Err(error) => HttpResponse::InternalServerError()
                .json(ApiResponse::<()>::error(error.to_string())),
        },
        Err(error) => {
            HttpResponse::InternalServerError().json(ApiResponse::<()>::error(error.to_string()))
        }
    }
}

// #[put("/users/{user_id}/books/{book_id}")]
// async fn update_book_route(
//     pool: web::Data<MySqlPool>,
//     req: HttpRequest,
//     user_id: web::Path<i32>,
//     book_id: web::Path<String>,
//     book: web::Json<Book>,
// ) -> HttpResponse {
//     if let Err(err) = validate_token(&req) {
//         return err;
//     }

//     let result = web::block(move || {
//         let mut conn = get_connection(&pool);
//         update_book(&mut conn, &book_id, &book.0, *user_id)
//     })
//     .await;
//     match result {
//         Ok(users_result) => match users_result {
//             Ok(rows) => HttpResponse::Ok().json(rows),
//             Err(error) => HttpResponse::InternalServerError().body(error.to_string()),
//         },
//         Err(error) => HttpResponse::InternalServerError().body(error.to_string()),
//     }
// }

#[delete("/users/{user_id}/books/{book_id}")]
async fn delete_book_route(
    pool: web::Data<MySqlPool>,
    req: HttpRequest,
    params: web::Path<(String, String)>,
) -> HttpResponse {
    if let Err(err) = validate_token(&req) {
        return err;
    }

    let (user_id, book_id) = params.into_inner();

    let result = web::block(move || {
        let mut conn = get_connection(&pool);
        delete_book(&mut conn, &user_id, &book_id)
    })
    .await;
    match result {
        Ok(users_result) => match users_result {
            Ok(rows) => HttpResponse::Ok().json(ApiResponse::success(rows)),
            Err(error) => HttpResponse::InternalServerError()
                .json(ApiResponse::<()>::error(error.to_string())),
        },
        Err(error) => {
            HttpResponse::InternalServerError().json(ApiResponse::<()>::error(error.to_string()))
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let secret_key = Data::new(env::var(JWT_SECRET).expect("JWT_SECRET must be set"));

    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let db_pool = db_util::init_database();

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .service(
                web::scope("rubook")
                    .app_data(Data::new(db_pool.clone()))
                    .app_data(secret_key.clone())
                    .service(get_users_route)
                    // .service(get_user_by_id_route)
                    // .service(update_user_route)
                    .service(delete_user_route)
                    .service(get_books_by_user_id_route)
                    .service(create_book_route)
                    .service(get_book_by_id_route)
                    // .service(update_book_route)
                    .service(delete_book_route)
                    .service(
                        web::scope("auth")
                            .service(register_user_route)
                            .service(login_user_route),
                    )
                    .default_service(web::route().to(|| HttpResponse::NotFound())),
            )
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
