use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Book {
    isbn: String,
    name: String,
    authors: Vec<String>,
    tags: Vec<String>,
}
