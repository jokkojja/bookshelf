use crate::rest::api::api::{
    get_authors, get_book, get_books, get_genres, put_author, put_genre, ApiDoc, AppState,
    __path_get_authors, __path_get_book, __path_get_books, __path_get_genres, __path_put_author,
    __path_put_genre,
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
use utoipa_axum::{
    router::{self, OpenApiRouter},
    routes,
};
use utoipa_swagger_ui::SwaggerUi;

mod database;
mod rest;

async fn router() -> Result<OpenApiRouter, DatabaseError> {
    let database: Database = create_database().await?;
    let app_state = AppState { database };

    let cors = CorsLayer::new().allow_origin(Any).allow_methods([
        Method::GET,
        Method::POST,
        Method::PUT,
        Method::DELETE,
    ]);

    let router = OpenApiRouter::new()
        .routes(routes!(get_authors))
        .routes(routes!(put_author))
        .routes(routes!(get_genres))
        .routes(routes!(put_genre))
        .routes(routes!(get_book))
        .routes(routes!(get_books))
        .with_state(app_state)
        .layer(cors);

    Ok(router)
}
async fn create_database() -> Result<Database, DatabaseError> {
    let config = DatabaseConfig::from_env().expect("Invalid database config");
    let database: Database = Database::new(config).await?;
    Ok(database)
}
#[tokio::main]
async fn main() -> Result<(), DatabaseError> {
    env_logger::init();
    let api_config: ApiConfig = ApiConfig::from_env();
    let router = router().await?;

    let (router, api) = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .nest("/api/v1", router)
        .split_for_parts();

    let router =
        router.merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", api.clone()));

    let listener = tokio::net::TcpListener::bind(api_config.address)
        .await
        .unwrap();

    axum::serve(listener, router.into_make_service())
        .await
        .unwrap();
    Ok(())
}
