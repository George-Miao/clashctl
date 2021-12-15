#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Invalid URL format")]
    UrlParseError,

    #[error("Error while requesting API ({0})")]
    RequestError(#[from] ureq::Error),

    #[error("Broken response from server")]
    BadResponseEncoding,

    #[error("Broken response from server: {0}")]
    BadResponseFormat(#[from] serde_json::Error),

    #[error("Failed response from server (Code {0})")]
    FailedResponse(u16),

    #[error("Other errors ({0})")]
    Other(String),
}

pub type Result<T> = std::result::Result<T, Error>;
