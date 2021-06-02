use thiserror::Error as ThisError;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(ThisError, Debug)]
pub enum Error {
    #[error("Repository owner name is not defined")]
    EmptyRepositoryOwner,
    #[error("Repository name is not defined")]
    EmptyRepositoryName,
    #[error("Failed to parse URL with provided params. {0}")]
    UrlParseError(String),
}
