mod auth;
mod db_models;
mod db_util;
mod routes;
mod schema;

use std::{
    env,
    sync::{Arc, Mutex},
};

use actix_web::{
    middleware::Logger,
    web::{self, Data},
    App, HttpResponse, HttpServer,
};
use dotenvy::dotenv;
use env_logger::Env;
use openssl::ssl::{SslAcceptor, SslAcceptorBuilder, SslFiletype, SslMethod};
use routes::*;

const JWT_SECRET: &str = "JWT_SECRET";

fn configure_ssl() -> SslAcceptorBuilder {
    let (key_path, cert_path) = if cfg!(debug_assertions) {
        ("resources/key.pem", "resources/certificate.pem")
    } else {
        ("key.pem", "certificate.pem")
    };
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file(key_path, SslFiletype::PEM)
        .unwrap();
    builder.set_certificate_chain_file(cert_path).unwrap();
    builder
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let secret_key = Data::new(env::var(JWT_SECRET).expect("JWT_SECRET must be set"));

    let db_pool = db_util::init_database();
    let ssl_builder = configure_ssl();

    let ip_blacklist = Arc::new(Mutex::new(Vec::<String>::new()));
    actix_web::rt::spawn(auth::update_blacklist(db_pool.clone(), ip_blacklist.clone()));

    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::new(
                "%a \"%r\" %s %b %D \"%{Referer}i\" \"%{User-Agent}i\" %U %{r}a",
            ))
            .service(
                web::scope("rubook")
                    .app_data(Data::new(db_pool.clone()))
                    .app_data(Data::new(ip_blacklist.clone()))
                    .app_data(secret_key.clone())
                    .service(get_users_route)
                    .service(delete_user_route)
                    .service(get_books_by_user_id_route)
                    .service(create_book_route)
                    .service(get_book_by_id_route)
                    .service(delete_book_route)
                    .service(get_mirrors_route)
                    .service(
                        web::scope("auth")
                            .service(register_user_route)
                            .service(login_user_route),
                    )
                    .default_service(web::route().to(|| HttpResponse::NotFound())),
            )
    })
    .bind_openssl("0.0.0.0:9595", ssl_builder)?
    .run()
    .await
}
