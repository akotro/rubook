use diesel::{AsChangeset, Insertable, Queryable};
use serde::{Deserialize, Serialize};

use crate::schema::*;

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct DbUser {
    pub id: String,
    pub username: String,
    pub password: String,
}

#[derive(AsChangeset, Insertable, Serialize, Deserialize)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub id: String,
    pub username: String,
    pub password: String,
}

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct DbBook {
    pub id: String,
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = books)]
pub struct NewBook {
    pub id: String,
}

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct DbVolumeInfo {
    pub book_id: String,
    pub title: Option<String>,
    pub subtitle: Option<String>,
    pub publisher: Option<String>,
    pub published_date: Option<String>,
    pub description: Option<String>,
}

#[derive(AsChangeset, Insertable, Serialize, Deserialize)]
#[diesel(table_name = volume_infos)]
pub struct NewVolumeInfo<'a> {
    pub book_id: &'a str,
    pub title: Option<&'a str>,
    pub subtitle: Option<&'a str>,
    pub publisher: Option<&'a str>,
    pub published_date: Option<&'a str>,
    pub description: Option<&'a str>,
}

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct DbAccessInfo {
    pub book_id: String,
    pub epub_is_available: bool,
    pub pdf_is_available: bool,
}

#[derive(AsChangeset, Insertable, Serialize, Deserialize)]
#[diesel(table_name = access_infos)]
pub struct NewAccessInfo<'a> {
    pub book_id: &'a str,
    pub epub_is_available: bool,
    pub pdf_is_available: bool,
}

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct DbAuthor {
    pub id: i32,
    pub book_id: String,
    pub name: String,
}

#[derive(AsChangeset, Insertable, Serialize, Deserialize)]
#[diesel(table_name = authors)]
pub struct NewAuthor<'a> {
    pub book_id: &'a str,
    pub name: &'a str,
}

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct DbIndustryIdentifier {
    pub id: i32,
    pub book_id: String,
    pub isbn_type: String,
    pub identifier: String,
}

#[derive(AsChangeset, Insertable, Serialize, Deserialize)]
#[diesel(table_name = industry_identifiers)]
pub struct NewIndustryIdentifier<'a> {
    pub book_id: &'a str,
    pub isbn_type: &'a str,
    pub identifier: &'a str,
}

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct DbMirror {
    pub id: i32,
    pub host_url: String,
    pub search_url: Option<String>,
    pub search_url_fiction: Option<String>,
    pub download_url: Option<String>,
    pub download_url_fiction: Option<String>,
    pub download_pattern: Option<String>,
    pub sync_url: Option<String>,
    pub cover_pattern: Option<String>,
}

#[derive(AsChangeset, Insertable, Serialize, Deserialize)]
#[diesel(table_name = mirrors)]
pub struct NewMirror<'a> {
    pub host_url: &'a str,
    pub search_url: Option<&'a str>,
    pub search_url_fiction: Option<&'a str>,
    pub download_url: Option<&'a str>,
    pub download_url_fiction: Option<&'a str>,
    pub download_pattern: Option<&'a str>,
    pub sync_url: Option<&'a str>,
    pub cover_pattern: Option<&'a str>,
}

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct DbIp {
    pub id: i32,
    pub ip_address: String,
}

#[derive(AsChangeset, Insertable, Serialize, Deserialize)]
#[diesel(table_name = ip_blacklist)]
pub struct NewIp<'a> {
    pub ip_address: &'a str,
}
