use crate::SpaceError;
use serde::{Deserialize, Serialize};

pub fn serialize<T: Serialize + ?Sized>(data: &T) -> Result<Vec<u8>, SpaceError> {
    Ok(rmp_serde::to_vec(data)?)
}

pub fn deserialize<'a, T: Deserialize<'a>>(data: &'a [u8]) -> Result<T, SpaceError> {
    Ok(rmp_serde::from_slice(data)?)
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
