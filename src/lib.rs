#![deny(missing_debug_implementations)]
//! This library exposes a standardized API across a number of file formats, as well as providing traits to make it
//! possible to dynamically choose file serialization/deserialization format at runtime or based on generics.
//!
//! Formats can be chosen dynamically with generics using the traits, or with dynamic dispatch.
//! All file formats here implement [`SerdeBytes`] and [`SerdeStream`].
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
//! [`SerdeBytes`]: ./traits/enum.SerdeBytes.html
//! [`SerdeStream`]: ./traits/enum.SerdeStream.html
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
