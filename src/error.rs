#[cfg(feature = "cli")]
use requestty::ErrorKind;
use reqwest::StatusCode;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Invalid URL format")]
    UrlParseError,
    #[error("Error whiling requesting API")]
    RequestError(#[from] reqwest::Error),
    #[error("Broken response from server")]
    BadResponseEncoding,
    #[error("Broken response from server")]
    BadResponseFormat,
    #[error("Failed response from server")]
    FailedResponse(StatusCode),
    #[cfg(feature = "cli")]
    #[error("Requestty error")]
    RequesttyError(#[from] ErrorKind),
}

pub type Result<T> = std::result::Result<T, Error>;
