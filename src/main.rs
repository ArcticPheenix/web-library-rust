use actix_web::{get, post, delete, web, App, HttpResponse, HttpServer, Responder};
use std::sync::Mutex;
mod library;

struct AppState {
    library: Mutex<library::Library>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let library = library::Library::new("Library".to_string());
    let state = web::Data::new(AppState {
        library: Mutex::new(library),
    });
    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .service(
                web::scope("/api")
                    .route("/books", web::get().to(get_books))
                    // .route("/book/{isbn}", web::get().to(get_book))
                    // .route("/book/{isbn}", web::post().to(add_book))
                    // .route("/book/{isbn}", web::delete().to(remove_book)),
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

#[get("/books")]
async fn get_books(state: web::Data<AppState>) -> impl Responder {
    let library = state.library.lock().unwrap();
    HttpResponse::Ok().json(library.get_books())
}