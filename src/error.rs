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
    FailedResponse(reqwest::StatusCode),

    #[cfg(feature = "cli")]
    #[error("Cannot find server")]
    ServerNotFound,

    #[cfg(feature = "cli")]
    #[error("Requestty error")]
    RequesttyError(#[from] requestty::ErrorKind),

    #[cfg(feature = "cli")]
    #[error("Config file cannot be read")]
    ConfigFileIoError(std::io::Error),

    #[cfg(feature = "cli")]
    #[error("Config file cannot be parsed")]
    ConfigFileFormatError(#[from] ron::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
