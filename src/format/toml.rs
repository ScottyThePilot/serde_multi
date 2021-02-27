use serde::de::Deserialize;
use serde::ser::Serialize;

use crate::traits::{Serde, SerdeBytes, SerdePretty, SerdeText};

#[derive(Copy, Clone)]
pub struct Toml;

function!(to_string_pretty, serde_toml::to_string_pretty);
function!(to_string, serde_toml::to_string);
function!(from_str, serde_toml::from_str);
function!(to_vec_pretty, |value| serde_toml::to_string_pretty(value).map(String::into_bytes));
function!(to_vec, serde_toml::to_vec);
function!(from_slice, serde_toml::from_slice);

implement!(Toml, SerdeTextPretty);
implement!(Toml, SerdeBytesPretty);
implement!(Toml, SerdePretty);
implement!(Toml, Serde);

error_struct!(InnerError);

#[doc(hide)]
impl From<serde_toml::ser::Error> for Error {
  #[inline]
  fn from(inner: serde_toml::ser::Error) -> Error {
    Error {
      inner: InnerError::Ser(inner)
    }
  }
}

#[doc(hide)]
impl From<serde_toml::de::Error> for Error {
  #[inline]
  fn from(inner: serde_toml::de::Error) -> Error {
    Error {
      inner: InnerError::De(inner)
    }
  }
}

pub enum InnerError {
  Ser(serde_toml::ser::Error),
  De(serde_toml::de::Error)
}

impl std::fmt::Debug for InnerError {
  #[inline]
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      InnerError::Ser(err) => write!(f, "{:?}", err),
      InnerError::De(err) => write!(f, "{:?}", err)
    }
  }
}

impl std::fmt::Display for InnerError {
  #[inline]
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      InnerError::Ser(err) => write!(f, "{}", err),
      InnerError::De(err) => write!(f, "{}", err)
    }
  }
}

impl std::error::Error for InnerError {}
