use std::{
    fs::{read, File},
    io::{stdout, Write},
    str::from_utf8,
    sync::mpsc::{self, Sender},
    thread,
    time::Duration,
};

use inquire::Select;
use reqwest::{Client, Response};

use crate::{
    libgen::{
        download::download_book,
        mirrors::{Mirror, MirrorList, MirrorType},
        models::LibgenBook,
        search::{search_fiction, search_non_fiction, SearchType},
    },
    models::Book,
};

pub async fn libgen_book_download(book: Book) -> Result<(), String> {
    let client = Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .expect("Could not build reqwest client");

    let mirrors = parse_mirrors();

    println!("Getting working search mirrors...");
    let working_search_mirrors = mirrors
        .get_working_mirrors(MirrorType::Search, &client)
        .await?;
    let search_mirror = select_mirror(MirrorType::Search, &working_search_mirrors);

    let selected_search_type = select_search_type();

    match selected_search_type {
        SearchType::NonFiction => {
            let books = search_non_fiction(&book, &search_mirror, &client).await?;

            if books.is_empty() {
                return Err(String::from("No books were found"));
            }

            let selected_book = select_libgen_book(&books, "Select a book to download");

            println!("Getting working download mirrors...");
            let working_download_mirrors = mirrors
                .get_working_mirrors(MirrorType::Download, &client)
                .await?;
            let download_mirror = select_mirror(MirrorType::Download, &working_download_mirrors);

            let download_response =
                download_book(&client, &download_mirror, &selected_book).await?;

            write_response_to_file(download_response, &selected_book)
                .await
                .expect("Failed to save file");
        }
        SearchType::Fiction => {
            println!("*****************************************************");
            println!("NOTE: Fiction books are not fully supported yet");
            println!("The first book found will be downloaded automatically");
            println!("*****************************************************");
        }
    }

    Ok(())
}

fn start_loading_spinner() -> Sender<()> {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let spinner = "|/-\\";
        let mut i = 0;
        loop {
            if rx.try_recv().is_ok() {
                break;
            }
            print!("\r{}", spinner.chars().nth(i).unwrap());
            stdout().flush().unwrap();
            i = (i + 1) % spinner.len();
            thread::sleep(Duration::from_millis(100));
        }
    });
    tx
}

async fn write_response_to_file(
    mut response: Response,
    libgen_book: &LibgenBook,
) -> Result<(), Box<dyn std::error::Error>> {
    let total_size = response.content_length().unwrap_or(0);
    let mut downloaded: u64 = 0;

    let file_path = format!(
        "{} - {}.{}",
        libgen_book.title, libgen_book.author, libgen_book.extension
    );
    let mut file = File::create(file_path)?;

    let tx = start_loading_spinner();

    while let Some(chunk) = response.chunk().await? {
        file.write_all(&chunk)?;
        downloaded += chunk.len() as u64;
        print!(
            "\r   Progress -- {:.2}%",
            (downloaded as f64 / total_size as f64) * 100.0
        );
    }

    println!("\nDone!");
    tx.send(()).unwrap();

    Ok(())
}

fn parse_mirrors() -> MirrorList {
    let mirror_path = "resources/mirrors.json";
    let json = from_utf8(&read(mirror_path).expect("Couldn't read mirrors from json"))
        .unwrap()
        .to_owned();
    MirrorList::parse_mirrors(&json)
}

fn select_mirror(mirror_type: MirrorType, mirrors: &Vec<Mirror>) -> Mirror {
    match mirror_type {
        MirrorType::Search => Select::new("Select a search mirror:", mirrors.clone())
            .prompt()
            .expect("No valid mirror selected"),
        MirrorType::Download => Select::new("Select a download mirror:", mirrors.clone())
            .prompt()
            .expect("No valid mirror selected"),
    }
}

fn select_search_type() -> SearchType {
    let options = vec![SearchType::NonFiction, SearchType::Fiction];

    Select::new("Select search type:", options)
        .prompt()
        .expect("No valid search type selected")
}

fn select_libgen_book(books: &Vec<LibgenBook>, prompt: &str) -> LibgenBook {
    Select::new(prompt, books.clone())
        .prompt()
        .expect("No valid book selected")
}
