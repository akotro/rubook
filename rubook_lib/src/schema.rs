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
        user_id -> Integer,
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
    users (id) {
        id -> Integer,
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
diesel::joinable!(books -> users (user_id));
diesel::joinable!(industry_identifiers -> books (book_id));
diesel::joinable!(volume_infos -> books (book_id));

diesel::allow_tables_to_appear_in_same_query!(
    access_infos,
    authors,
    books,
    industry_identifiers,
    users,
    volume_infos,
);
