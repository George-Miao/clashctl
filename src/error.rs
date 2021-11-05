#[cfg(feature = "cli")]
use std::path::PathBuf;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Invalid URL format")]
    UrlParseError,

    #[error("Error whiling requesting API ({0})")]
    RequestError(#[from] attohttpc::Error),

    #[error("Broken response from server")]
    BadResponseEncoding,

    #[error("Broken response from server")]
    BadResponseFormat,

    #[error("Failed response from server (Code {0})")]
    FailedResponse(attohttpc::StatusCode),

    #[cfg(feature = "cli")]
    #[error("Cannot find server")]
    ServerNotFound,

    #[cfg(feature = "cli")]
    #[error("Requestty error")]
    RequesttyError(#[from] requestty::ErrorKind),

    #[cfg(feature = "cli")]
    #[error("{0} is not a file")]
    ConfigFileTypeError(PathBuf),

    #[cfg(feature = "cli")]
    #[error("Config file cannot be found")]
    ConfigFileOpenError,

    #[cfg(feature = "cli")]
    #[error("Config file cannot be read")]
    ConfigFileIoError(std::io::Error),

    #[cfg(feature = "cli")]
    #[error("Config file cannot be parsed")]
    ConfigFileFormatError(#[from] ron::Error),

    #[cfg(feature = "cli")]
    #[error("Bad option")]
    BadOption,
}

pub type Result<T> = std::result::Result<T, Error>;
