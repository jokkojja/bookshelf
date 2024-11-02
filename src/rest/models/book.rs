use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use super::author::Author;

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize, ToSchema)]
pub struct Books {
    pub books: Vec<Book>,
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize, ToSchema)]
pub struct Book {
    pub title: String,
    pub author: Author,
    pub genre: String,
}
