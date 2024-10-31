use super::models::author::{Author, Authors};
use super::models::genres::{self, Genre, Genres};
use crate::database::Database;
use axum::extract::State;
use axum::{http::StatusCode, Json};

#[derive(Clone)]
pub struct AppState {
    pub database: Database,
}

pub async fn get_authors(State(state): State<AppState>) -> Result<Json<Authors>, StatusCode> {
    let authors = state
        .database
        .get_authors()
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR))?;
    Ok(Json(authors))
}

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

pub async fn get_genres(State(state): State<AppState>) -> Result<Json<Genres>, StatusCode> {
    let genres = state
        .database
        .get_genres()
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR))?;

    Ok(Json(genres))
}

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
