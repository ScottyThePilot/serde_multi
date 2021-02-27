//! JSON serialization/deserialization, courtesy of the [`serde_json`] crate.
//! 
//! [`serde_json`]: https://crates.io/crates/serde_json

use serde::de::{Deserialize, DeserializeOwned};
use serde::ser::Serialize;
use std::io::{Read, Write};

use crate::traits::{Serde, SerdeBytes, SerdePretty, SerdeStream, SerdeText};

#[derive(Copy, Clone)]
pub struct Json;

function!(to_string_pretty, JsonError, serde_json::to_string_pretty);
function!(to_string, JsonError, serde_json::to_string);
function!(from_str, JsonError, serde_json::from_str);
function!(to_vec_pretty, JsonError, serde_json::to_vec_pretty);
function!(to_vec, JsonError, serde_json::to_vec);
function!(from_slice, JsonError, serde_json::from_slice);
function!(to_writer_pretty, JsonError, serde_json::to_writer_pretty);
function!(to_writer, JsonError, serde_json::to_writer);
function!(from_reader, JsonError, serde_json::from_reader);

implement!(Json, SerdeTextPretty);
implement!(Json, SerdeBytesPretty);
implement!(Json, SerdeStreamPretty);
implement!(Json, SerdePretty);
implement!(Json, Serde, JsonError);

error_struct!(serde_json::Error, JsonError);

#[doc(hidden)]
impl From<serde_json::Error> for JsonError {
  #[inline]
  fn from(inner: serde_json::Error) -> JsonError {
    JsonError { inner }
  }
}
