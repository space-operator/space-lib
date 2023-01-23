use crate::{common::Method, error::HostError, ffi, SpaceError};

pub struct Request {
    url: String,
    method: Method,
    headers: Vec<String>,
    queries: Vec<String>,
}

impl Request {
    /// Create a new request with method.
    pub fn new<T: Into<String>>(url: T, method: Method) -> Self {
        Self {
            url: url.into(),
            method,
            headers: Vec::new(),
            queries: Vec::new(),
        }
    }

    /// Make a GET request.
    pub fn get<T: Into<String>>(url: T) -> Self {
        Self::new(url, Method::GET)
    }

    /// Make a POST request.
    pub fn post<T: Into<String>>(url: T) -> Self {
        Self::new(url, Method::POST)
    }

    /// Make a DELETE request.
    pub fn delete<T: Into<String>>(url: T) -> Self {
        Self::new(url, Method::DELETE)
    }

    /// Make a HEAD request.
    pub fn head<T: Into<String>>(url: T) -> Self {
        Self::new(url, Method::HEAD)
    }

    /// Make a PATCH request.
    pub fn patch<T: Into<String>>(url: T) -> Self {
        Self::new(url, Method::PATCH)
    }

    /// Make a PUT request.
    pub fn put<T: Into<String>>(url: T) -> Self {
        Self::new(url, Method::PUT)
    }

    /// Set a header field.
    pub fn set<T: ToString, U: ToString>(mut self, header: T, value: U) -> Self {
        self.headers.extend([header.to_string(), value.to_string()]);
        self
    }

    /// Set a query parameter.
    pub fn query<T: ToString, U: ToString>(mut self, param: T, value: U) -> Self {
        self.queries.extend([param.to_string(), value.to_string()]);
        self
    }

    /// Send bytes with request.
    pub fn send_bytes<T: Into<Vec<u8>>>(self, data: T) -> Result<Response, HostError> {
        let bytes = ffi::send_bytes(
            self.url,
            self.headers,
            self.queries,
            self.method,
            data.into(),
        )?;
        Ok(Response { bytes })
    }

    /// Send string with request.
    pub fn send_string<T: Into<String>>(self, data: T) -> Result<Response, HostError> {
        let bytes = ffi::send_string(
            self.url,
            self.headers,
            self.queries,
            self.method,
            data.into(),
        )?;
        Ok(Response { bytes })
    }

    /// Send form with request.
    pub fn send_form(self, data: Vec<(String, String)>) -> Result<Response, HostError> {
        let bytes = ffi::send_form(self.url, self.headers, self.queries, self.method, data)?;
        Ok(Response { bytes })
    }

    /// Send json with request.
    pub fn send_json(self, data: impl serde::Serialize) -> Result<Response, HostError> {
        let bytes = ffi::send_json(self.url, self.headers, self.queries, self.method, &data)?;
        Ok(Response { bytes })
    }

    /// Send the request.
    pub fn call(self) -> Result<Response, HostError> {
        let bytes = ffi::call_request(self.url, self.headers, self.queries, self.method)?;
        Ok(Response { bytes })
    }
}

pub struct Response {
    bytes: Vec<u8>,
}

impl Response {
    pub fn into_vec(self) -> Vec<u8> {
        self.bytes
    }

    pub fn into_string(self) -> Result<String, SpaceError> {
        Ok(std::str::from_utf8(&self.bytes).map(|it| it.to_string())?)
    }

    pub fn into_json<T: serde::de::DeserializeOwned>(self) -> Result<T, SpaceError> {
        let json = std::str::from_utf8(&self.bytes)?;
        Ok(serde_json::from_str(json)?)
    }
}
