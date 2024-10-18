use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq)]
pub enum FetchError {
    #[error("Url parsing error: {0}")]
    UrlParsingError(String),
    #[error("Slugs serialization error: {0}")]
    SlugsSerializationError(String),
    #[error("Query deserialization error: {0}")]
    QuerySerializationError(String),
    #[error("Header initialization error: {0}")]
    HeaderInitializationError(String),
    #[error("Header mutation error: {0}")]
    HeaderMutationError(String),
    #[error("Body serialization error: {0}")]
    BodySerializationError(String),
    #[error("Network error: {0}")]
    NetworkError(String),
    #[error("Window not found")]
    WindowNotFound(),
    #[error("Invalid response: {0}")]
    InvalidResponse(String),
    #[error("Http error: {0}")]
    HttpError(String),
    #[error("Json error: {0}")]
    JsonError(String),
    #[error("Response deserialization error: {0}")]
    ResponseDeserializationError(String),
    #[error("Unknown error: {0}")]
    UnknownError(String),
}
