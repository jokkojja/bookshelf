use super::author::Author;

pub struct Books {
    books: Vec<Book>,
}
pub struct Book {
    pub name: String,
    pub author: Author,
    pub genre: String,
}
