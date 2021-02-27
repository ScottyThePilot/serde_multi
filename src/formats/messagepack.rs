//! MessagePack serialization/deserialization, courtesy of the [`rmp`] and [`rmp-serde`] crate.
//! 
//! [`rmp`]: https://crates.io/crates/rmp
//! [`rmp-serde`]: https://crates.io/crates/rmp-serde

use serde::de::{Deserialize, DeserializeOwned};
use serde::ser::Serialize;
use std::io::{Read, Write};

use crate::traits::{Serde, SerdeBytes, SerdeStream};

#[derive(Copy, Clone)]
pub struct MessagePack;

function!(to_vec, MessagePackError, |value| rmp_serde::to_vec(value));
function!(from_slice, MessagePackError, |data| rmp_serde::from_slice(data));
function!(to_writer, MessagePackError, |writer, value| {
  let mut writer = writer;
  rmp_serde::encode::write(&mut writer, value)
});
function!(from_reader, MessagePackError, |reader| rmp_serde::decode::from_read(reader));

implement!(MessagePack, SerdeBytes);
implement!(MessagePack, SerdeStream);
implement!(MessagePack, Serde, MessagePackError);

error_struct!(InnerError, MessagePackError);

#[doc(hidden)]
impl From<rmp_serde::encode::Error> for MessagePackError {
  #[inline]
  fn from(inner: rmp_serde::encode::Error) -> MessagePackError {
    MessagePackError {
      inner: InnerError::Encode(inner)
    }
  }
}

#[doc(hidden)]
impl From<rmp_serde::decode::Error> for MessagePackError {
  #[inline]
  fn from(inner: rmp_serde::decode::Error) -> MessagePackError {
    MessagePackError {
      inner: InnerError::Decode(inner)
    }
  }
}

enum InnerError {
  Encode(rmp_serde::encode::Error),
  Decode(rmp_serde::decode::Error)
}

impl std::fmt::Debug for InnerError {
  #[inline]
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      InnerError::Encode(err) => write!(f, "{:?}", err),
      InnerError::Decode(err) => write!(f, "{:?}", err)
    }
  }
}

impl std::fmt::Display for InnerError {
  #[inline]
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      InnerError::Encode(err) => write!(f, "{}", err),
      InnerError::Decode(err) => write!(f, "{}", err)
    }
  }
}

impl std::error::Error for InnerError {}
