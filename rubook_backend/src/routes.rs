use actix_web::{delete, get, post, put, web, HttpRequest, HttpResponse};
use rubook_lib::{
    models::{ApiResponse, Book},
    user::User,
};
use uuid::Uuid;

use crate::{
    auth::{generate_password_hash, generate_token, validate_password, validate_token, validate_ip},
    db_models::NewUser,
    db_util::{
        create_book, create_user, delete_book, delete_user, get_book_by_id, get_books_by_user_id,
        get_connection, get_mirrors, get_user_by_credentials, get_user_by_id, get_users,
        update_user, MySqlPool,
    },
};

#[post("/register")]
async fn register_user_route(
    pool: web::Data<MySqlPool>,
    req: HttpRequest,
    mut new_user: web::Json<NewUser>,
) -> HttpResponse {
    if let Err(err) = validate_ip(&req) {
        return err;
    }

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
    if let Err(err) = validate_ip(&req) {
        return err;
    }

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
    if let Err(err) = validate_ip(&req) {
        return err;
    }

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
    if let Err(err) = validate_ip(&req) {
        return err;
    }

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
    if let Err(err) = validate_ip(&req) {
        return err;
    }

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
    if let Err(err) = validate_ip(&req) {
        return err;
    }

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
    if let Err(err) = validate_ip(&req) {
        return err;
    }

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
    if let Err(err) = validate_ip(&req) {
        return err;
    }

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
    if let Err(err) = validate_ip(&req) {
        return err;
    }

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
    if let Err(err) = validate_ip(&req) {
        return err;
    }

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

#[get("/mirrors")]
async fn get_mirrors_route(pool: web::Data<MySqlPool>, req: HttpRequest) -> HttpResponse {
    if let Err(err) = validate_ip(&req) {
        return err;
    }

    if let Err(err) = validate_token(&req) {
        return err;
    }

    let result = web::block(move || {
        let mut conn = get_connection(&pool);
        get_mirrors(&mut conn)
    })
    .await;

    match result {
        Ok(mirrors_result) => match mirrors_result {
            Ok(mirrors) => HttpResponse::Ok().json(ApiResponse::success(mirrors)),
            Err(error) => HttpResponse::InternalServerError()
                .json(ApiResponse::<()>::error(error.to_string())),
        },
        Err(error) => {
            HttpResponse::InternalServerError().json(ApiResponse::<()>::error(error.to_string()))
        }
    }
}
