use thiserror::Error;

#[derive(Error, Debug)]
pub enum SpaceError {
    #[error("utf8 error occured: `{0}`")]
    Utf8(#[from] std::str::Utf8Error),
    #[error("json error occured: `{0}`")]
    Json(#[from] serde_json::Error),
    #[error("message pack serializing error: `{0}`")]
    RmpSerialize(#[from] rmp_serde::encode::Error),
    #[error("message pack deserializing error: `{0}`")]
    RmpDeserialize(#[from] rmp_serde::decode::Error),
}

#[derive(Error, Debug)]
pub enum HostError {
    #[error("reading raw bytes from guest")]
    ReadRawBytes = 1,
    #[error("trying to deserialize raw bytes")]
    DeserializeBytes = 2,
    #[error("calling http request")]
    CallHttpRequest = 3,
    #[error("reading http response")]
    ReadResponse = 4,
    #[error("accessing web assembly memory")]
    MemoryAccess = 5,
    #[error("sending http request with body")]
    SendHttpRequest = 6,
    #[error("serializing data")]
    SerializeData = 7,
    #[error("growing memory")]
    GrowingMemory = 8,
    #[error("writing to memory")]
    WritingMemory = 9,
}
