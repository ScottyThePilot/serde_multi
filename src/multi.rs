//! Utility enum for dynamically picking the data format.
//! 
//! Note that the functions here return [`FormatError`] instead of the usual [`Error`].
//! Use these functions if you would like to be able to detect when a feature is unsupported.
//! 
//! [`FormatError`]: ./enum.FormatError.html
//! [`Error`]: ../type.Error.html

use serde::de::{Deserialize, DeserializeOwned};
use serde::ser::Serialize;
use std::fmt::{self, Debug, Display};
use std::io::{Read, Write};

#[cfg(feature = "bincode")]
use crate::formats::bincode;
#[cfg(feature = "cbor")]
use crate::formats::cbor;
#[cfg(feature = "json")]
use crate::formats::json;
#[cfg(feature = "messagepack")]
use crate::formats::messagepack;
#[cfg(feature = "ron")]
use crate::formats::ron;
#[cfg(feature = "toml")]
use crate::formats::toml;
#[cfg(feature = "xml")]
use crate::formats::xml;

use crate::traits::{SerdeText, SerdeBytes, SerdeStream};

/// Dynamically pick which format data is serialized from or deserialized into.
/// 
/// Note: calling the respective trait function for a format that does
/// not support it will return a [`FormatError::Unsupported`] `Err`
/// to indicate that that operation was not supported by the format.
/// 
/// [`FormatError::Unsupported`]: ./enum.FormatError.html#variant.Unsupported
#[non_exhaustive]
#[derive(Copy, Clone, Debug)]
pub enum Format {
  #[cfg(feature = "bincode")]
  Bincode,
  #[cfg(feature = "cbor")]
  Cbor,
  #[cfg(feature = "json")]
  Json,
  #[cfg(feature = "messagepack")]
  MessagePack,
  #[cfg(feature = "ron")]
  Ron,
  #[cfg(feature = "toml")]
  Toml,
  #[cfg(feature = "xml")]
  Xml
}

impl Format {
  /// Gets the name of the format currently being used.
  pub fn name(&self) -> &'static str {
    match self {
      #[cfg(feature = "bincode")]
      Format::Bincode => "Bincode",
      #[cfg(feature = "cbor")]
      Format::Cbor => "CBOR",
      #[cfg(feature = "json")]
      Format::Json => "JSON",
      #[cfg(feature = "messagepack")]
      Format::MessagePack => "MessagePack",
      #[cfg(feature = "ron")]
      Format::Ron => "RON",
      #[cfg(feature = "toml")]
      Format::Toml => "TOML",
      #[cfg(feature = "xml")]
      Format::Xml => "XML",
      #[allow(unreachable_patterns)]
      _ => "unsupported"
    }
  }

  /// Converts this format into a `dyn SerdeBytes`.
  pub fn as_dyn_bytes(&self) -> &'static dyn SerdeBytes {
    match self {
      #[cfg(feature = "bincode")]
      Format::Bincode => &bincode::Bincode,
      #[cfg(feature = "cbor")]
      Format::Cbor => &cbor::Cbor,
      #[cfg(feature = "json")]
      Format::Json => &json::Json,
      #[cfg(feature = "messagepack")]
      Format::MessagePack => &messagepack::MessagePack,
      #[cfg(feature = "ron")]
      Format::Ron => &ron::Ron,
      #[cfg(feature = "toml")]
      Format::Toml => &toml::Toml,
      #[cfg(feature = "xml")]
      Format::Xml => &xml::Xml,
      #[allow(unreachable_patterns)]
      _ => panic!("unsupported")
    }
  }

  /// Converts this format into a `dyn SerdeText`.
  pub fn as_dyn_text(&self) -> &'static dyn SerdeText {
    match self {
      #[cfg(feature = "json")]
      Format::Json => &json::Json,
      #[cfg(feature = "ron")]
      Format::Ron => &ron::Ron,
      #[cfg(feature = "toml")]
      Format::Toml => &toml::Toml,
      #[cfg(feature = "xml")]
      Format::Xml => &xml::Xml,
      #[allow(unreachable_patterns)]
      _ => panic!("unsupported")
    }
  }

  /// Converts this format into a `dyn SerdeStream`
  pub fn as_dyn_stream(&self) -> &'static dyn SerdeStream {
    match self {
      #[cfg(feature = "bincode")]
      Format::Bincode => &bincode::Bincode,
      #[cfg(feature = "cbor")]
      Format::Cbor => &cbor::Cbor,
      #[cfg(feature = "json")]
      Format::Json => &json::Json,
      #[cfg(feature = "messagepack")]
      Format::MessagePack => &messagepack::MessagePack,
      #[cfg(feature = "ron")]
      Format::Ron => &ron::Ron,
      #[cfg(feature = "toml")]
      Format::Toml => &toml::Toml,
      #[cfg(feature = "xml")]
      Format::Xml => &xml::Xml,
      #[allow(unreachable_patterns)]
      _ => panic!("unsupported")
    }
  }
}

impl SerdeText for Format {
  #[inline]
  fn to_string_pretty<T>(&self, value: &T) -> Result<String, crate::Error>
  where T: Serialize {
    to_string_pretty(*self, value).map_err(map_err)
  }

  #[inline]
  fn to_string<T>(&self, value: &T) -> Result<String, crate::Error>
  where T: Serialize {
    to_string(*self, value).map_err(map_err)
  }

  #[inline]
  fn from_str<'d, T>(&self, data: &'d str) -> Result<T, crate::Error>
  where T: Deserialize<'d> {
    from_str(*self, data).map_err(map_err)
  }
}

impl SerdeBytes for Format {
  fn to_vec_pretty<T>(&self, value: &T) -> Result<Vec<u8>, crate::Error>
  where T: Serialize {
    to_vec_pretty(*self, value).map_err(map_err)
  }

  fn to_vec<T>(&self, value: &T) -> Result<Vec<u8>, crate::Error>
  where T: Serialize {
    to_vec(*self, value).map_err(map_err)
  }

  fn from_slice<'d, T>(&self, data: &'d [u8]) -> Result<T, crate::Error>
  where T: Deserialize<'d> {
    from_slice(*self, data).map_err(map_err)
  }
}

impl SerdeStream for Format {
  fn to_writer_pretty<W, T>(&self, writer: W, value: &T) -> Result<(), crate::Error>
  where W: Write, T: Serialize {
    to_writer_pretty(*self, writer, value).map_err(map_err)
  }

  fn to_writer<W, T>(&self, writer: W, value: &T) -> Result<(), crate::Error>
  where W: Write, T: Serialize {
    to_writer(*self, writer, value).map_err(map_err)
  }

  fn from_reader<R, T>(&self, reader: R) -> Result<T, crate::Error>
  where R: Read, T: DeserializeOwned {
    from_reader(*self, reader).map_err(map_err)
  }
}

impl Display for Format {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.name())
  }
}

#[cfg(feature = "bincode")]
impl From<bincode::Bincode> for Format {
  #[inline]
  fn from(_: bincode::Bincode) -> Format {
    Format::Bincode
  }
}

#[cfg(feature = "cbor")]
impl From<cbor::Cbor> for Format {
  #[inline]
  fn from(_: cbor::Cbor) -> Format {
    Format::Cbor
  }
}

#[cfg(feature = "json")]
impl From<json::Json> for Format {
  #[inline]
  fn from(_: json::Json) -> Format {
    Format::Json
  }
}

#[cfg(feature = "messagepack")]
impl From<messagepack::MessagePack> for Format {
  #[inline]
  fn from(_: messagepack::MessagePack) -> Format {
    Format::MessagePack
  }
}

#[cfg(feature = "ron")]
impl From<ron::Ron> for Format {
  #[inline]
  fn from(_: ron::Ron) -> Format {
    Format::Ron
  }
}

#[cfg(feature = "toml")]
impl From<toml::Toml> for Format {
  #[inline]
  fn from(_: toml::Toml) -> Format {
    Format::Toml
  }
}

#[cfg(feature = "xml")]
impl From<xml::Xml> for Format {
  #[inline]
  fn from(_: xml::Xml) -> Format {
    Format::Xml
  }
}

pub enum FormatError {
  Error(crate::Error),
  Unsupported(Unsupported)
}

impl Debug for FormatError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      FormatError::Error(error) => write!(f, "Error({:?})", error),
      FormatError::Unsupported(unsupported) => write!(f, "{:?}", unsupported)
    }
  }
}

impl Display for FormatError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      FormatError::Error(error) => Display::fmt(error, f),
      FormatError::Unsupported(unsupported) => write!(f, "{}", unsupported)
    }
  }
}

impl std::error::Error for FormatError {}

pub enum Feature {
  Text,
  Bytes,
  Stream,
  Pretty
}

impl Display for Feature {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Feature::Text => write!(f, "text serialization/deserialization"),
      Feature::Bytes => write!(f, "binary serialization/deserialization"),
      Feature::Stream => write!(f, "stream serialization/deserialization"),
      Feature::Pretty => write!(f, "pretty-print serialization")
    }
  }
}

pub struct Unsupported {
  pub format: Format,
  pub feature: Feature
}

impl Debug for Unsupported {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "Unsupported({}, {})", self.format, self.feature)
  }
}

impl Display for Unsupported {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{} does not support {}", self.format.name(), self.feature)
  }
}

impl std::error::Error for Unsupported {}

fn map_err(format_error: FormatError) -> crate::Error {
  match format_error {
    FormatError::Error(error) => error,
    FormatError::Unsupported(unsupported) => Box::new(unsupported)
  }
}

#[inline]
fn unsupported(format: Format, feature: Feature) -> FormatError {
  FormatError::Unsupported(Unsupported { format, feature })
}

pub fn to_string_pretty<T>(format: Format, value: &T) -> Result<String, FormatError>
where T: Serialize {
  match format {
    #[cfg(feature = "json")]
    Format::Json => json::to_string_pretty(value).map_err(FormatError::Error),
    #[cfg(feature = "ron")]
    Format::Ron => ron::to_string_pretty(value).map_err(FormatError::Error),
    #[cfg(feature = "toml")]
    Format::Toml => toml::to_string_pretty(value).map_err(FormatError::Error),
    #[allow(unreachable_patterns)]
    _ => Err(unsupported(format, Feature::Pretty))
  }
}

pub fn to_string<T>(format: Format, value: &T) -> Result<String, FormatError>
where T: Serialize {
  match format {
    #[cfg(feature = "json")]
    Format::Json => json::to_string(value).map_err(FormatError::Error),
    #[cfg(feature = "ron")]
    Format::Ron => ron::to_string(value).map_err(FormatError::Error),
    #[cfg(feature = "toml")]
    Format::Toml => toml::to_string(value).map_err(FormatError::Error),
    #[cfg(feature = "xml")]
    Format::Xml => xml::to_string(value).map_err(FormatError::Error),
    #[allow(unreachable_patterns)]
    _ => Err(unsupported(format, Feature::Text))
  }
}

pub fn from_str<'d, T>(format: Format, data: &'d str) -> Result<T, FormatError>
where T: Deserialize<'d> {
  match format {
    #[cfg(feature = "json")]
    Format::Json => json::from_str(data).map_err(FormatError::Error),
    #[cfg(feature = "ron")]
    Format::Ron => ron::from_str(data).map_err(FormatError::Error),
    #[cfg(feature = "toml")]
    Format::Toml => toml::from_str(data).map_err(FormatError::Error),
    #[cfg(feature = "xml")]
    Format::Xml => xml::from_str(data).map_err(FormatError::Error),
    #[allow(unreachable_patterns)]
    _ => Err(unsupported(format, Feature::Text))
  }
}

pub fn to_vec_pretty<T>(format: Format, value: &T) -> Result<Vec<u8>, FormatError>
where T: Serialize {
  match format {
    #[cfg(feature = "json")]
    Format::Json => json::to_vec_pretty(value).map_err(FormatError::Error),
    #[cfg(feature = "ron")]
    Format::Ron => ron::to_vec_pretty(value).map_err(FormatError::Error),
    #[cfg(feature = "toml")]
    Format::Toml => toml::to_vec_pretty(value).map_err(FormatError::Error),
    #[allow(unreachable_patterns)]
    _ => Err(unsupported(format, Feature::Pretty))
  }
}

pub fn to_vec<T>(format: Format, value: &T) -> Result<Vec<u8>, FormatError>
where T: Serialize {
  match format {
    #[cfg(feature = "bincode")]
    Format::Bincode => bincode::to_vec(value).map_err(FormatError::Error),
    #[cfg(feature = "cbor")]
    Format::Cbor => cbor::to_vec(value).map_err(FormatError::Error),
    #[cfg(feature = "json")]
    Format::Json => json::to_vec(value).map_err(FormatError::Error),
    #[cfg(feature = "messagepack")]
    Format::MessagePack => messagepack::to_vec(value).map_err(FormatError::Error),
    #[cfg(feature = "ron")]
    Format::Ron => ron::to_vec(value).map_err(FormatError::Error),
    #[cfg(feature = "toml")]
    Format::Toml => toml::to_vec(value).map_err(FormatError::Error),
    #[cfg(feature = "xml")]
    Format::Xml => xml::to_vec(value).map_err(FormatError::Error),
    #[allow(unreachable_patterns)]
    _ => Err(unsupported(format, Feature::Bytes))
  }
}

pub fn from_slice<'d, T>(format: Format, data: &'d [u8]) -> Result<T, FormatError>
where T: Deserialize<'d> {
  match format {
    #[cfg(feature = "bincode")]
    Format::Bincode => bincode::from_slice(data).map_err(FormatError::Error),
    #[cfg(feature = "cbor")]
    Format::Cbor => cbor::from_slice(data).map_err(FormatError::Error),
    #[cfg(feature = "json")]
    Format::Json => json::from_slice(data).map_err(FormatError::Error),
    #[cfg(feature = "messagepack")]
    Format::MessagePack => messagepack::from_slice(data).map_err(FormatError::Error),
    #[cfg(feature = "ron")]
    Format::Ron => ron::from_slice(data).map_err(FormatError::Error),
    #[cfg(feature = "toml")]
    Format::Toml => toml::from_slice(data).map_err(FormatError::Error),
    #[cfg(feature = "xml")]
    Format::Xml => xml::from_slice(data).map_err(FormatError::Error),
    #[allow(unreachable_patterns)]
    _ => Err(unsupported(format, Feature::Stream))
  }
}

pub fn to_writer_pretty<W, T>(format: Format, writer: W, value: &T) -> Result<(), FormatError>
where W: Write, T: Serialize {
  match format {
    #[cfg(feature = "json")]
    Format::Json => json::to_writer_pretty(writer, value).map_err(FormatError::Error),
    #[cfg(feature = "ron")]
    Format::Ron => ron::to_writer_pretty(writer, value).map_err(FormatError::Error),
    #[cfg(feature = "toml")]
    Format::Toml => toml::to_writer_pretty(writer, value).map_err(FormatError::Error),
    #[allow(unreachable_patterns)]
    _ => Err(unsupported(format, Feature::Pretty))
  }
}

pub fn to_writer<W, T>(format: Format, writer: W, value: &T) -> Result<(), FormatError>
where W: Write, T: Serialize {
  match format {
    #[cfg(feature = "bincode")]
    Format::Bincode => bincode::to_writer(writer, value).map_err(FormatError::Error),
    #[cfg(feature = "cbor")]
    Format::Cbor => cbor::to_writer(writer, value).map_err(FormatError::Error),
    #[cfg(feature = "json")]
    Format::Json => json::to_writer(writer, value).map_err(FormatError::Error),
    #[cfg(feature = "messagepack")]
    Format::MessagePack => messagepack::to_writer(writer, value).map_err(FormatError::Error),
    #[cfg(feature = "ron")]
    Format::Ron => ron::to_writer(writer, value).map_err(FormatError::Error),
    #[cfg(feature = "toml")]
    Format::Toml => toml::to_writer(writer, value).map_err(FormatError::Error),
    #[cfg(feature = "xml")]
    Format::Xml => xml::to_writer(writer, value).map_err(FormatError::Error),
    #[allow(unreachable_patterns)]
    _ => Err(unsupported(format, Feature::Stream))
  }
}

pub fn from_reader<R, T>(format: Format, reader: R) -> Result<T, FormatError>
where R: Read, T: DeserializeOwned {
  match format {
    #[cfg(feature = "bincode")]
    Format::Bincode => bincode::from_reader(reader).map_err(FormatError::Error),
    #[cfg(feature = "cbor")]
    Format::Cbor => cbor::from_reader(reader).map_err(FormatError::Error),
    #[cfg(feature = "json")]
    Format::Json => json::from_reader(reader).map_err(FormatError::Error),
    #[cfg(feature = "messagepack")]
    Format::MessagePack => messagepack::from_reader(reader).map_err(FormatError::Error),
    #[cfg(feature = "ron")]
    Format::Ron => ron::from_reader(reader).map_err(FormatError::Error),
    #[cfg(feature = "toml")]
    Format::Toml => ron::from_reader(reader).map_err(FormatError::Error),
    #[cfg(feature = "xml")]
    Format::Xml => xml::from_reader(reader).map_err(FormatError::Error),
    #[allow(unreachable_patterns)]
    _ => Err(unsupported(format, Feature::Stream))
  }
}
