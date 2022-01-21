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

#[derive(Debug)]
pub enum LibraryResult {
    AlreadyExists,
    BookUpdated,
    BookAdded,
    DoesNotExist
}

impl Library {
    pub fn new() -> Self {
        Library {
            books: HashMap::new(),
        }
    }

    pub fn add_book(&mut self, book: Book) -> Result<LibraryResult, LibraryResult> {
        let isbn = book.isbn.clone();
        if self.books.contains_key(&isbn) {
            Err(LibraryResult::AlreadyExists)
        } else {
            let result = self.books.insert(isbn, book);
            match result {
                Some(_) => Err(LibraryResult::BookUpdated),
                None => Ok(LibraryResult::BookAdded)
            }
        }
    }

    pub fn get_book(&self, isbn: &str) -> Result<&Book, LibraryResult> {
        match self.books.get(isbn) {
            Some(book) => Ok(book),
            None => Err(LibraryResult::DoesNotExist)
        }
    }

    pub fn remove_book(&mut self, isbn: &str) -> Result<String, String> {
        let removed = self.books.remove(isbn);
        match removed {
            Some(_) => Ok("Removed".to_string()),
            None => Err("Nonexistant".to_string()),
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
        &mut self.books
    }
}

impl Deref for Library {
    type Target = HashMap<String, Book>;

    fn deref(&self) -> &Self::Target {
        &self.books
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_library() {
        // Initialize library test
        let mut lib = Library::new();
        let book1 = Book {
            author: "Mark Twain".to_string(),
            title: "Huckleberry Finn".to_string(),
            year: 1876,
            isbn: "012-34567-890".to_string(),
        };
        let book2 = Book {
            author: "Mark Twain".to_string(),
            title: "Tom Sawyer".to_string(),
            year: 1877,
            isbn: "123-45678-901".to_string(),
        };
        let book3 = Book {
            author: "Dingus McDingleberry".to_string(),
            title: "Being a Dingleberry".to_string(),
            year: 2021,
            isbn: "321-65432-901".to_string(),
        };
        match lib.add_book(book1) {
            Ok(LibraryResult::BookAdded) => (),
            _ => panic!("Expected BookAdded"),
        }
        match lib.add_book(book2) {
            Ok(LibraryResult::BookAdded) => (),
            _ => panic!("Expected BookAdded"),
        }
        match lib.add_book(book3) {
            Ok(LibraryResult::BookAdded) => (),
            _ => panic!("Expected BookAdded"),
        }

        // Test get_book
        match lib.get_book("012-34567-890") {
            Ok(book) => {
                assert_eq!(book.title, "Huckleberry Finn".to_string())
            },
            Err(LibraryResult::DoesNotExist) => panic!("Didn't retrieve the right book!"),
            _ => panic!("Expected a book"),
        };

        // Test get_books
        let result = lib.get_books();
        assert_eq!(result.len(), 3, "Incorrect number of books.");

        // Test search_book
        let result = lib.search_book("Mark Twain");
        assert_eq!(result.len(), 2, "Incorrect number of books.");

        // Test remove_book
        let result = lib.remove_book("012-34567-890");
        assert_eq!(result.unwrap(), "Removed".to_string());
    }
}
