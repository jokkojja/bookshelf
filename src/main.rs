use axum::{routing::get, routing::put, Router};
mod database;
mod rest;

use rest::models::config::ApiConfig;
#[tokio::main]
async fn main() {
    let api_config: ApiConfig = ApiConfig::from_env();

    let app = Router::new().route("/", get(|| async { "Hello world" }));
    // .route("/", put(add_gook))
    // .route("/books", get(get_books))
    // .route("/book/:id", get(get_book_by_id)

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(api_config.address)
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
