use crate::rest::models::author::{Author, Authors};
use dotenv::dotenv;
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
        unimplemented!();
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
}
