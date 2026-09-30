// The first Markdown link definition wins; use local HTML paths in Rustdoc.
#![doc = "[guide]: guide/index.html\n\n"]
#![doc = include_str!("../README.md")]
#![forbid(unsafe_code)]
#![warn(missing_docs)]

/// Client configuration and asynchronous endpoint methods.
pub mod client;
/// Error types returned by the library.
pub mod error;
/// Typed API responses and their JSON field mappings.
pub mod types;

#[doc = "[readme]: ../index.html\n\n"]
#[doc = include_str!("../documentation.md")]
pub mod guide {}

pub use client::{Client, ClientBuilder};
pub use error::{Error, Result};
pub use types::*;
