use std::{
    fs::{read, File},
    io::Write,
    str::from_utf8,
};

use inquire::Select;
use reqwest::{Client, Response};

use crate::{
    libgen::{
        download::download_book,
        mirrors::{Mirror, MirrorList, MirrorType},
        models::LibgenBook,
        search::search,
    },
    models::Book,
};

pub async fn libgen_book_download(book: Book) -> Result<(), &'static str> {
    let client = Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .expect("Could not build reqwest client");

    let mirrors = parse_mirrors();
    let search_mirror = select_mirror(MirrorType::Search, &mirrors);
    match search_mirror.check_connection(&client).await {
        Ok(_) => (),
        Err(code) => {
            println!("Connection failed with status code: {}", code);
            return Err("Connection to mirror failed");
        }
    };

    let books = search(book, &search_mirror, &client).await?;
    for book in books.iter() {
        print_libgen_book_info(book)?;
    }
    let selected_book = select_libgen_book(&books, "Select a book to download");

    let download_mirror = select_mirror(MirrorType::Download, &mirrors);

    let download_response = download_book(&client, &download_mirror, &selected_book).await?;

    write_response_to_file(download_response, "test").await.expect("Failed to save file");

    Ok(())
}

async fn write_response_to_file(
    mut response: Response,
    file_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let total_size = response.content_length().unwrap_or(0);
    let mut downloaded: u64 = 0;
    let mut file = File::create(file_path)?;

    while let Some(chunk) = response.chunk().await? {
        file.write_all(&chunk)?;
        downloaded += chunk.len() as u64;
        print!(
            "\rProgress: {:.2}%",
            (downloaded as f64 / total_size as f64) * 100.0
        );
    }
    println!("\nDone!");
    Ok(())
}

fn parse_mirrors() -> MirrorList {
    let mirror_path = "resources/mirrors.json";
    let json = from_utf8(&read(mirror_path).expect("Couldn't read mirrors from json"))
        .unwrap()
        .to_owned();
    MirrorList::parse_mirrors(&json)
}

fn select_mirror(mirror_type: MirrorType, mirrors: &MirrorList) -> Mirror {
    match mirror_type {
        MirrorType::Search => {
            Select::new("Select a search mirror:", mirrors.search_mirrors.clone())
                .prompt()
                .expect("No valid mirror selected")
        }
        MirrorType::Download => Select::new(
            "Select a download mirror:",
            mirrors.download_mirrors.clone(),
        )
        .prompt()
        .expect("No valid mirror selected"),
    }
}

fn select_libgen_book(books: &Vec<LibgenBook>, prompt: &str) -> LibgenBook {
    Select::new(prompt, books.clone())
        .prompt()
        .expect("No valid book selected")
}

fn print_libgen_book_info(libgen_book: &LibgenBook) -> Result<(), &'static str> {
    println!("ID: {}", libgen_book.id);
    println!("Title: {}", libgen_book.title);
    println!("Author: {}", libgen_book.author);
    println!(
        "Filesize: {:.2} Mb",
        libgen_book.filesize.parse::<u32>().unwrap() as f32 / 1048576.0
    );
    println!("Year: {}", libgen_book.year);
    println!("Language: {}", libgen_book.language);
    println!("Pages: {}", libgen_book.pages);
    println!("Publisher: {}", libgen_book.publisher);
    println!("Edition: {}", libgen_book.edition);
    println!("MD5: {}", libgen_book.md5);
    println!("Cover: {}", libgen_book.coverurl);
    Ok(())
}
