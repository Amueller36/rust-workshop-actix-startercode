use crate::models::Book;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct BooksDb(Arc<Mutex<HashMap<u32, Book>>>);

impl BooksDb {
    pub fn new() -> Self {
        BooksDb(Arc::new(Mutex::new(HashMap::new())))
    }

    pub fn lock(&self) -> std::sync::MutexGuard<HashMap<u32, Book>> {
        self.0.lock().unwrap()
    }
}