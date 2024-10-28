use dotenv::dotenv;
use sqlx::sqlite::SqliteConnectOptions;
use sqlx::SqlitePool;
use std::env;
use std::str::FromStr;

struct DatabaseConfig {
    options: SqliteConnectOptions,
}

#[derive(Debug)]
enum DatabaseConfigError {
    EnvVar(std::env::VarError),
    Parse(sqlx::Error),
}
#[derive(Debug)]
enum DatabaseError {
    Pool(sqlx::Error),
}

impl From<std::env::VarError> for DatabaseConfigError {
    fn from(err: std::env::VarError) -> Self {
        DatabaseConfigError::EnvVar(err)
    }
}

impl From<sqlx::Error> for DatabaseConfigError {
    fn from(err: sqlx::Error) -> Self {
        DatabaseConfigError::Parse(err)
    }
}
impl From<sqlx::Error> for DatabaseError {
    fn from(err: sqlx::Error) -> Self {
        DatabaseError::Pool(err)
    }
}
impl DatabaseConfig {
    fn from_env() -> Result<Self, DatabaseConfigError> {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL")?;
        let options = SqliteConnectOptions::from_str(&database_url)?;

        Ok(Self { options })
    }
}

struct Database {
    pool: SqlitePool,
}

impl Database {
    async fn new(config: DatabaseConfig) -> Result<Self, DatabaseError> {
        let pool = SqlitePool::connect_with(config.options).await?;

        Ok(Self { pool })
    }
}
