use crate::{
    common::{
        extract, serialize, Method, RequestData, SendBytesData, SendFormData, SendJsonData,
        SendStringData,
    },
    error::HostError,
};

extern "C" {
    fn http_call_request(bytes: u32, bytes_len: u32) -> u64;
    fn http_send_bytes(bytes: u32, bytes_len: u32) -> u64;
    fn http_send_string(bytes: u32, bytes_len: u32) -> u64;
    fn http_send_form(bytes: u32, bytes_len: u32) -> u64;
    fn http_send_json(bytes: u32, bytes_len: u32) -> u64;
}

/// Calls http request, then returns length of body
pub fn call_request(
    url: String,
    headers: Vec<String>,
    queries: Vec<String>,
    method: Method,
) -> Result<Vec<u8>, HostError> {
    // Call ffi function
    let data = RequestData {
        url,
        headers,
        queries,
        method,
    };
    let bytes = serialize(&data).map_err(|_| HostError::SerializeData)?;
    let status = unsafe { http_call_request(bytes.as_ptr() as u32, bytes.len() as u32) };

    // Extract pointer and read Vec<u8> from memory
    let offset = extract(status)?;
    unsafe {
        let ptr = *((offset + 0) as *const u32);
        let cap = *((offset + 4) as *const u32);
        let len = *((offset + 8) as *const u32);
        Ok(Vec::from_raw_parts(
            ptr as *mut u8,
            cap as usize,
            len as usize,
        ))
    }
}

/// Sends http request with bytes, then returns length of body
pub fn send_bytes(
    url: String,
    headers: Vec<String>,
    queries: Vec<String>,
    method: Method,
    data: Vec<u8>,
) -> Result<Vec<u8>, HostError> {
    // Call ffi function
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
    let status = unsafe { http_send_bytes(bytes.as_ptr() as u32, bytes.len() as u32) };

    // Extract pointer and read Vec<u8> from memory
    let offset = extract(status)?;
    unsafe {
        let ptr = *((offset + 0) as *const u32);
        let cap = *((offset + 4) as *const u32);
        let len = *((offset + 8) as *const u32);
        Ok(Vec::from_raw_parts(
            ptr as *mut u8,
            cap as usize,
            len as usize,
        ))
    }
}

/// Sends http request with string, then returns length of body
pub fn send_string(
    url: String,
    headers: Vec<String>,
    queries: Vec<String>,
    method: Method,
    data: String,
) -> Result<Vec<u8>, HostError> {
    // Call ffi function
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
    let status = unsafe { http_send_string(bytes.as_ptr() as u32, bytes.len() as u32) };

    // Extract pointer and read Vec<u8> from memory
    let offset = extract(status)?;
    unsafe {
        let ptr = *((offset + 0) as *const u32);
        let cap = *((offset + 4) as *const u32);
        let len = *((offset + 8) as *const u32);
        Ok(Vec::from_raw_parts(
            ptr as *mut u8,
            cap as usize,
            len as usize,
        ))
    }
}

/// Sends http request with form, then returns length of body
pub fn send_form(
    url: String,
    headers: Vec<String>,
    queries: Vec<String>,
    method: Method,
    data: Vec<(String, String)>,
) -> Result<Vec<u8>, HostError> {
    // Call ffi function
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
    let status = unsafe { http_send_form(bytes.as_ptr() as u32, bytes.len() as u32) };

    // Extract pointer and read Vec<u8> from memory
    let offset = extract(status)?;
    unsafe {
        let ptr = *((offset + 0) as *const u32);
        let cap = *((offset + 4) as *const u32);
        let len = *((offset + 8) as *const u32);
        Ok(Vec::from_raw_parts(
            ptr as *mut u8,
            cap as usize,
            len as usize,
        ))
    }
}

/// Sends http request with json, then returns length of body
pub fn send_json<T>(
    url: String,
    headers: Vec<String>,
    queries: Vec<String>,
    method: Method,
    data: &T,
) -> Result<Vec<u8>, HostError>
where
    T: serde::Serialize,
{
    // Call ffi function
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
    let status = unsafe { http_send_json(bytes.as_ptr() as u32, bytes.len() as u32) };

    // Extract pointer and read Vec<u8> from memory
    let offset = extract(status)?;
    unsafe {
        let ptr = *((offset + 0) as *const u32);
        let cap = *((offset + 4) as *const u32);
        let len = *((offset + 8) as *const u32);
        Ok(Vec::from_raw_parts(
            ptr as *mut u8,
            cap as usize,
            len as usize,
        ))
    }
}
