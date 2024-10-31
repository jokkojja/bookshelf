use axum::{routing::get, routing::put, Router};

use database::{Database, DatabaseConfig, DatabaseError};
use rest::api::{get_authors, get_book, get_books, get_genres, put_author, put_genre, AppState};
use rest::models::config::ApiConfig;

mod database;
mod rest;

async fn create_database() -> Result<Database, DatabaseError> {
    let config = DatabaseConfig::from_env().expect("Invalid database config");
    let database: Database = Database::new(config).await?;
    Ok(database)
}
#[tokio::main]
async fn main() -> Result<(), DatabaseError> {
    env_logger::init();
    let api_config: ApiConfig = ApiConfig::from_env();
    let database: Database = create_database().await?;
    let app_state = AppState { database };
    let app = Router::new()
        .route("/", get(|| async { "Hello world" }))
        .route("/authors", get(get_authors))
        .route("/author", put(put_author))
        .route("/genres", get(get_genres))
        .route("/genre", put(put_genre))
        .route("/books", get(get_books))
        .route("/books/:id", get(get_book))
        .with_state(app_state); // Задаем состояние приложения для маршрутизатора

    let listener = tokio::net::TcpListener::bind(api_config.address)
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
    Ok(())
}
