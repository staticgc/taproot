
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("I/O error")]
    IOError(#[from] std::io::Error),

    #[error("KeyValue Impl error")]
    ImplError(String),

    #[error("Unknown KeyValue error")]
    Unknown,
}