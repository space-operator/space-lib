//! This crate provides WebAssembly host functions and other utilities for Space Operator.
//! 
//! ## Macro
//! 
//! ```rust
//! use space_lib::{space, Result};
//! use serde::{Serialize, Deserialize};
//! 
//! #[derive(Deserialize)]
//! struct Input {
//!     value: usize,
//!     name: String,
//! }
//! 
//! #[derive(Serialize)]
//! struct Output {
//!     value: usize,
//!     name: String,
//! }
//! 
//! #[space]
//! fn main(input: Input) -> Result<Output> {
//!     let output = Output {
//!         value: input.value * 2,
//!         name: input.name.chars().rev().collect(),
//!     };
//!     Ok(output)
//! }
//! ```
//!
//! ## HTTP client
//! 
//! ```rust
//! use space_lib::Request;
//! 
//! let body = Request::get("https://www.spaceoperator.com")
//!     .call()?
//!     .into_string()?;
//! ```

// Modules
pub mod common;
mod error;
mod ffi;
mod http;

// Exports
pub use error::{Error, Result};
pub use http::{Request, Response};

// Macro
pub use space_macro::space;

#[repr(C)]
pub struct SpaceSlice {
    pub len: usize,
    pub ptr: *mut u8,
}
