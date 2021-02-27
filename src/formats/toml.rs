//! TOML serialization/deserialization, courtesy of the [`toml`] crate.
//! 
//! Note: the `Toml` struct does not implement [`SerdeStream`]; the [`toml`] crate
//! doesn't expose any functions for stream serialization/deserialization for some reason.
//! 
//! [`toml`]: https://crates.io/crates/toml
//! [`SerdeStream`]: ../../traits/trait.SerdeStream.html

use serde::de::Deserialize;
use serde::ser::Serialize;

use crate::traits::{Serde, SerdeBytes, SerdePretty, SerdeText};

#[derive(Copy, Clone)]
pub struct Toml;

function!(to_string_pretty, TomlError, serde_toml::to_string_pretty);
function!(to_string, TomlError, serde_toml::to_string);
function!(from_str, TomlError, serde_toml::from_str);
function!(to_vec_pretty, TomlError, |value| serde_toml::to_string_pretty(value).map(String::into_bytes));
function!(to_vec, TomlError, serde_toml::to_vec);
function!(from_slice, TomlError, serde_toml::from_slice);

implement!(Toml, SerdeTextPretty);
implement!(Toml, SerdeBytesPretty);
implement!(Toml, SerdePretty);
implement!(Toml, Serde, TomlError);

error_struct!(InnerError, TomlError);

#[doc(hidden)]
impl From<serde_toml::ser::Error> for TomlError {
  #[inline]
  fn from(inner: serde_toml::ser::Error) -> TomlError {
    TomlError {
      inner: InnerError::Ser(inner)
    }
  }
}

#[doc(hidden)]
impl From<serde_toml::de::Error> for TomlError {
  #[inline]
  fn from(inner: serde_toml::de::Error) -> TomlError {
    TomlError {
      inner: InnerError::De(inner)
    }
  }
}

enum InnerError {
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
