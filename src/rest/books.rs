use super::models::author::{self, Authors};
use crate::database::{Database, DatabaseError};
use axum::extract::State;
use axum::{http::StatusCode, Json};
#[derive(Clone)]
pub struct AppState {
    pub database: Database,
}

pub struct MaybeJson<T>(pub Option<T>);

pub async fn get_authors(State(state): State<AppState>) -> Result<Json<Authors>, StatusCode> {
    let authors = state
        .database
        .get_authors()
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR))?;
    Ok(Json(authors))
}
