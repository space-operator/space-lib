use crate::{SpaceError, HostError};
use serde::{Deserialize, Serialize};

pub fn serialize<T: Serialize + ?Sized>(data: &T) -> Result<Vec<u8>, SpaceError> {
    Ok(rmp_serde::to_vec_named(data)?)
}

pub fn deserialize<'a, T: Deserialize<'a>>(data: &'a [u8]) -> Result<T, SpaceError> {
    Ok(rmp_serde::from_slice(data)?)
}

pub fn extract(status: u64) -> Result<u32, HostError> {
    match status >> 32 {
        1 => Err(HostError::ReadRawBytes),
        2 => Err(HostError::DeserializeBytes),
        3 => Err(HostError::CallHttpRequest),
        4 => Err(HostError::ReadResponse),
        5 => Err(HostError::MemoryAccess),
        6 => Err(HostError::SendHttpRequest),
        7 => Err(HostError::SerializeData),
        8 => Err(HostError::GrowingMemory),
        9 => Err(HostError::WritingMemory),
        _  => Ok((status & 0xFFFFFFFF) as u32),
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq)]
pub enum Method {
    GET,
    POST,
    DELETE,
    HEAD,
    PATCH,
    PUT,
}

#[derive(Serialize, Deserialize)]
pub struct RequestData {
    pub url: String,
    pub headers: Vec<String>,
    pub queries: Vec<String>,
    pub method: Method,
}

#[derive(Serialize, Deserialize)]
pub struct SendBytesData {
    pub request_data: RequestData,
    pub data: Vec<u8>,
}

#[derive(Serialize, Deserialize)]
pub struct SendStringData {
    pub request_data: RequestData,
    pub data: String,
}

#[derive(Serialize, Deserialize)]
pub struct SendFormData {
    pub request_data: RequestData,
    pub data: Vec<(String, String)>,
}

#[derive(Serialize, Deserialize)]
pub struct SendJsonData {
    pub request_data: RequestData,
    pub data: String,
}
