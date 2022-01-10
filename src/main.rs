use std::collections::HashMap;

struct Book {
    title: String,
    author: String,
    year: u32,
    isbn: String,
}

struct Library {
    name: String,
    books: HashMap<String, Book>,
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
    fn new(name: String) -> Self {
        Library {
            name,
            books: HashMap::new(),
        }
    }

    fn add_book(&mut self, book: Book) {
        let isbn = book.isbn.clone();
        self.books.insert(isbn, book);
    }

    fn get_book(&self, isbn: &str) -> Option<&Book> {
        self.books.get(isbn)
    }

    fn remove_book(&mut self, isbn: &str) {
        self.books.remove(isbn);
    }

    fn print_books(&self) {
        for (isbn, book) in &self.books {
            println!("{} - {}", isbn, book.title);
        }
    }

    fn search_book(&self, query: &str) -> Vec<&Book> {
        let mut results = Vec::new();
        for (_isbn, book) in &self.books {
            if book.title.contains(query) || book.author.contains(query) {
                results.push(book);
            }
        }
        results
    }
}

fn main() {
    println!("Hello, world!");
}
