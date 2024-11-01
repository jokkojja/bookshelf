use crate::rest::api::api::{
    get_authors, get_book, get_books, get_genres, put_author, put_genre, ApiDoc, AppState,
};

use axum::{
    http::Method,
    routing::{get, put},
    Router,
};
use database::{Database, DatabaseConfig, DatabaseError};
use rest::models::config::ApiConfig;
use tower_http::cors::{Any, CorsLayer};
use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;
use utoipa_swagger_ui::SwaggerUi;

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

    let cors = CorsLayer::new().allow_origin(Any).allow_methods([
        Method::GET,
        Method::POST,
        Method::PUT,
        Method::DELETE,
    ]);

    // let app = OpenApiRouter::with_openapi(ApiDoc::openapi())
    //     .route("/authors", get(get_authors))
    //     .route("/author", put(put_author))
    //     .route("/genres", get(get_genres))
    //     .route("/genre", put(put_genre))
    //     .route("/books", get(get_books))
    //     .route("/books/:id", get(get_book))
    //     .with_state(app_state)
    //     .layer(cors)
    //     .nest(path, router)
    //     .into_make_service();

    let app = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .route("/authors", get(get_authors))
        .route("/author", put(put_author))
        .route("/genres", get(get_genres))
        .route("/genre", put(put_genre))
        .route("/books", get(get_books))
        .route("/books/:id", get(get_book))
        .with_state(app_state)
        .layer(cors)
        .nest(path, router)
        .into_make_service();

    // let swagger_ui = SwaggerUi::new("/docs").url("/api-doc/openapi.json", ApiDoc::openapi());

    // let app = app.merge(swagger_ui);

    let listener = tokio::net::TcpListener::bind(api_config.address)
        .await
        .unwrap();

    axum::serve(listener, app).unwrap();
    Ok(())
}
