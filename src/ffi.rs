use crate::{
    common::{
        serialize, Method, RequestData, SendBytesData, SendFormData, SendJsonData, SendStringData,
    },
    error::HostError,
};

extern {
    fn http_call_request(bytes: u32, bytes_len: u32, output: u32, output_len: &mut u32) -> u32;
    fn http_send_bytes(bytes: u32, bytes_len: u32, output: u32, output_len: &mut u32) -> u32;
    fn http_send_string(bytes: u32, bytes_len: u32, output: u32, output_len: &mut u32) -> u32;
    fn http_send_form(bytes: u32, bytes_len: u32, output: u32, output_len: &mut u32) -> u32;
    fn http_send_json(bytes: u32, bytes_len: u32, output: u32, output_len: &mut u32) -> u32;
}

/// Calls http request, then returns length of body
pub fn call_request(
    url: String,
    headers: Vec<String>,
    queries: Vec<String>,
    method: Method,
    output: &mut [u8],
) -> Result<u32, HostError> {
    let mut output_len = 0;
    let data = RequestData {
        url,
        headers,
        queries,
        method,
    };
    let bytes = serialize(&data).map_err(|_| HostError::SerializeData)?;
    let status = unsafe {
        http_call_request(
            bytes.as_ptr() as u32,
            bytes.len() as u32,
            output.as_ptr() as u32,
            &mut output_len,
        )
    };
    HostError::try_from(status).map(|_| output_len)
}

/// Sends http request with bytes, then returns length of body
pub fn send_bytes(
    url: String,
    headers: Vec<String>,
    queries: Vec<String>,
    method: Method,
    data: Vec<u8>,
    output: &mut [u8],
) -> Result<u32, HostError> {
    let mut output_len = 0;
    let data = SendBytesData {
        request_data: RequestData {
            url,
            headers,
            queries,
            method,
        },
        data,
    };
    let bytes = serialize(&data).map_err(|_| HostError::SerializeData)?;
    let status = unsafe {
        http_send_bytes(
            bytes.as_ptr() as u32,
            bytes.len() as u32,
            output.as_ptr() as u32,
            &mut output_len,
        )
    };
    HostError::try_from(status).map(|_| output_len)
}

/// Sends http request with string, then returns length of body
pub fn send_string(
    url: String,
    headers: Vec<String>,
    queries: Vec<String>,
    method: Method,
    data: String,
    output: &mut [u8],
) -> Result<u32, HostError> {
    let mut output_len = 0;
    let data = SendStringData {
        request_data: RequestData {
            url,
            headers,
            queries,
            method,
        },
        data,
    };
    let bytes = serialize(&data).map_err(|_| HostError::SerializeData)?;
    let status = unsafe {
        http_send_string(
            bytes.as_ptr() as u32,
            bytes.len() as u32,
            output.as_ptr() as u32,
            &mut output_len,
        )
    };
    HostError::try_from(status).map(|_| output_len)
}

/// Sends http request with form, then returns length of body
pub fn send_form(
    url: String,
    headers: Vec<String>,
    queries: Vec<String>,
    method: Method,
    data: Vec<(String, String)>,
    output: &mut [u8],
) -> Result<u32, HostError> {
    let mut output_len = 0;
    let data = SendFormData {
        request_data: RequestData {
            url,
            headers,
            queries,
            method,
        },
        data,
    };
    let bytes = serialize(&data).map_err(|_| HostError::SerializeData)?;
    let status = unsafe {
        http_send_form(
            bytes.as_ptr() as u32,
            bytes.len() as u32,
            output.as_ptr() as u32,
            &mut output_len,
        )
    };
    HostError::try_from(status).map(|_| output_len)
}

/// Sends http request with json, then returns length of body
pub fn send_json<T>(
    url: String,
    headers: Vec<String>,
    queries: Vec<String>,
    method: Method,
    data: &T,
    output: &mut [u8],
) -> Result<u32, HostError>
where
    T: serde::Serialize,
{
    let mut output_len = 0;
    let data = SendJsonData {
        request_data: RequestData {
            url,
            headers,
            queries,
            method,
        },
        data: serde_json::to_string(data).map_err(|_| HostError::SerializeData)?,
    };
    let bytes = serialize(&data).map_err(|_| HostError::SerializeData)?;
    let status = unsafe {
        http_send_json(
            bytes.as_ptr() as u32,
            bytes.len() as u32,
            output.as_ptr() as u32,
            &mut output_len,
        )
    };
    HostError::try_from(status).map(|_| output_len)
}
