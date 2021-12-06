use std::path::PathBuf;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Clashctl error: {0}")]
    ClashCtl(#[from] clashctl_core::Error),

    #[error("Cannot find server")]
    ServerNotFound,

    #[error("{0} is not a directory")]
    ConfigFileTypeError(PathBuf),

    #[error("Config file cannot be found")]
    ConfigFileOpenError,

    #[error("Config file IO error ({0})")]
    ConfigFileIoError(std::io::Error),

    #[error("Config file cannot be parsed")]
    ConfigFileFormatError(#[from] ron::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
