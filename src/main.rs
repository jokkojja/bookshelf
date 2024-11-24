// There __path using like shit, but i didnt find any other way to use
// endpoints defined in other file, like my api.rs
use crate::rest::api::{
    get_authors, get_book, get_books, get_genres, put_author, put_genre, ApiDoc, AppState,
    __path_get_authors, __path_get_book, __path_get_books, __path_get_genres, __path_put_author,
    __path_put_genre,
};
use axum::http::Method;
use database::{Database, DatabaseConfig};
use dotenv::dotenv;
use rest::models::config::ApiConfig;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use tower_http::trace::{DefaultMakeSpan, DefaultOnResponse};
use tracing_subscriber::layer::SubscriberExt;
use utoipa::OpenApi;
use utoipa_axum::{router::OpenApiRouter, routes};
use utoipa_swagger_ui::SwaggerUi;
mod database;
mod rest;

async fn router() -> Result<OpenApiRouter, anyhow::Error> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                format!(
                    "{}=debug,tower_http=debug,axum::rejection=trace",
                    env!("CARGO_CRATE_NAME")
                )
                .into()
            }),
        )
        .with(tracing_subscriber::fmt::layer());

    let database: Database = create_database().await?;
    let app_state = AppState { database };

    let cors = CorsLayer::new().allow_origin(Any).allow_methods([
        Method::GET,
        Method::POST,
        Method::PUT,
        Method::DELETE,
    ]);

    let router = OpenApiRouter::new()
        .routes(routes!(get_authors, put_author))
        .routes(routes!(get_genres, put_genre))
        .routes(routes!(get_book))
        .routes(routes!(get_books))
        .with_state(app_state)
        .layer(cors)
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::default().include_headers(true)) // Log headers
                .on_response(DefaultOnResponse::new().include_headers(true)), // Log response details
        );

    Ok(router)
}
async fn create_database() -> Result<Database, anyhow::Error> {
    let config: DatabaseConfig = serde_env::from_env()?;
    let database: Database = Database::new(config).await?;
    Ok(database)
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    dotenv().ok();
    env_logger::init();
    let api_config: ApiConfig = ApiConfig::from_env();
    let router = router().await?;

    let (router, api) = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .nest("/api/v1", router)
        .split_for_parts();

    let router = router.merge(SwaggerUi::new("/docs").url("/api-docs/openapi.json", api));

    let listener = tokio::net::TcpListener::bind(api_config.address)
        .await
        .unwrap();

    axum::serve(listener, router.into_make_service())
        .await
        .unwrap();
    Ok(())
}
