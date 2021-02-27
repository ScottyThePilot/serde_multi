use serde::de::{Deserialize, DeserializeOwned};
use serde::ser::Serialize;
use std::io::{Read, Write};

#[cfg(feature = "bincode")]
use crate::format::bincode;
#[cfg(feature = "cbor")]
use crate::format::cbor;
#[cfg(feature = "json")]
use crate::format::json;
#[cfg(feature = "messagepack")]
use crate::format::messagepack;
#[cfg(feature = "ron")]
use crate::format::ron;
#[cfg(feature = "toml")]
use crate::format::toml;
#[cfg(feature = "xml")]
use crate::format::xml;

use crate::traits::{Serde, SerdeBytes, SerdePretty, SerdeStream, SerdeText};

#[non_exhaustive]
#[derive(Copy, Clone)]
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

impl SerdeText for Format {
  #[inline]
  fn to_string_pretty<T>(&self, value: &T) -> Result<String, Self::Error>
  where T: Serialize, Self: SerdePretty {
    match self {
      #[cfg(feature = "json")]
      Format::Json => json::to_string_pretty(value).map_err(From::from),
      #[cfg(feature = "ron")]
      Format::Ron => ron::to_string_pretty(value).map_err(From::from),
      #[cfg(feature = "toml")]
      Format::Toml => toml::to_string_pretty(value).map_err(From::from),
      #[allow(unreachable_patterns)]
      _ => Err(FormatError::Unsupported)
    }
  }

  #[inline]
  fn to_string<T>(&self, value: &T) -> Result<String, Self::Error>
  where T: Serialize {
    match self {
      #[cfg(feature = "json")]
      Format::Json => json::to_string(value).map_err(From::from),
      #[cfg(feature = "ron")]
      Format::Ron => ron::to_string(value).map_err(From::from),
      #[cfg(feature = "toml")]
      Format::Toml => toml::to_string(value).map_err(From::from),
      #[cfg(feature = "xml")]
      Format::Xml => xml::to_string(value).map_err(From::from),
      #[allow(unreachable_patterns)]
      _ => Err(FormatError::Unsupported)
    }
  }

  #[inline]
  fn from_str<'d, T>(&self, data: &'d str) -> Result<T, Self::Error>
  where T: Deserialize<'d> {
    match self {
      #[cfg(feature = "json")]
      Format::Json => json::from_str(data).map_err(From::from),
      #[cfg(feature = "ron")]
      Format::Ron => ron::from_str(data).map_err(From::from),
      #[cfg(feature = "toml")]
      Format::Toml => toml::from_str(data).map_err(From::from),
      #[cfg(feature = "xml")]
      Format::Xml => xml::from_str(data).map_err(From::from),
      #[allow(unreachable_patterns)]
      _ => Err(FormatError::Unsupported)
    }
  }
}

impl SerdeBytes for Format {
  fn to_vec_pretty<T>(&self, value: &T) -> Result<Vec<u8>, Self::Error>
  where T: Serialize, Self: SerdePretty {
    match self {
      #[cfg(feature = "json")]
      Format::Json => json::to_vec_pretty(value).map_err(From::from),
      #[cfg(feature = "ron")]
      Format::Ron => ron::to_vec_pretty(value).map_err(From::from),
      #[cfg(feature = "toml")]
      Format::Toml => toml::to_vec_pretty(value).map_err(From::from),
      #[allow(unreachable_patterns)]
      _ => Err(FormatError::Unsupported)
    }
  }

  fn to_vec<T>(&self, value: &T) -> Result<Vec<u8>, Self::Error>
  where T: Serialize {
    match self {
      #[cfg(feature = "bincode")]
      Format::Bincode => bincode::to_vec(value).map_err(From::from),
      #[cfg(feature = "cbor")]
      Format::Cbor => cbor::to_vec(value).map_err(From::from),
      #[cfg(feature = "json")]
      Format::Json => json::to_vec(value).map_err(From::from),
      #[cfg(feature = "messagepack")]
      Format::MessagePack => messagepack::to_vec(value).map_err(From::from),
      #[cfg(feature = "ron")]
      Format::Ron => ron::to_vec(value).map_err(From::from),
      #[cfg(feature = "toml")]
      Format::Toml => toml::to_vec(value).map_err(From::from),
      #[cfg(feature = "xml")]
      Format::Xml => xml::to_vec(value).map_err(From::from),
      #[allow(unreachable_patterns)]
      _ => Err(FormatError::Unsupported)
    }
  }

  fn from_slice<'d, T>(&self, data: &'d [u8]) -> Result<T, Self::Error>
  where T: Deserialize<'d> {
    match self {
      #[cfg(feature = "bincode")]
      Format::Bincode => bincode::from_slice(data).map_err(From::from),
      #[cfg(feature = "cbor")]
      Format::Cbor => cbor::from_slice(data).map_err(From::from),
      #[cfg(feature = "json")]
      Format::Json => json::from_slice(data).map_err(From::from),
      #[cfg(feature = "messagepack")]
      Format::MessagePack => messagepack::from_slice(data).map_err(From::from),
      #[cfg(feature = "ron")]
      Format::Ron => ron::from_slice(data).map_err(From::from),
      #[cfg(feature = "toml")]
      Format::Toml => toml::from_slice(data).map_err(From::from),
      #[cfg(feature = "xml")]
      Format::Xml => xml::from_slice(data).map_err(From::from),
      #[allow(unreachable_patterns)]
      _ => Err(FormatError::Unsupported)
    }
  }
}

impl SerdeStream for Format {
  fn to_writer_pretty<W, T>(&self, writer: W, value: &T) -> Result<(), Self::Error>
  where W: Write, T: Serialize, Self: SerdePretty {
    match self {
      #[cfg(feature = "json")]
      Format::Json => json::to_writer_pretty(writer, value).map_err(From::from),
      #[cfg(feature = "ron")]
      Format::Ron => ron::to_writer_pretty(writer, value).map_err(From::from),
      #[allow(unreachable_patterns)]
      _ => Err(FormatError::Unsupported)
    }
  }

  fn to_writer<W, T>(&self, writer: W, value: &T) -> Result<(), Self::Error>
  where W: Write, T: Serialize {
    match self {
      #[cfg(feature = "bincode")]
      Format::Bincode => bincode::to_writer(writer, value).map_err(From::from),
      #[cfg(feature = "cbor")]
      Format::Cbor => cbor::to_writer(writer, value).map_err(From::from),
      #[cfg(feature = "json")]
      Format::Json => json::to_writer(writer, value).map_err(From::from),
      #[cfg(feature = "messagepack")]
      Format::MessagePack => messagepack::to_writer(writer, value).map_err(From::from),
      #[cfg(feature = "ron")]
      Format::Ron => ron::to_writer(writer, value).map_err(From::from),
      #[cfg(feature = "xml")]
      Format::Xml => xml::to_writer(writer, value).map_err(From::from),
      #[allow(unreachable_patterns)]
      _ => Err(FormatError::Unsupported)
    }
  }

  fn from_reader<R, T>(&self, reader: R) -> Result<T, Self::Error>
  where R: Read, T: DeserializeOwned {
    match self {
      #[cfg(feature = "bincode")]
      Format::Bincode => bincode::from_reader(reader).map_err(From::from),
      #[cfg(feature = "cbor")]
      Format::Cbor => cbor::from_reader(reader).map_err(From::from),
      #[cfg(feature = "json")]
      Format::Json => json::from_reader(reader).map_err(From::from),
      #[cfg(feature = "messagepack")]
      Format::MessagePack => messagepack::from_reader(reader).map_err(From::from),
      #[cfg(feature = "ron")]
      Format::Ron => ron::from_reader(reader).map_err(From::from),
      #[cfg(feature = "xml")]
      Format::Xml => xml::from_reader(reader).map_err(From::from),
      #[allow(unreachable_patterns)]
      _ => Err(FormatError::Unsupported)
    }
  }
}

impl SerdePretty for Format {}

impl Serde for Format {
  type Error = FormatError;
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

#[non_exhaustive]
pub enum FormatError {
  #[cfg(feature = "bincode")]
  Bincode(bincode::Error),
  #[cfg(feature = "cbor")]
  Cbor(cbor::Error),
  #[cfg(feature = "json")]
  Json(json::Error),
  #[cfg(feature = "messagepack")]
  MessagePack(messagepack::Error),
  #[cfg(feature = "ron")]
  Ron(ron::Error),
  #[cfg(feature = "toml")]
  Toml(toml::Error),
  #[cfg(feature = "xml")]
  Xml(xml::Error),
  Unsupported
}

#[cfg(feature = "bincode")]
impl From<bincode::Error> for FormatError {
  #[inline]
  fn from(error: bincode::Error) -> FormatError {
    FormatError::Bincode(error)
  }
}

#[cfg(feature = "cbor")]
impl From<cbor::Error> for FormatError {
  #[inline]
  fn from(error: cbor::Error) -> FormatError {
    FormatError::Cbor(error)
  }
}

#[cfg(feature = "json")]
impl From<json::Error> for FormatError {
  #[inline]
  fn from(error: json::Error) -> FormatError {
    FormatError::Json(error)
  }
}

#[cfg(feature = "messagepack")]
impl From<messagepack::Error> for FormatError {
  #[inline]
  fn from(error: messagepack::Error) -> FormatError {
    FormatError::MessagePack(error)
  }
}

#[cfg(feature = "ron")]
impl From<ron::Error> for FormatError {
  #[inline]
  fn from(error: ron::Error) -> FormatError {
    FormatError::Ron(error)
  }
}

#[cfg(feature = "toml")]
impl From<toml::Error> for FormatError {
  #[inline]
  fn from(error: toml::Error) -> FormatError {
    FormatError::Toml(error)
  }
}

#[cfg(feature = "xml")]
impl From<xml::Error> for FormatError {
  #[inline]
  fn from(error: xml::Error) -> FormatError {
    FormatError::Xml(error)
  }
}
