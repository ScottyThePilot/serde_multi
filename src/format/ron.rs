use serde::de::{Deserialize, DeserializeOwned};
use serde::ser::Serialize;
use std::io::{Read, Write};

use crate::traits::{Serde, SerdeBytes, SerdePretty, SerdeStream, SerdeText};

#[derive(Copy, Clone)]
pub struct Ron;

function!(to_string_pretty, |value| serde_ron::ser::to_string_pretty(value, pretty()));
function!(to_string, serde_ron::ser::to_string);
function!(from_str, serde_ron::de::from_str);
function!(to_vec_pretty, |value| serde_ron::ser::to_string_pretty(value, pretty()).map(String::into_bytes));
function!(to_vec, |value| serde_ron::ser::to_string(value).map(String::into_bytes));
function!(from_slice, serde_ron::de::from_bytes);
function!(to_writer_pretty, |writer, value| serde_ron::ser::to_writer_pretty(writer, value, pretty()));
function!(to_writer, serde_ron::ser::to_writer);
function!(from_reader, serde_ron::de::from_reader);

implement!(Ron, SerdeTextPretty);
implement!(Ron, SerdeBytesPretty);
implement!(Ron, SerdeStreamPretty);
implement!(Ron, SerdePretty);
implement!(Ron, Serde);

error_struct!(serde_ron::error::Error);

#[doc(hide)]
impl From<serde_ron::error::Error> for Error {
  #[inline]
  fn from(inner: serde_ron::error::Error) -> Error {
    Error { inner }
  }
}

#[inline]
fn pretty() -> serde_ron::ser::PrettyConfig {
  serde_ron::ser::PrettyConfig::new().with_indentor("  ".to_owned())
}
