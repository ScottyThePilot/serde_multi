use serde::de::{Deserialize, DeserializeOwned};
use serde::ser::Serialize;
use std::io::{Read, Write};

use crate::traits::{Serde, SerdeBytes, SerdePretty, SerdeStream, SerdeText};

#[derive(Copy, Clone)]
pub struct Json;

function!(to_string_pretty, serde_json::to_string_pretty);
function!(to_string, serde_json::to_string);
function!(from_str, serde_json::from_str);
function!(to_vec_pretty, serde_json::to_vec_pretty);
function!(to_vec, serde_json::to_vec);
function!(from_slice, serde_json::from_slice);
function!(to_writer_pretty, serde_json::to_writer_pretty);
function!(to_writer, serde_json::to_writer);
function!(from_reader, serde_json::from_reader);

implement!(Json, SerdeTextPretty);
implement!(Json, SerdeBytesPretty);
implement!(Json, SerdeStreamPretty);
implement!(Json, SerdePretty);
implement!(Json, Serde);

error_struct!(serde_json::Error);

#[doc(hide)]
impl From<serde_json::Error> for Error {
  #[inline]
  fn from(inner: serde_json::Error) -> Error {
    Error { inner }
  }
}
