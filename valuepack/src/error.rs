
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("I/O error")]
    IOError(#[from] std::io::Error),

    #[error("Msgpack decode error")]
    MsgpackDecodeError(#[from] rmp_serde::decode::Error),

    #[error("Msgpack encode error")]
    MsgpackEncodeError(#[from] rmp_serde::encode::Error),

    #[error("Unknown pack error")]
    Unknown,
}