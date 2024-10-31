use serde::{Deserialize, Serialize};
use sqlx::Decode;
use utoipa::ToSchema;

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize, ToSchema)]
pub struct Genres {
    pub genres: Vec<Genre>,
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize, Decode, ToSchema)]
pub struct Genre {
    pub genre: String,
}
