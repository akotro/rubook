-- Your SQL goes here
CREATE TABLE users (
    id CHAR(36) PRIMARY KEY,
    username TEXT NOT NULL,
    password TEXT NOT NULL
);

CREATE TABLE books (
    id VARCHAR(255) PRIMARY KEY
);

-- Create a new join table to associate users with books
CREATE TABLE user_books (
    id INTEGER PRIMARY KEY AUTO_INCREMENT,
    user_id CHAR(36) NOT NULL,
    book_id VARCHAR(255) NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    FOREIGN KEY (book_id) REFERENCES books(id) ON DELETE CASCADE
);

CREATE TABLE volume_infos (
    book_id VARCHAR(255) PRIMARY KEY,
    FOREIGN KEY (book_id) REFERENCES books(id) ON DELETE CASCADE,
    title TEXT,
    subtitle TEXT,
    publisher TEXT,
    published_date TEXT,
    description TEXT
);

CREATE TABLE authors (
    id INTEGER PRIMARY KEY AUTO_INCREMENT,
    book_id VARCHAR(255) NOT NULL,
    FOREIGN KEY (book_id) REFERENCES books(id) ON DELETE CASCADE,
    name TEXT NOT NULL
);

CREATE TABLE industry_identifiers (
    id INTEGER PRIMARY KEY AUTO_INCREMENT,
    book_id VARCHAR(255) NOT NULL,
    FOREIGN KEY (book_id) REFERENCES books(id) ON DELETE CASCADE,
    isbn_type TEXT NOT NULL,
    identifier TEXT NOT NULL
);

CREATE TABLE access_infos (
    book_id VARCHAR(255) PRIMARY KEY,
    FOREIGN KEY (book_id) REFERENCES books(id) ON DELETE CASCADE,
    epub_is_available BOOLEAN NOT NULL,
    pdf_is_available BOOLEAN NOT NULL
);
