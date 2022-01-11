use actix_web::{body, web, App, HttpResponse, HttpServer};
use std::sync::Mutex;
mod library;

struct AppState {
    library: Mutex<library::Library>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let library = library::Library::new();
    let state = web::Data::new(AppState {
        library: Mutex::new(library),
    });

    HttpServer::new(move || {
        App::new()
            .data(state.clone())
            .service(web::resource("/books").route(web::get().to(get_books)))
            .service(web::resource("/book/{isbn}").route(web::get().to(get_book)))
            .service(web::resource("/book").route(web::post().to(add_book)))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

async fn get_books(data: web::Data<AppState>) -> HttpResponse {
    let library = data.library.lock().unwrap();
    let books = library.get_books();
    HttpResponse::Ok().json(books)
}

async fn get_book(info: web::Path<String>, data: web::Data<AppState>) -> HttpResponse {
    let library = data.library.lock().unwrap();
    let book = library.get_book(&info).unwrap();
    HttpResponse::Ok().json(book)
}

async fn add_book(item: web::Json<library::Book>, data: web::Data<AppState>) -> HttpResponse {
    let mut library = data.library.lock().unwrap();
    let book = item.0;
    library.add_book(book);
    HttpResponse::NoContent().body(body::Body::Empty)
}
