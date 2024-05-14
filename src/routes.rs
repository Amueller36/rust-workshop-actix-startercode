use actix_web::{get, web, HttpResponse, Responder};
use crate::db::BooksDb;


#[get("/hello")]
pub async fn hello(books_db: web::Data<BooksDb>) -> impl Responder {
    let books = books_db.lock();
    let num_books = books.len();
    HttpResponse::Ok().body(format!("Number of books: {}", num_books))
}


// Hier weitere Routen hinzufügen
