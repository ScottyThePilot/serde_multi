//! This library exposes a standardized API across a number of file formats,
//! [traits] to make this API easily implementable, and a [`Format`] enum for
//! dynamically choosing the file format for Serialization/Deserialization.
//! This crate does not do any substantial extra work, it simply re-organizes
//! the APIs of a number of other serde file format crates.
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

pub use multi::*;
pub use traits::*;
