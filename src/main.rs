use std::collections::HashMap;
use actix_web::{web, App, HttpServer};
use std::sync::{Arc, Mutex};
use crate::routes::hello;

mod models;
mod db;
mod routes;



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let books_db = web::Data::new(Arc::new(Mutex::new(HashMap::<u32, models::Book>::new())));

    HttpServer::new(move || {
        App::new()
            .app_data(books_db.clone())
            .service(hello)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}