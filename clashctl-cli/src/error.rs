#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    InteractiveError(#[from] clashctl_interactive::Error),

    #[error("Clashctl error: {0}")]
    ClashCtl(#[from] crate::clashctl::Error),

    #[error("Bad option")]
    BadOption,

    #[error("Requestty error")]
    RequesttyError(#[from] requestty::ErrorKind),
}

pub type Result<T> = std::result::Result<T, Error>;
