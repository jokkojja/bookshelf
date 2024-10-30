use super::author::Author;
use super::genre::Genre;

pub struct Books {
    books: Vec<Book>,
}
pub struct Book {
    pub name: String,
    pub author: Author,
    pub genre: Genre,
}
