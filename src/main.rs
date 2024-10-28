use axum::{routing::get, routing::put, Router};
mod database;
mod rest;

use database::{Database, DatabaseConfig, DatabaseError};
use rest::models::config::ApiConfig;

async fn create_database() -> Result<Database, DatabaseError> {
    let config = DatabaseConfig::from_env().expect("Invalid database config");
    let database: Database = Database::new(config).await?;
    Ok(database)
}
#[tokio::main]
async fn main() -> Result<(), DatabaseError> {
    let api_config: ApiConfig = ApiConfig::from_env();
    let database: Database = create_database().await?;
    let app = Router::new().route("/", get(|| async { "Hello world" }));
    // .route("/", put(add_gook))
    // .route("/books", get(get_books))
    // .route("/book/:id", get(get_book_by_id)

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(api_config.address)
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
    Ok(())
}
