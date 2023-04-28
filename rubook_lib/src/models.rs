use core::fmt;

use diesel::Queryable;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Book {
    pub id: String,
    pub volume_info: VolumeInfo,
    pub access_info: AccessInfo,
}

impl fmt::Display for Book {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.volume_info.clone())
    }
}

#[derive(Queryable)]
pub struct DbBook {
    pub id: String,
    pub user_id: i32,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VolumeInfo {
    pub title: Option<String>,
    pub subtitle: Option<String>,
    pub publisher: Option<String>,
    pub published_date: Option<String>,
    pub description: Option<String>,
    pub authors: Option<Vec<String>>,
    pub industry_identifiers: Option<Vec<IndustryIdentifier>>,
}

impl fmt::Display for VolumeInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} by {:?} ({}, {})",
            self.title.as_ref().unwrap_or(&"".to_string()),
            self.authors.as_ref().unwrap_or(&vec![]),
            self.publisher.as_ref().unwrap_or(&"".to_string()),
            self.published_date.as_ref().unwrap_or(&"".to_string())
        )
    }
}

#[derive(Queryable)]
pub struct DbVolumeInfo {
    pub book_id: String,
    pub title: Option<String>,
    pub subtitle: Option<String>,
    pub publisher: Option<String>,
    pub published_date: Option<String>,
    pub description: Option<String>,
}

#[derive(Queryable)]
pub struct DbAuthor {
    pub id: i32,
    pub book_id: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IndustryIdentifier {
    #[serde(rename = "type")]
    pub isbn_type: String,
    pub identifier: String,
}

#[derive(Queryable)]
pub struct DbIndustryIdentifier {
    pub id: i32,
    pub book_id: String,
    pub isbn_type: String,
    pub identifier: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccessInfo {
    pub epub: BookFormat,
    pub pdf: BookFormat,
}

impl fmt::Display for AccessInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "epub: {}, pdf: {}",
            self.epub.is_available, self.pdf.is_available
        )
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BookFormat {
    pub is_available: bool,
}

#[derive(Queryable)]
pub struct DbAccessInfo {
    pub book_id: String,
    pub epub_is_available: bool,
    pub pdf_is_available: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Error {
    pub message: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub items: Option<Vec<Book>>,
    pub error: Option<Error>,
}
