use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize, Clone)]
pub struct LibgenBook {
    pub id: String,
    pub title: String,
    pub author: String,
    pub filesize: String,
    pub year: String,
    pub language: String,
    pub pages: String,
    pub publisher: String,
    pub edition: String,
    pub extension: String,
    pub md5: String,
    pub coverurl: String,
}

impl LibgenBook {
    #![allow(dead_code)]
    fn print_libgen_book_info(&self) -> Result<(), &'static str> {
        println!("ID: {}", self.id);
        println!("Title: {}", self.title);
        println!("Author: {}", self.author);
        println!(
            "Filesize: {:.2} Mb",
            self.filesize.parse::<u32>().unwrap() as f32 / 1048576.0
        );
        println!("Year: {}", self.year);
        println!("Language: {}", self.language);
        println!("Pages: {}", self.pages);
        println!("Publisher: {}", self.publisher);
        println!("Edition: {}", self.edition);
        println!("MD5: {}", self.md5);
        println!("Cover: {}", self.coverurl);
        Ok(())
    }
}

impl fmt::Display for LibgenBook {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}.{}, {} = {}",
            self.title,
            self.extension,
            self.author,
            self.filesize.parse::<u32>().unwrap() as f32 / 1048576.0
        )
    }
}
