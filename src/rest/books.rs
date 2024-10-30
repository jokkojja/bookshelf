use super::models::author::Author;
use crate::database::{Database, DatabaseError};
use axum::extract::State;
use axum::{http::StatusCode, Json};
pub struct AppState {
    pub database: Database,
}

pub struct MaybeJson<T>(pub Option<T>);

#[axum::debug_handler]
pub async fn get_authors(
    State(state): State<AppState>,
) -> Result<(StatusCode, Json<Vec<Author>>), (StatusCode, String)> {
    let authors = state.database.get_authors().await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Database error: {}", e),
        )
    })?;
    Ok((StatusCode::OK, Json(authors)))
}
