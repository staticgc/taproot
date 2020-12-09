
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Init error {0}")]
    InitError(String),

    #[error("Index not found")]
    IndexNotFound,

    #[error("Diff error")]
    InvalidDiffState,

    #[error("Pack not found v={0} p={1}")]
    PackNotFound(u16, u32),

    #[error("Commit not found v={0}")]
    CommitNotFound(u16),

    #[error("Head version not found v={0}")]
    HeadVersionNotFound(u16),

    #[error("Version not found v={0}")]
    VersionNotFound(u16),

    #[error("I/O error")]
    IOError(#[from] std::io::Error),

    #[error("KeyValue error")]
    KVError(#[from] keyvalue::Error),

    #[error("Msgpack encode error")]
    MsgpackEncodeError(#[from] rmp_serde::encode::Error),

    #[error("Msgpack decode error")]
    MsgpackDecodeError(#[from] rmp_serde::decode::Error),

    #[error("Unknown KeyValue error")]
    Unknown,
}