use actix_web::{body, web, App, HttpResponse, HttpServer};
use serde::Deserialize;
use std::sync::Mutex;
mod library;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let library = library::Library::new();
    let state = web::Data::new(Mutex::new(library));

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .service(web::resource("/books").route(web::get().to(get_books)))
            .service(
                web::resource("/book/{isbn}")
                    .route(web::get().to(get_book))
                    .route(web::delete().to(delete_book)),
            )
            .service(web::resource("/book").route(web::post().to(add_book)))
            .service(web::resource("/search").route(web::get().to(search_book)))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}

#[derive(Debug, Deserialize)]
pub struct Params {
    q: String,
}

async fn get_books(data: web::Data<Mutex<library::Library>>) -> HttpResponse {
    let library = data.lock().unwrap();
    let books = library.get_books();
    HttpResponse::Ok().json(books)
}

async fn get_book(
    info: web::Path<String>,
    data: web::Data<Mutex<library::Library>>,
) -> HttpResponse {
    let library = data.lock().unwrap();
    match library.get_book(&info) {
        Ok(book) => HttpResponse::Ok().json(book),
        Err(_) => HttpResponse::NotFound().body("Book not found"),
    }
}

async fn add_book(
    item: web::Json<library::Book>,
    data: web::Data<Mutex<library::Library>>,
) -> HttpResponse {
    let mut library = data.lock().unwrap();
    let book = item.0;
    match library.add_book(book) {
        Ok(library::LibraryResult::BookAdded) => HttpResponse::NoContent().body(body::Body::Empty),
        Err(library::LibraryResult::AlreadyExists) => {
            HttpResponse::Conflict().body(body::Body::Empty)
        },
        _ => HttpResponse::InternalServerError().body(body::Body::Empty),
    }
}

async fn delete_book(
    info: web::Path<String>,
    data: web::Data<Mutex<library::Library>>,
) -> HttpResponse {
    let mut library = data.lock().unwrap();
    match library.remove_book(&info) {
        Ok(library::LibraryResult::BookRemoved) => HttpResponse::NoContent().body(body::Body::Empty),
        Err(library::LibraryResult::DoesNotExist) => {
            HttpResponse::NotFound().body(body::Body::Empty)
        },
        _ => HttpResponse::InternalServerError().body(body::Body::Empty),
    }
}

async fn search_book(
    web::Query(search): web::Query<Params>,
    data: web::Data<Mutex<library::Library>>,
) -> HttpResponse {
    let library = data.lock().unwrap();
    let result = library.search_book(&search.q);
    HttpResponse::Ok().json(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{http, test, App};

    #[actix_rt::test]
    async fn test_add_book() {
        let mut app = test::init_service(
            App::new()
                .app_data(web::Data::new(Mutex::new(library::Library::new())))
                .service(
                    web::resource("/book")
                        .route(web::post().to(add_book))
                        .route(web::get().to(get_books)),
                ),
        )
        .await;

        let book = library::Book {
            title: "Test Book".to_string(),
            author: "Dingus".to_string(),
            year: 2021,
            isbn: "123-45678-901".to_string(),
        };
        let req = test::TestRequest::post()
            .uri("/book")
            .set_json(&book)
            .to_request();
        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), http::StatusCode::NO_CONTENT);
    }

    #[actix_rt::test]
    async fn test_search_book() {
        // Need to add a few books first
        let mut app = test::init_service(
            App::new()
                .app_data(web::Data::new(Mutex::new(library::Library::new())))
                .service(web::resource("/book").route(web::post().to(add_book)))
                .service(web::resource("/search").route(web::get().to(search_book))),
        )
        .await;

        let book1 = library::Book {
            title: "Test Book".to_string(),
            author: "Dingus".to_string(),
            year: 2021,
            isbn: "123-45678-901".to_string(),
        };
        let book2 = library::Book {
            title: "Test Book 2".to_string(),
            author: "Dingus".to_string(),
            year: 2022,
            isbn: "123-45678-902".to_string(),
        };
        let book3 = library::Book {
            title: "Test Book 3".to_string(),
            author: "Testy McTesterson".to_string(),
            year: 2020,
            isbn: "123-45678-903".to_string(),
        };
        let book3_copy = library::Book {
            title: "Test Book 3".to_string(),
            author: "Testy McTesterson".to_string(),
            year: 2020,
            isbn: "123-45678-903".to_string(),
        };

        // Add books
        let req = test::TestRequest::post()
            .uri("/book")
            .set_json(&book1)
            .to_request();
        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), http::StatusCode::NO_CONTENT);

        let req = test::TestRequest::post()
            .uri("/book")
            .set_json(&book2)
            .to_request();
        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), http::StatusCode::NO_CONTENT);

        let req = test::TestRequest::post()
            .uri("/book")
            .set_json(&book3)
            .to_request();
        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), http::StatusCode::NO_CONTENT);

        // Try to add a duplicate book
        let req = test::TestRequest::post()
            .uri("/book")
            .set_json(&book3_copy)
            .to_request();
        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), http::StatusCode::CONFLICT);

        // Search for books
        let req = test::TestRequest::get()
            .uri("/search?q=Dingus")
            .to_request();
        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), http::StatusCode::OK);

        let req = test::TestRequest::get()
            .uri("/search?q=Dingus")
            .to_request();
        let result: Vec<library::Book> = test::read_response_json(&mut app, req).await;
        assert_eq!(result.len(), 2);

        // Get book
        let req = test::TestRequest::get()
            .uri("/book/101-12345-101")
            .to_request();
        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), http::StatusCode::NOT_FOUND);

        // Delete book
        let req = test::TestRequest::delete()
            .uri("/book/123-45678-901")
            .to_request();
        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), http::StatusCode::NO_CONTENT);

        // Delete book again
        let req = test::TestRequest::delete()
            .uri("/book/123-45678-901")
            .to_request();
        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), http::StatusCode::NOT_FOUND);
    }
}
