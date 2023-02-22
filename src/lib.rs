//! This crate provides WebAssembly host functions for Space Operator.
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
//! 
//! ## Supabase
//! 
//! ```rust
//! use space_lib::Supabase;
//! 
//! let client = Supabase::new("https://hyjbiblkjrrvkzaqsyxe.supabase.co")
//!     .insert_header("apikey", "anon_api_key");
//! 
//! let rows = client
//!     .from("dogs")
//!     .select("name")
//!     .execute()?
//!     .into_string()?;
//! ```
//! 
//! ## Solana
//! 
//! ```rust
//! use space_lib::Solana;
//! 
//! let client = Solana::new("https://api.devnet.solana.com");
//! let balance = client.get_balance("base58_encoded_pubkey")?;
//! ```
//! 
//! ## Result
//! 
//! ```rust
//! use space_lib::Result;
//! 
//! #[no_mangle]
//! fn main() -> Result<u64> {
//!     Ok("123".parse()?)
//! }
//! ```

pub mod common;
mod error;
mod ffi;
mod http;
mod solana;
mod supabase;

pub use solana::Solana;
pub use serde_json::json;
pub use http::{Request, Response};
pub use supabase::{Supabase, Builder};
pub use error::{SpaceError, HostError};

// Error handling compatible with the space runtime
#[allow(dead_code)]
#[repr(transparent)]
pub struct Error(pub String);

impl Error {
    pub fn new<T: std::fmt::Display>(message: T) -> Self {
        Self(message.to_string())
    }
}

impl<T: std::fmt::Display> From<T> for Error {
    fn from(message: T) -> Self {
        Self(message.to_string())
    }
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
