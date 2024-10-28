use super::genre::Genre;

pub struct Books {
    books: Vec<Book>,
}
pub struct Book {
    pub name: String,
    pub author: String,
    pub genre: Genre,
}
