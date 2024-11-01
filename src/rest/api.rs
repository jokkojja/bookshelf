pub mod api {
    use crate::database::Database;
    use crate::rest::models::author::{Author, Authors};
    use crate::rest::models::book::{Book, Books};
    use crate::rest::models::genres::{Genre, Genres};

    use axum::extract::{Path, State};
    use axum::{http::StatusCode, Json};
    use log::info;
    use utoipa::OpenApi;

    #[derive(OpenApi)]
    #[openapi(paths(get_authors, put_author, get_genres, put_genre, get_books))]
    pub struct ApiDoc;

    #[derive(Clone)]
    pub struct AppState {
        pub database: Database,
    }

    #[utoipa::path(
    get,
    path="/authors",
    responses(
        (status = 200, body = [Authors]),
        (status = 404)
    ))
]
    pub async fn get_authors(State(state): State<AppState>) -> Result<Json<Authors>, StatusCode> {
        info!("Call method: get_authors with author");
        let authors = state
            .database
            .get_authors()
            .await
            .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(Json(authors))
    }

    #[utoipa::path(
    put,
    path="/author",
    responses(
        (status = 200),
        (status = 404)
    ))
]
    pub async fn put_author(
        State(state): State<AppState>,
        Json(author): Json<Author>,
    ) -> Result<StatusCode, StatusCode> {
        info!(
            "Call method: put_author with author: {} {}",
            author.last_name, author.first_name
        );
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
    responses(
        (status = 200, body = [Genres]),
        (status = 404)
    ))
]
    pub async fn get_genres(State(state): State<AppState>) -> Result<Json<Genres>, StatusCode> {
        info!("Call method: get genres with genre");
        let genres = state
            .database
            .get_genres()
            .await
            .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR))?;

        Ok(Json(genres))
    }

    #[utoipa::path(
    put,
    path="/genre",
    responses(
        (status = 200),
        (status = 404)
    ))
]
    pub async fn put_genre(
        State(state): State<AppState>,
        Json(genre): Json<Genre>,
    ) -> Result<StatusCode, StatusCode> {
        info!("Call method: put_genre with genre: {}", genre.genre);
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
    responses(
        (status = 200, body = [Books]),
        (status = 404)
    ))
]
    pub async fn get_books(State(state): State<AppState>) -> Result<Json<Books>, StatusCode> {
        info!("Call method: get_books");

        let books = state
            .database
            .get_books()
            .await
            .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(Json(books))
    }

    #[utoipa::path(
    put,
    path="/book",
    responses(
        (status = 200),
        (status = 404)
    ))
]
    pub async fn get_book(
        State(state): State<AppState>,
        Path(book_id): Path<String>,
    ) -> Result<Json<Book>, StatusCode> {
        info!("Call method: get_book with ID: {}", book_id);
        let book = state
            .database
            .get_book(book_id)
            .await
            .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(Json(book))
    }
}
