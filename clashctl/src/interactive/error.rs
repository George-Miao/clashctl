use std::path::PathBuf;

#[derive(Debug, thiserror::Error)]
pub enum InteractiveError {
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

    #[error("Config file cannot be parsed ({0})")]
    ConfigFileFormatError(#[from] ron::error::SpannedError),

    #[error("Config file cannot be generated ({0})")]
    ConfigFileGenerateError(#[from] ron::Error),
}

pub type InteractiveResult<T> = std::result::Result<T, InteractiveError>;
