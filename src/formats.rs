//! Supported formats. Each module here is toggled by a feature of the same name.

#[cfg(feature = "bincode")]
pub mod bincode;
#[cfg(feature = "cbor")]
pub mod cbor;
#[cfg(feature = "json")]
pub mod json;
#[cfg(feature = "messagepack")]
pub mod messagepack;
#[cfg(feature = "ron")]
pub mod ron;
#[cfg(feature = "toml")]
pub mod toml;
#[cfg(feature = "xml")]
pub mod xml;

fn map_err<T: std::error::Error + Send + Sync + 'static>(err: T) -> crate::Error {
  Box::new(err)
}
