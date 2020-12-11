
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Vstore error")]
    VStoreError(#[from] vstore::Error),

    #[error("KeyValue error")]
    KVError(#[from] keyvalue::Error),

    #[error("Msgpack encode error")]
    MsgpackEncodeError(#[from] rmp_serde::encode::Error),

    #[error("Msgpack decode error")]
    MsgpackDecodeError(#[from] rmp_serde::decode::Error),

    #[error("Unknown FS error")]
    Unknown,
}