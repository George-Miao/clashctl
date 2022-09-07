#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    InteractiveError(#[from] clashctl_interactive::Error),

    #[error("{0}")]
    TuiError(#[from] clashctl_tui::Error),

    #[error("{0}")]
    ClashCtl(#[from] crate::clashctl::Error),

    #[error("Requestty error")]
    RequesttyError(#[from] requestty::ErrorKind),
}

pub type Result<T> = std::result::Result<T, Error>;
