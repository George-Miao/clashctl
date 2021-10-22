#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Invalid URL format")]
    UrlParseError,
    #[error("Error whiling requesting API")]
    RequestError,
    #[error("Broken response from server")]
    BadResponseEncoding,
    #[error("Broken response from server")]
    BadResponseFormat,
}

pub type Result<T> = std::result::Result<T, Error>;
