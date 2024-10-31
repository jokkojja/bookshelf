use axum::{routing::get, routing::put, Router};

use database::{Database, DatabaseConfig, DatabaseError};
use rest::books::{get_authors, AppState};
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
    let api_config: ApiConfig = ApiConfig::from_env();
    let database: Database = create_database().await?;
    let app_state = AppState { database };

    let app = Router::new()
        .route("/", get(|| async { "Hello world" }))
        .route("/authors", get(get_authors))
        .with_state(app_state); // Задаем состояние приложения для маршрутизатора

    let listener = tokio::net::TcpListener::bind(api_config.address)
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
    Ok(())
}
