use bytes::Bytes;
use regex::bytes::Regex;
use reqwest::{Url, Client};
use lazy_static::lazy_static;

use super::{mirrors::Mirror, models::LibgenBook};

lazy_static! {
    static ref KEY_REGEX: Regex = Regex::new(r"get\.php\?md5=\w{32}&key=\w{16}").unwrap();
    static ref KEY_REGEX_LOL: Regex =
        Regex::new(r"http://62\.182\.86\.140/main/\d{7}/\w{32}/.+?(gz|pdf|rar|djvu|epub|chm)")
            .unwrap();
    static ref KEY_REGEX_LOL_CLOUDFLARE: Regex = Regex::new(
        r"https://cloudflare-ipfs\.com/ipfs/\w{62}\?filename=.+?(gz|pdf|rar|djvu|epub|chm)"
    )
    .unwrap();
    static ref KEY_REGEX_LOL_IPFS: Regex =
        Regex::new(r"https://ipfs\.io/ipfs/\w{62}\?filename=.+?(gz|pdf|rar|djvu|epub|chm)")
            .unwrap();
}

pub async fn download_book(
    client: &Client,
    mirror: &Mirror,
    book: &LibgenBook,
) -> Result<reqwest::Response, &'static str> {
    let download_page_url_md5 =
        mirror
        .download_pattern
        .as_ref()
        .unwrap()
        .replace("{md5}", &book.md5);
    let download_page_url = Url::parse(&download_page_url_md5).unwrap();

    let content = client
        .get(download_page_url)
        .send()
        .await
        .or(Err("Couldn't connect to mirror"))?
        .bytes()
        .await
        .or(Err("Couldn't get mirror page"))?;

    match mirror.host_url.as_str() {
        "https://libgen.rocks/" => match download_book_from_ads(&content, mirror, client).await {
            Ok(b) => Ok(b),
            Err(_e) => Err("Download error"),
        },
        "http://libgen.lc/" => match download_book_from_ads(&content, mirror, client).await {
            Ok(b) => Ok(b),
            Err(_e) => Err("Download error"),
        },
        "http://libgen.lol/" => match download_book_from_lol(&content, mirror, client).await {
            Ok(b) => Ok(b),
            Err(_e) => Err("Download error"),
        },
        "http://libgen.me/" => match download_book_from_lol(&content, mirror, client).await {
            Ok(b) => Ok(b),
            Err(_e) => Err("Download error"),
        },
        &_ => Err("Couldn't find download url"),
    }
}

pub async fn download_book_fiction(
    client: &Client,
    mirror: &Mirror,
    md5: &String,
) -> Result<reqwest::Response, &'static str> {
    let download_page_url_md5 =
        mirror
        .download_pattern
        .as_ref()
        .unwrap()
        .replace("{md5}", &md5);
    let download_page_url = Url::parse(&download_page_url_md5).unwrap();

    let content = client
        .get(download_page_url)
        .send()
        .await
        .or(Err("Couldn't connect to mirror"))?
        .bytes()
        .await
        .or(Err("Couldn't get mirror page"))?;

    match mirror.host_url.as_str() {
        "https://libgen.rocks/" => match download_book_from_ads(&content, mirror, client).await {
            Ok(b) => Ok(b),
            Err(_e) => Err("Download error"),
        },
        "http://libgen.lc/" => match download_book_from_ads(&content, mirror, client).await {
            Ok(b) => Ok(b),
            Err(_e) => Err("Download error"),
        },
        "http://libgen.lol/" => match download_book_from_lol(&content, mirror, client).await {
            Ok(b) => Ok(b),
            Err(_e) => Err("Download error"),
        },
        "http://libgen.me/" => match download_book_from_lol(&content, mirror, client).await {
            Ok(b) => Ok(b),
            Err(_e) => Err("Download error"),
        },
        &_ => Err("Couldn't find download url"),
    }
}

async fn download_book_from_ads(
    download_page: &Bytes,
    mirror: &Mirror,
    client: &Client,
) -> Result<reqwest::Response, &'static str> {
    let key = KEY_REGEX
        .captures(download_page)
        .map(|c| std::str::from_utf8(c.get(0).unwrap().as_bytes()).unwrap());
    if key.is_none() {
        return Err("Couldn't find download key");
    }

    let download_url = Url::parse(mirror.host_url.as_ref()).unwrap();
    let options = Url::options();
    let base_url = options.base_url(Some(&download_url));
    let download_url = base_url.parse(key.unwrap()).unwrap();
    client
        .get(download_url)
        .send()
        .await
        .or(Err("Couldn't connect to mirror"))
}

async fn download_book_from_lol(
    download_page: &Bytes,
    mirror: &Mirror,
    client: &Client,
) -> Result<reqwest::Response, &'static str> {
    let mut key = KEY_REGEX_LOL
        .captures(download_page)
        .map(|c| std::str::from_utf8(c.get(0).unwrap().as_bytes()).unwrap());
    if key.is_none() {
        key = KEY_REGEX_LOL_CLOUDFLARE
            .captures(download_page)
            .map(|c| std::str::from_utf8(c.get(0).unwrap().as_bytes()).unwrap());
    }
    if key.is_none() {
        key = KEY_REGEX_LOL_IPFS
            .captures(download_page)
            .map(|c| std::str::from_utf8(c.get(0).unwrap().as_bytes()).unwrap());
    }
    if key.is_none() {
        return Err("Couldn't find download key");
    }

    let download_url = Url::parse(mirror.host_url.as_ref()).unwrap();

    let options = Url::options();
    let base_url = options.base_url(Some(&download_url));
    let download_url = base_url.parse(key.unwrap()).unwrap();
    client
        .get(download_url)
        .send()
        .await
        .or(Err("Couldn't connect to mirror"))
}
