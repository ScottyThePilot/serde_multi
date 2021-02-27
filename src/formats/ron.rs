//! RON serialization/deserialization, courtesy of the [`ron`] crate.
//! 
//! [`ron`]: https://crates.io/crates/ron

use serde::de::{Deserialize, DeserializeOwned};
use serde::ser::Serialize;
use std::io::{Read, Write};

use crate::traits::{Serde, SerdeBytes, SerdePretty, SerdeStream, SerdeText};

#[derive(Copy, Clone)]
pub struct Ron;

function!(to_string_pretty, RonError, |value| serde_ron::ser::to_string_pretty(value, pretty()));
function!(to_string, RonError, serde_ron::ser::to_string);
function!(from_str, RonError, serde_ron::de::from_str);
function!(to_vec_pretty, RonError, |value| serde_ron::ser::to_string_pretty(value, pretty()).map(String::into_bytes));
function!(to_vec, RonError, |value| serde_ron::ser::to_string(value).map(String::into_bytes));
function!(from_slice, RonError, serde_ron::de::from_bytes);
function!(to_writer_pretty, RonError, |writer, value| serde_ron::ser::to_writer_pretty(writer, value, pretty()));
function!(to_writer, RonError, serde_ron::ser::to_writer);
function!(from_reader, RonError, serde_ron::de::from_reader);

implement!(Ron, SerdeTextPretty);
implement!(Ron, SerdeBytesPretty);
implement!(Ron, SerdeStreamPretty);
implement!(Ron, SerdePretty);
implement!(Ron, Serde, RonError);

error_struct!(serde_ron::error::Error, RonError);

#[doc(hidden)]
impl From<serde_ron::error::Error> for RonError {
  #[inline]
  fn from(inner: serde_ron::error::Error) -> RonError {
    RonError { inner }
  }
}

#[inline]
fn pretty() -> serde_ron::ser::PrettyConfig {
  serde_ron::ser::PrettyConfig::new().with_indentor("  ".to_owned())
}
