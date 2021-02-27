use serde::de::{Deserialize, DeserializeOwned};
use serde::ser::Serialize;
use std::io::{Read, Write};

use crate::traits::{Serde, SerdeBytes, SerdeStream};

#[derive(Copy, Clone)]
pub struct MessagePack;

function!(to_vec, |value| rmp_serde::to_vec(value));
function!(from_slice, |data| rmp_serde::from_slice(data));
function!(to_writer, |writer, value| {
  let mut writer = writer;
  rmp_serde::encode::write(&mut writer, value)
});
function!(from_reader, |reader| rmp_serde::decode::from_read(reader));

implement!(MessagePack, SerdeBytes);
implement!(MessagePack, SerdeStream);
implement!(MessagePack, Serde);

error_struct!(InnerError);

#[doc(hide)]
impl From<rmp_serde::encode::Error> for Error {
  #[inline]
  fn from(inner: rmp_serde::encode::Error) -> Error {
    Error {
      inner: InnerError::Encode(inner)
    }
  }
}

#[doc(hide)]
impl From<rmp_serde::decode::Error> for Error {
  #[inline]
  fn from(inner: rmp_serde::decode::Error) -> Error {
    Error {
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
