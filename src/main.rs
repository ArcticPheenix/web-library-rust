use actix_web::{body, web, App, HttpResponse, HttpServer};
use serde::Deserialize;
use std::sync::Mutex;
mod library;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let library = library::Library::new();
    // let book = library::Book::new("Test Book".to_string(), "Dingus".to_string(), 2021, "123-45678-901".to_string());
    // library.add_book(book);
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
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

#[derive(Debug, Deserialize)]
pub struct Params {
    q: String,
}

async fn get_books(data: web::Data<Mutex<library::Library>>) -> HttpResponse {
    println!("Entering get_books");
    let library = data.lock().unwrap();
    let books = library.get_books();
    HttpResponse::Ok().json(books)
}

async fn get_book(
    info: web::Path<String>,
    data: web::Data<Mutex<library::Library>>,
) -> HttpResponse {
    let library = data.lock().unwrap();
    let book = library.get_book(&info).unwrap();
    HttpResponse::Ok().json(book)
}

async fn add_book(
    item: web::Json<library::Book>,
    data: web::Data<Mutex<library::Library>>,
) -> HttpResponse {
    println!("Entered add_book");
    let mut library = data.lock().unwrap();
    let book = item.0;
    println!("Adding book: {:?}", book);
    library.add_book(book);
    HttpResponse::NoContent().body(body::Body::Empty)
}

async fn delete_book(
    info: web::Path<String>,
    data: web::Data<Mutex<library::Library>>,
) -> HttpResponse {
    println!("Entered delete_book");
    let mut library = data.lock().unwrap();
    library.remove_book(&info).unwrap();
    HttpResponse::Ok().body("Removed")
}

async fn search_book(
    web::Query(search): web::Query<Params>,
    data: web::Data<Mutex<library::Library>>,
) -> HttpResponse {
    println!("Entered search_book");
    let library = data.lock().unwrap();
    let result = library.search_book(&search.q);
    HttpResponse::Ok().json(result)
}
