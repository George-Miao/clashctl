#[derive(thiserror::Error, Debug)]
pub enum ErrorKind {
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

#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub struct Error(Box<ErrorKind>);

impl Error {
    pub fn url_parse() -> Self {
        Error(Box::new(ErrorKind::UrlParseError))
    }

    pub fn failed_response(status: u16) -> Self {
        Error(Box::new(ErrorKind::FailedResponse(status)))
    }

    pub fn bad_response_encoding() -> Self {
        Error(Box::new(ErrorKind::BadResponseEncoding))
    }

    pub fn other(msg: String) -> Self {
        Error(Box::new(ErrorKind::Other(msg)))
    }
}

impl<E> From<E> for Error
where
    ErrorKind: From<E>,
{
    fn from(err: E) -> Self {
        Error(Box::new(ErrorKind::from(err)))
    }
}

pub type Result<T> = std::result::Result<T, Error>;
