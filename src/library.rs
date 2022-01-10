use std::{collections::HashMap, ops::DerefMut, ops::Deref};
use serde::{Deserialize, Serialize};

pub struct Library {
    books: HashMap<String, Book>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Book {
    title: String,
    author: String,
    year: u32,
    isbn: String,
}

impl Book {
    fn new(title: String, author: String, year: u32, isbn: String) -> Self {
        Book {
            title,
            author,
            year,
            isbn,
        }
    }
}

impl Library {
    pub fn new() -> Self {
        Library {
            books: HashMap::new(),
        }
    }

    pub fn add_book(&mut self, book: Book) {
        let isbn = book.isbn.clone();
        self.books.insert(isbn, book);
    }

    pub fn get_book(&self, isbn: &str) -> Option<&Book> {
        self.books.get(isbn)
    }

    pub fn remove_book(&mut self, isbn: &str) {
        self.books.remove(isbn);
    }

    fn print_books(&self) {
        for (isbn, book) in &self.books {
            println!("{} - {}", isbn, book.title);
        }
    }

    pub fn get_books(&self) -> Vec<&Book> {
        let books: Vec<&Book> = self.books.values().collect();
        books
    }

    pub fn search_book(&self, query: &str) -> Vec<&Book> {
        let mut results = Vec::new();
        for (_isbn, book) in &self.books {
            if book.title.contains(query) || book.author.contains(query) {
                results.push(book);
            }
        }
        results
    }
}

impl DerefMut for Library {
    fn deref_mut(&mut self) -> &mut HashMap<String, Book> {
        self
    }
}

impl Deref for Library {
    type Target = HashMap<String, Book>;

    fn deref(&self) -> &Self::Target {
        &self.books
    }
}