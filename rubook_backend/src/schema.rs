// @generated automatically by Diesel CLI.

diesel::table! {
    access_infos (book_id) {
        book_id -> Varchar,
        epub_is_available -> Bool,
        pdf_is_available -> Bool,
    }
}

diesel::table! {
    authors (id) {
        id -> Integer,
        book_id -> Varchar,
        name -> Text,
    }
}

diesel::table! {
    books (id) {
        id -> Varchar,
    }
}

diesel::table! {
    industry_identifiers (id) {
        id -> Integer,
        book_id -> Varchar,
        isbn_type -> Text,
        identifier -> Text,
    }
}

diesel::table! {
    ip_blacklist (id) {
        id -> Integer,
        ip_address -> Varchar,
    }
}

diesel::table! {
    mirrors (id) {
        id -> Integer,
        host_url -> Text,
        search_url -> Nullable<Text>,
        search_url_fiction -> Nullable<Text>,
        download_url -> Nullable<Text>,
        download_url_fiction -> Nullable<Text>,
        download_pattern -> Nullable<Text>,
        sync_url -> Nullable<Text>,
        cover_pattern -> Nullable<Text>,
    }
}

diesel::table! {
    user_books (id) {
        id -> Integer,
        user_id -> Char,
        book_id -> Varchar,
    }
}

diesel::table! {
    users (id) {
        id -> Char,
        username -> Text,
        password -> Text,
    }
}

diesel::table! {
    volume_infos (book_id) {
        book_id -> Varchar,
        title -> Nullable<Text>,
        subtitle -> Nullable<Text>,
        publisher -> Nullable<Text>,
        published_date -> Nullable<Text>,
        description -> Nullable<Text>,
    }
}

diesel::joinable!(access_infos -> books (book_id));
diesel::joinable!(authors -> books (book_id));
diesel::joinable!(industry_identifiers -> books (book_id));
diesel::joinable!(user_books -> books (book_id));
diesel::joinable!(user_books -> users (user_id));
diesel::joinable!(volume_infos -> books (book_id));

diesel::allow_tables_to_appear_in_same_query!(
    access_infos,
    authors,
    books,
    industry_identifiers,
    ip_blacklist,
    mirrors,
    user_books,
    users,
    volume_infos,
);
