#[cfg(feature = "cli")]
use std::path::PathBuf;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Invalid URL format")]
    UrlParseError,

    #[error("Error whiling requesting API ({0})")]
    RequestError(#[from] ureq::Error),

    #[error("Broken response from server")]
    BadResponseEncoding,

    #[error("Broken response from server: {0}")]
    BadResponseFormat(#[from] serde_json::Error),

    #[error("Failed response from server (Code {0})")]
    FailedResponse(u16),

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
    #[error("Config file IO error ({0})")]
    ConfigFileIoError(std::io::Error),

    #[cfg(feature = "cli")]
    #[error("Config file cannot be parsed")]
    ConfigFileFormatError(#[from] ron::Error),

    #[cfg(feature = "cli")]
    #[error("Bad option")]
    BadOption,

    #[cfg(feature = "ui")]
    #[error("TUI error")]
    TuiError(#[from] std::io::Error),

    #[cfg(feature = "ui")]
    #[error("TUI backend error")]
    TuiBackendErr,

    #[cfg(feature = "ui")]
    #[error("TUI interuptted error")]
    TuiInterupttedErr,

    #[cfg(feature = "ui")]
    #[error("TUI internal error")]
    TuiInternalErr,

    #[cfg(feature = "ui")]
    #[error("TUI internal error")]
    SetLoggerError(#[from] log::SetLoggerError),

    #[error("Other errors ({0})")]
    Other(String),
}

#[cfg(feature = "ui")]
impl<T> From<std::sync::mpsc::SendError<T>> for Error {
    fn from(_: std::sync::mpsc::SendError<T>) -> Self {
        Self::TuiBackendErr
    }
}

pub type Result<T> = std::result::Result<T, Error>;
