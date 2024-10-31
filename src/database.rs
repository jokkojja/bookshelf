use crate::rest::models::author::{Author, Authors};
use crate::rest::models::book::{Book, Books};
use crate::rest::models::genres::{Genre, Genres};
use dotenv::dotenv;
use log::error;
use sqlx::sqlite::SqliteConnectOptions;
use sqlx::SqlitePool;
use std::env;
use std::str::FromStr;

pub struct DatabaseConfig {
    options: SqliteConnectOptions,
}

#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub enum DatabaseConfigError {
    InvalidEnv(#[from] std::env::VarError),
    InvalidDatabaseUrl(#[from] sqlx::Error),
}

#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub struct DatabaseError(#[from] pub sqlx::Error);

impl DatabaseConfig {
    pub fn from_env() -> Result<Self, DatabaseConfigError> {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL")?;
        let options = SqliteConnectOptions::from_str(&database_url)?;

        Ok(Self { options })
    }
}
#[derive(Clone)]
pub struct Database {
    pool: SqlitePool,
}
impl Database {
    pub async fn new(config: DatabaseConfig) -> Result<Self, DatabaseError> {
        let pool = SqlitePool::connect_with(config.options).await?;

        Ok(Self { pool })
    }

    pub async fn put_author(&self, author: Author) -> Result<(), DatabaseError> {
        sqlx::query!(
            "INSERT INTO AUTHORS (first_name, last_name) VALUES ($1, $2);",
            author.first_name,
            author.last_name
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn get_authors(&self) -> Result<Authors, DatabaseError> {
        let rows = sqlx::query!(
            r#"SELECT first_name as "first_name: String", last_name as "last_name: String" FROM authors;"#
        )
        .fetch_all(&self.pool)
        .await?;

        let authors = Authors {
            authors: rows
                .iter()
                .map(|row| Author {
                    first_name: row.first_name.clone(),
                    last_name: row.last_name.clone(),
                })
                .collect(),
        };
        Ok(authors)
    }

    pub async fn get_genres(&self) -> Result<Genres, DatabaseError> {
        let rows = sqlx::query!(r#"SELECT name as "genre: String" FROM genres"#)
            .fetch_all(&self.pool)
            .await?;

        let genres: Genres = Genres {
            genres: rows
                .iter()
                .map(|row| Genre {
                    genre: row.genre.clone(),
                })
                .collect(),
        };

        Ok(genres)
    }

    pub async fn put_genre(&self, genre: Genre) -> Result<(), DatabaseError> {
        sqlx::query!("INSERT INTO genres (name) VALUES ($1);", genre.genre)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn get_books(&self) -> Result<Books, DatabaseError> {
        let rows = sqlx::query!(
            r#"SELECT books.title as "title: String",
        authors.first_name as "first_name: String",
        authors.last_name as "last_name: String",
        genres.name as "genre: String"
        from books
            join authors on books.author_id = authors.id
            join genres on books.genre_id = genres.id;
        "#
        )
        .fetch_all(&self.pool)
        .await?;
        let books: Books = Books {
            books: rows
                .iter()
                .map(|row| Book {
                    title: row.title.clone(),
                    author: Author {
                        first_name: row.first_name.clone(),
                        last_name: row.last_name.clone(),
                    },
                    genre: row.genre.clone(),
                })
                .collect(),
        };
        Ok(books)
    }

    pub async fn get_book(&self, book_id: String) -> Result<Book, DatabaseError> {
        let row = sqlx::query!(
            r#"SELECT books.title as "title: String",
        authors.first_name as "first_name: String",
        authors.last_name as "last_name: String",
        genres.name as "genre: String"
        from books
            join authors on books.author_id = authors.id
            join genres on books.genre_id = genres.id
        where books.id = $1;
        "#,
            book_id
        )
        .fetch_one(&self.pool)
        .await?;

        let book: Book = Book {
            title: row.title.clone(),
            author: Author {
                first_name: row.first_name.clone(),
                last_name: row.last_name.clone(),
            },
            genre: row.genre.clone(),
        };
        Ok(book)
    }
}
