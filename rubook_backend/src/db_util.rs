#![allow(dead_code)]

use diesel::result::{DatabaseErrorKind, Error};
use diesel::{prelude::*, r2d2::ConnectionManager};
use diesel_migrations::MigrationHarness;
use diesel_migrations::{embed_migrations, EmbeddedMigrations};
use r2d2::Pool;
use rubook_lib::models::{
    AccessInfo, Book, BookFormat, DbAccessInfo, DbAuthor, DbBook, DbIndustryIdentifier,
    DbVolumeInfo, IndustryIdentifier, VolumeInfo,
};
use rubook_lib::schema::{
    access_infos, authors, books, industry_identifiers, user_books, users, volume_infos,
};
use rubook_lib::user::{DbUser, NewUser, User};
use serde::{Deserialize, Serialize};
use std::{env, fmt};

use dotenvy::dotenv;

// NOTE:(akotro) Database

const LOCAL_DB: &str = "DATABASE_URL";
const SANDBOX_DB: &str = "DATABASE_URL_SANDBOX";
const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

pub type MySqlPool = Pool<ConnectionManager<MysqlConnection>>;

#[derive(Debug)]
pub enum DbError {
    UserAlreadyExists(String),
}

impl fmt::Display for DbError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DbError::UserAlreadyExists(username) => {
                write!(f, "User already exists with username: {}", username)
            }
        }
    }
}

impl std::error::Error for DbError {}

pub fn init_database() -> MySqlPool {
    dotenv().ok();
    let database_url = env::var(LOCAL_DB).expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<MysqlConnection>::new(database_url);
    let pool = Pool::builder()
        .build(manager)
        .expect("Failed to create MysqlConnection pool");

    let mut conn = get_connection(&pool);
    conn.run_pending_migrations(MIGRATIONS)
        .expect("Failed to run pending migrations");

    pool
}

pub fn get_connection(
    pool: &MySqlPool,
) -> r2d2::PooledConnection<ConnectionManager<MysqlConnection>> {
    pool.get().expect("Failed to get MysqlConnection from pool")
}

// NOTE:(akotro) Users

pub fn get_users(conn: &mut MysqlConnection) -> QueryResult<Vec<DbUser>> {
    users::table.load::<DbUser>(conn)
}

pub fn create_user(conn: &mut MysqlConnection, new_user: &NewUser) -> QueryResult<DbUser> {
    let existing_user = users::table
        .filter(users::username.eq(&new_user.username))
        .first::<DbUser>(conn)
        .optional()?;

    if let Some(_) = existing_user {
        return Err(Error::DatabaseError(
            DatabaseErrorKind::UniqueViolation,
            Box::new(format!(
                "User already exists with username: {}",
                new_user.username
            )),
        ));
    }

    diesel::insert_into(users::table)
        .values(new_user)
        .execute(conn)?;

    users::table.order(users::id.desc()).first(conn)
}

pub fn get_user_by_id(conn: &mut MysqlConnection, user_id: &str) -> QueryResult<User> {
    let db_user = users::table.find(user_id).first::<DbUser>(conn)?;

    let collection = get_books_by_user_id(conn, user_id)?;

    Ok(User {
        id: db_user.id,
        token: String::new(),
        username: db_user.username,
        password: db_user.password,
        collection,
    })
}

pub fn get_user_by_credentials(conn: &mut MysqlConnection, username: &str) -> QueryResult<User> {
    let db_user = users::table
        .filter(users::username.eq(username))
        .first::<DbUser>(conn)?;

    let collection = get_books_by_user_id(conn, &db_user.id)?;

    Ok(User {
        id: db_user.id,
        token: String::new(),
        username: db_user.username,
        password: db_user.password,
        collection,
    })
}

pub fn update_user(conn: &mut MysqlConnection, user_id: &str, user: &User) -> QueryResult<usize> {
    let updated_user = NewUser {
        id: user.id.clone(),
        username: user.username.clone(),
        password: user.password.clone(),
    };

    diesel::update(users::table.find(user_id))
        .set(&updated_user)
        .execute(conn)
}

pub fn delete_user(conn: &mut MysqlConnection, user_id: &str) -> QueryResult<usize> {
    diesel::delete(users::table.find(user_id)).execute(conn)
}

// NOTE:(akotro) Books

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = books)]
pub struct NewBook {
    pub id: String,
}

pub fn create_book(conn: &mut MysqlConnection, book: &Book, user_id: &str) -> QueryResult<usize> {
    let new_book = NewBook {
        id: book.id.clone(),
    };

    let rows_inserted = conn.transaction::<_, diesel::result::Error, _>(|transaction_context| {
        let book_exists = books::table
            .filter(books::id.eq(&new_book.id))
            .first::<DbBook>(transaction_context)
            .optional()?
            .is_some();

        if !book_exists {
            println!("Book does not exist. Inserting...");
            diesel::insert_into(books::table)
                .values(&new_book)
                .execute(transaction_context)?;

            create_volume_info(transaction_context, &book.id, &book.volume_info)?;
            create_access_info(transaction_context, &book.id, &book.access_info)?;
            let empty_vec: Vec<String> = Vec::new();
            create_authors(
                transaction_context,
                &book.id,
                &book.volume_info.authors.as_ref().unwrap_or(&empty_vec),
            )?;
        }

        diesel::insert_into(user_books::table)
            .values((
                user_books::user_id.eq(user_id),
                user_books::book_id.eq(&new_book.id),
            ))
            .execute(transaction_context)?;

        Ok(1)
    })?;

    Ok(rows_inserted)
}

pub fn get_books_by_user_id(
    conn: &mut MysqlConnection,
    db_user_id: &str,
) -> QueryResult<Vec<Book>> {
    let db_books = books::table
        .inner_join(user_books::table.on(books::id.eq(user_books::book_id)))
        .filter(user_books::user_id.eq(db_user_id))
        .select(books::all_columns)
        .load::<DbBook>(conn)?;

    let mut collection = Vec::new();
    for db_book in db_books {
        let volume_info = get_volume_info_by_book_id(conn, &db_book.id)?;
        let access_info = get_access_info_by_book_id(conn, &db_book.id)?;
        collection.push(Book {
            id: db_book.id,
            volume_info,
            access_info,
        });
    }

    Ok(collection)
}

pub fn get_book_by_id(conn: &mut MysqlConnection, book_id: &str) -> QueryResult<Book> {
    let db_book = books::table.find(book_id).first::<DbBook>(conn)?;

    let volume_info_ = get_volume_info_by_book_id(conn, &db_book.id)?;
    let access_info_ = get_access_info_by_book_id(conn, &db_book.id)?;

    Ok(Book {
        id: db_book.id,
        volume_info: volume_info_,
        access_info: access_info_,
    })
}

pub fn delete_book(conn: &mut MysqlConnection, user_id: &str, book_id: &str) -> QueryResult<usize> {
    diesel::delete(
        user_books::table
            .filter(user_books::user_id.eq(user_id))
            .filter(user_books::book_id.eq(book_id)),
    )
    .execute(conn)
}

// NOTE:(akotro) Book Volume Infos

#[derive(AsChangeset, Insertable, Serialize, Deserialize)]
#[diesel(table_name = volume_infos)]
struct NewVolumeInfo<'a> {
    book_id: &'a str,
    title: Option<&'a str>,
    subtitle: Option<&'a str>,
    publisher: Option<&'a str>,
    published_date: Option<&'a str>,
    description: Option<&'a str>,
}

pub fn create_volume_info(
    conn: &mut MysqlConnection,
    book_id: &str,
    volume_info: &VolumeInfo,
) -> QueryResult<usize> {
    let new_volume_info = NewVolumeInfo {
        book_id,
        title: volume_info.title.as_deref(),
        subtitle: volume_info.subtitle.as_deref(),
        publisher: volume_info.publisher.as_deref(),
        published_date: volume_info.published_date.as_deref(),
        description: volume_info.description.as_deref(),
    };

    diesel::insert_into(volume_infos::table)
        .values(&new_volume_info)
        .execute(conn)
}

pub fn get_volume_info_by_book_id(
    conn: &mut MysqlConnection,
    book_id: &str,
) -> QueryResult<VolumeInfo> {
    let db_volume_info = volume_infos::table
        .filter(volume_infos::book_id.eq(book_id))
        .first::<DbVolumeInfo>(conn)?;

    let db_authors = authors::table
        .filter(authors::book_id.eq(book_id))
        .load::<DbAuthor>(conn)?;

    let authors: Vec<String> = db_authors
        .into_iter()
        .map(|db_author| db_author.name)
        .collect();

    let db_industry_identifiers = industry_identifiers::table
        .filter(industry_identifiers::book_id.eq(book_id))
        .load::<DbIndustryIdentifier>(conn)?;

    let industry_identifiers: Vec<IndustryIdentifier> = db_industry_identifiers
        .into_iter()
        .map(|db_industry_identifier| IndustryIdentifier {
            // id: db_industry_identifier.id,
            isbn_type: db_industry_identifier.isbn_type,
            identifier: db_industry_identifier.identifier,
        })
        .collect();

    Ok(VolumeInfo {
        title: db_volume_info.title,
        subtitle: db_volume_info.subtitle,
        publisher: db_volume_info.publisher,
        published_date: db_volume_info.published_date,
        description: db_volume_info.description,
        authors: Some(authors),
        industry_identifiers: Some(industry_identifiers),
    })
}

pub fn update_volume_info(
    conn: &mut MysqlConnection,
    book_id: &str,
    volume_info: &VolumeInfo,
) -> QueryResult<usize> {
    let updated_volume_info = NewVolumeInfo {
        book_id,
        title: volume_info.title.as_deref(),
        subtitle: volume_info.subtitle.as_deref(),
        publisher: volume_info.publisher.as_deref(),
        published_date: volume_info.published_date.as_deref(),
        description: volume_info.description.as_deref(),
    };

    diesel::update(volume_infos::table.filter(volume_infos::book_id.eq(book_id)))
        .set(&updated_volume_info)
        .execute(conn)
}

pub fn delete_volume_info(conn: &mut MysqlConnection, book_id: &str) -> QueryResult<usize> {
    diesel::delete(volume_infos::table.filter(volume_infos::book_id.eq(book_id))).execute(conn)
}

// NOTE:(akotro) Book Access Infos

#[derive(AsChangeset, Insertable, Serialize, Deserialize)]
#[diesel(table_name = access_infos)]
struct NewAccessInfo<'a> {
    book_id: &'a str,
    epub_is_available: bool,
    pdf_is_available: bool,
}

pub fn create_access_info(
    conn: &mut MysqlConnection,
    book_id: &str,
    access_info: &AccessInfo,
) -> QueryResult<usize> {
    let new_access_info = NewAccessInfo {
        book_id,
        epub_is_available: access_info.epub.is_available,
        pdf_is_available: access_info.pdf.is_available,
    };

    diesel::insert_into(access_infos::table)
        .values(&new_access_info)
        .execute(conn)
}

pub fn get_access_info_by_book_id(
    conn: &mut MysqlConnection,
    book_id: &str,
) -> QueryResult<AccessInfo> {
    let db_access_info = access_infos::table
        .filter(access_infos::book_id.eq(book_id))
        .first::<DbAccessInfo>(conn)?;

    Ok(AccessInfo {
        epub: BookFormat {
            is_available: db_access_info.epub_is_available,
        },
        pdf: BookFormat {
            is_available: db_access_info.pdf_is_available,
        },
    })
}

pub fn update_access_info(
    conn: &mut MysqlConnection,
    book_id: &str,
    access_info: &AccessInfo,
) -> QueryResult<usize> {
    let updated_access_info = NewAccessInfo {
        book_id,
        epub_is_available: access_info.epub.is_available,
        pdf_is_available: access_info.pdf.is_available,
    };

    diesel::update(access_infos::table.filter(access_infos::book_id.eq(book_id)))
        .set(&updated_access_info)
        .execute(conn)
}

pub fn delete_access_info(conn: &mut MysqlConnection, book_id: &str) -> QueryResult<usize> {
    diesel::delete(access_infos::table.filter(access_infos::book_id.eq(book_id))).execute(conn)
}

// NOTE:(akotro) Authors

#[derive(AsChangeset, Insertable, Serialize, Deserialize)]
#[diesel(table_name = authors)]
struct NewAuthor<'a> {
    book_id: &'a str,
    name: &'a str,
}

pub fn create_authors(
    conn: &mut MysqlConnection,
    book_id: &str,
    authors: &Vec<String>,
) -> QueryResult<usize> {
    let new_authors: Vec<NewAuthor> = authors
        .iter()
        .map(|name| NewAuthor { book_id, name })
        .collect();

    diesel::insert_into(authors::table)
        .values(&new_authors)
        .execute(conn)
}

fn get_authors_by_book_id(conn: &mut MysqlConnection, book_id: &str) -> QueryResult<Vec<DbAuthor>> {
    authors::table
        .filter(authors::book_id.eq(book_id))
        .load::<DbAuthor>(conn)
}

fn update_author(
    conn: &mut MysqlConnection,
    id: i32,
    book_id: &str,
    name: &str,
) -> QueryResult<usize> {
    diesel::update(authors::table.find(id))
        .set((authors::book_id.eq(book_id), authors::name.eq(name)))
        .execute(conn)
}

fn delete_authors(conn: &mut MysqlConnection, book: &Book) -> QueryResult<usize> {
    diesel::delete(authors::table.filter(authors::book_id.eq(&book.id))).execute(conn)
}
