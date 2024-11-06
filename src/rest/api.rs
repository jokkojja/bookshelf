use crate::database::Database;
use crate::rest::models::author::{Author, Authors};
use crate::rest::models::book::{Book, Books};
use crate::rest::models::genres::{Genre, Genres};

use axum::extract::{Path, State};
use axum::{http::StatusCode, Json};
use utoipa::OpenApi;

// There are two methods for adding paths in Swagger. One involves deriving an Api struct
// with paths to methods, which are specified in #[utoipa::path] attributes.
// The other method uses OpenApiRouter::with_openapi(ApiDoc::openapi()).nest("/api/v1", router),
// where `router` is an OpenApiRouter containing routes for the methods, defined as
// OpenApiRouter::new().routes(routes!(get_authors, put_author)).
// If both methods are used, the routes will be duplicated.

const TODO_TAG: &str = "bookshelf";
// #[openapi(paths(get_authors, put_author, get_genres, put_genre, get_books))]
#[derive(OpenApi)]
#[openapi(
    tags(
        (name = TODO_TAG, description = "Books management API")
    )
)]
pub struct ApiDoc;

#[derive(Clone)]
pub struct AppState {
    pub database: Database,
}

#[utoipa::path(
get,
path="/authors",
tag=TODO_TAG,
responses(
    (status = 200, body = [Authors], description = "List of all authors successfully"),
    (status = 404)
))
]
pub async fn get_authors(State(state): State<AppState>) -> Result<Json<Authors>, StatusCode> {
    let authors = state
        .database
        .get_authors()
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR))?;
    Ok(Json(authors))
}

#[utoipa::path(
put,
path="/authors",
tag=TODO_TAG,
responses(
    (status = 200),
    (status = 404)
))
]
pub async fn put_author(
    State(state): State<AppState>,
    Json(author): Json<Author>,
) -> Result<StatusCode, StatusCode> {
    state
        .database
        .put_author(author)
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR))?;
    Ok(StatusCode::OK)
}

#[utoipa::path(
get,
path="/genres",
tag=TODO_TAG,
responses(
    (status = 200, body = [Genres]),
    (status = 404)
))
]
pub async fn get_genres(State(state): State<AppState>) -> Result<Json<Genres>, StatusCode> {
    let genres = state
        .database
        .get_genres()
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR))?;

    Ok(Json(genres))
}

#[utoipa::path(
put,
path="/genres",
tag=TODO_TAG,
responses(
    (status = 200),
    (status = 404)
))
]
pub async fn put_genre(
    State(state): State<AppState>,
    Json(genre): Json<Genre>,
) -> Result<StatusCode, StatusCode> {
    state
        .database
        .put_genre(genre)
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR))?;

    Ok(StatusCode::OK)
}

#[utoipa::path(
get,
path="/books",
tag=TODO_TAG,
responses(
    (status = 200, body = [Books]),
    (status = 404)
))
]
pub async fn get_books(State(state): State<AppState>) -> Result<Json<Books>, StatusCode> {
    let books = state
        .database
        .get_books()
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR))?;
    Ok(Json(books))
}

#[utoipa::path(
get,
path="/books/{id}",
tag=TODO_TAG,
responses(
    (status = 200, body = [Book]),
    (status = 404)
),
params(
("id" = i32, Path, description = "Book id"))
)
]
pub async fn get_book(
    State(state): State<AppState>,
    Path(book_id): Path<i32>,
) -> Result<Json<Book>, StatusCode> {
    let book = state
        .database
        .get_book(book_id)
        .await
        .map_err(|_| (StatusCode::NOT_FOUND))?;
    Ok(Json(book))
}
