use serde::{Deserialize, Serialize};
use std::{collections::HashMap, ops::Deref, ops::DerefMut};

pub struct Library {
    books: HashMap<String, Book>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Book {
    pub title: String,
    pub author: String,
    pub year: u32,
    pub isbn: String,
}

impl Library {
    pub fn new() -> Self {
        Library {
            books: HashMap::new(),
        }
    }

    pub fn add_book(&mut self, book: Book) {
        let isbn = book.isbn.clone();
        println!("Added book {:?}", book);
        self.books.insert(isbn, book);
    }

    pub fn get_book(&self, isbn: &str) -> Option<&Book> {
        self.books.get(isbn)
    }

    pub fn remove_book(&mut self, isbn: &str) -> Result<String, String> {
        let removed = self.books.remove(isbn);
        match removed {
            Some(_) => Ok("Removed".to_string()),
            None => Err("Nonexistant".to_string())
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
