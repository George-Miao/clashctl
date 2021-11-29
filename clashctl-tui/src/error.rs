#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    InteractiveError(#[from] clashctl_interactive::Error),

    #[error("Clashctl error: {0}")]
    ClashCtl(#[from] crate::clashctl::Error),

    #[error("TUI error")]
    TuiError(#[from] std::io::Error),

    #[error("TUI backend error")]
    TuiBackendErr,

    #[error("TUI interuptted error")]
    TuiInterupttedErr,

    #[error("TUI internal error")]
    TuiInternalErr,

    #[error("Set logger error ({0})")]
    SetLoggerError(#[from] log::SetLoggerError),
}

impl<T> From<std::sync::mpsc::SendError<T>> for Error {
    fn from(_: std::sync::mpsc::SendError<T>) -> Self {
        Self::TuiBackendErr
    }
}

pub type Result<T> = std::result::Result<T, Error>;
