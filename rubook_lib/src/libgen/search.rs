use std::collections::HashSet;

use crate::{libgen::mirrors::Mirror, libgen::models::LibgenBook, models::Book};
use bytes::Bytes;
use lazy_static::lazy_static;
use regex::bytes::Regex;
use reqwest::{Client, Url};

lazy_static! {
    static ref HASH_REGEX: Regex = Regex::new(r"[A-Z0-9]{32}").unwrap();
    static ref JSON_QUERY: String =
        "id,title,author,filesize,extension,md5,year,language,pages,publisher,edition,coverurl"
            .to_string();
}

pub enum SearchType {
    NonFiction,
    Fiction,
}

impl std::fmt::Display for SearchType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SearchType::NonFiction => write!(f, "Non Fiction"),
            SearchType::Fiction => write!(f, "Fiction"),
        }
    }
}

pub async fn search_non_fiction(
    book: &Book,
    mirror: &Mirror,
    client: &Client,
) -> Result<Vec<LibgenBook>, &'static str> {
    let mut search_url = Url::parse(
        mirror
            .search_url
            .as_ref()
            .expect("Mirror search url is invalid")
            .as_str(),
    )
    .unwrap();
    let mut search_query = search_url.query_pairs_mut();

    search_query
        .append_pair(
            "req",
            format!(
                "{} {}",
                book.volume_info.clone().title.unwrap_or(String::from("")),
                book.volume_info
                    .authors
                    .clone()
                    .unwrap_or(vec![])
                    .join(", ")
            )
            .as_str(),
        )
        .append_pair("lg_topic", "libgen")
        .append_pair("res", "25")
        .append_pair("open", "0")
        .append_pair("view", "simple")
        .append_pair("phrase", "1");
    let search_url = search_query.finish();

    let content = match get_content(search_url, client).await {
        Ok(b) => b,
        Err(e) => {
            println!("Error: {:?}", e);
            return Err("Failed to get content from page");
        }
    };

    let book_hashes = parse_hashes(content);
    Ok(get_books(&book_hashes, &mirror, client).await)
}

pub async fn search_fiction(
    book: &Book,
    mirror: &Mirror,
    client: &Client,
) -> Result<String, &'static str> {
    let mut search_url = Url::parse(
        mirror
            .search_url_fiction
            .as_ref()
            .expect("Mirror search url fiction is invalid")
            .as_str(),
    )
    .unwrap();
    let mut search_query = search_url.query_pairs_mut();

    search_query
        .append_pair(
            "q",
            format!(
                "{} {}",
                book.volume_info.clone().title.unwrap_or(String::from("")),
                book.volume_info
                    .authors
                    .clone()
                    .unwrap_or(vec![])
                    .join(", ")
            )
            .as_str(),
        )
        .append_pair("criteria", "")
        .append_pair("language", "English")
        .append_pair("format", "");
    let search_url = search_query.finish();

    let content = match get_content(search_url, client).await {
        Ok(b) => b,
        Err(e) => {
            println!("Error: {:?}", e);
            return Err("Failed to get content from page");
        }
    };

    let book_hashes = parse_hashes(content);
    match book_hashes.first() {
        Some(h) => Ok(h.to_owned()),
        None => Err("No hash found"),
    }
}

async fn get_content(url: &Url, client: &Client) -> Result<Bytes, reqwest::Error> {
    println!("Getting content from: {}", url.as_str());
    client.get(url.as_str()).send().await?.bytes().await
}

fn parse_hashes(content: Bytes) -> Vec<String> {
    // println!("{}", std::str::from_utf8(content.as_ref()).unwrap().to_string());
    let mut hashes: Vec<String> = Vec::new();
    for caps in HASH_REGEX.captures_iter(&content) {
        let capture = match caps.get(0) {
            Some(c) => c,
            None => continue,
        };
        hashes.push(std::str::from_utf8(capture.as_bytes()).unwrap().to_string());
    }

    let mut unique_hashes = HashSet::new();

    hashes
        .into_iter()
        .filter(|x| unique_hashes.insert(x.clone()))
        .collect()
}

async fn get_books(hashes: &[String], mirror: &Mirror, client: &Client) -> Vec<LibgenBook> {
    let mut parsed_books: Vec<LibgenBook> = Vec::new();
    let cover_url = String::from(mirror.cover_pattern.as_ref().unwrap());

    for hash in hashes.iter() {
        println!("hash: {}", hash);
        let mut search_url =
            Url::parse(mirror.sync_url.as_ref().expect("Expected an Url").as_str()).unwrap();
        search_url
            .query_pairs_mut()
            .append_pair("ids", hash)
            .append_pair("fields", &JSON_QUERY);
        let content = match get_content(&search_url, client).await {
            Ok(v) => v,
            Err(_) => continue,
        };
        let mut libgen_books: Vec<LibgenBook> =
            match serde_json::from_str(std::str::from_utf8(&content).unwrap()) {
                Ok(v) => v,
                Err(_) => {
                    println!("Couldn't parse json");
                    continue;
                }
            };
        libgen_books.retain(|b| b.language == "English");
        libgen_books.iter_mut().for_each(|b| {
            if mirror.cover_pattern.is_some() {
                b.coverurl = cover_url.replace("{cover-url}", &b.coverurl);
            }
        });
        parsed_books.append(&mut libgen_books);
    }
    parsed_books
}
