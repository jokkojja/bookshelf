use serde::{Deserialize, Serialize};

use super::author::Author;

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Books {
    pub books: Vec<Book>,
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Book {
    pub title: String,
    pub author: Author,
    pub genre: String,
}
