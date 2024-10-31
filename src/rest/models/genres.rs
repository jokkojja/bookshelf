use serde::{Deserialize, Serialize};
use sqlx::Decode;

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Genres {
    pub genres: Vec<Genre>,
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize, Decode)]
pub struct Genre {
    pub genre: String,
}
