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
}

impl HostError {
    pub fn try_from(status: u32) -> Result<(), HostError> {
        match status {
            1 => Err(Self::ReadRawBytes),
            2 => Err(Self::DeserializeBytes),
            3 => Err(Self::CallHttpRequest),
            4 => Err(Self::ReadResponse),
            5 => Err(Self::MemoryAccess),
            6 => Err(Self::SendHttpRequest),
            7 => Err(Self::SerializeData),
            _  => Ok(()),
        }        
    }
}
