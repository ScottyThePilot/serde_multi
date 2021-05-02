#![deny(missing_debug_implementations)]
//! This library exposes a standardized API across a number of file formats,
//! [traits] to make this API easily implementable, and a [`Format`] enum for
//! dynamically choosing the file format for Serialization/Deserialization.
//! This crate does not do any substantial extra work, it simply re-organizes
//! the APIs of a number of other serde file format crates.
//!
//! Formats can be chosen dynamically with generics using the traits, or with dynamic dispatch.
//! All file formats here implement ``
//!
//! Each file format is toggled via feature, all of which are disabled by default.
//!
//! Currently the only supported file formats are:
//! - [Bincode]
//! - [CBOR]
//! - [JSON]
//! - [MessagePack]
//! - [RON]
//! - [TOML]
//! - [XML]
//!
//! ## Example Usage
//! ```rust
//! use serde_multi::SerdeText;
//! use serde_multi::formats::json::Json;
//! use std::collections::HashMap;
//!
//! let mut value: HashMap<String, i32> = HashMap::new();
//! value.insert("foo".to_owned(), -193);
//! value.insert("bar".to_owned(), 3058);
//!
//! // `Json` can be swapped out for any value that implements `SerdeText` here
//! let s = Json.to_string(&value).expect("failed to serialize");
//! println!("serialized: {}", s);
//! ```
//!
//! [traits]: ./traits/index.html
//! [`Format`]: ./multi/enum.Format.html
//! [Bincode]: ./formats/bincode/index.html
//! [CBOR]: ./formats/cbor/index.html
//! [JSON]: ./formats/json/index.html
//! [MessagePack]: ./formats/messagepack/index.html
//! [RON]: ./formats/ron/index.html
//! [TOML]: ./formats/toml/index.html
//! [XML]: ./formats/xml/index.html

#[macro_use]
mod macros;

pub mod formats;
pub mod multi;
pub mod traits;

pub use multi::{Format, FormatError};
pub use traits::{SerdeBytes, SerdeStream, SerdeText};

pub type Error = Box<dyn std::error::Error + Send + Sync + 'static>;
