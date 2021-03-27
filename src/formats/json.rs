//! JSON serialization/deserialization, courtesy of the [`serde_json`] crate.
//! 
//! [`serde_json`]: https://crates.io/crates/serde_json

use serde::de::{Deserialize, DeserializeOwned};
use serde::ser::Serialize;
use std::io::{Read, Write};

use crate::traits::{SerdeBytes, SerdePretty, SerdeStream, SerdeText};

#[derive(Copy, Clone)]
pub struct Json;

function!(to_string_pretty, super::map_err, serde_json::to_string_pretty);
function!(to_string, super::map_err, serde_json::to_string);
function!(from_str, super::map_err, serde_json::from_str);
function!(to_vec_pretty, super::map_err, serde_json::to_vec_pretty);
function!(to_vec, super::map_err, serde_json::to_vec);
function!(from_slice, super::map_err, serde_json::from_slice);
function!(to_writer_pretty, super::map_err, serde_json::to_writer_pretty);
function!(to_writer, super::map_err, serde_json::to_writer);
function!(from_reader, super::map_err, serde_json::from_reader);

implement!(Json, SerdeTextPretty);
implement!(Json, SerdeBytesPretty);
implement!(Json, SerdeStreamPretty);
implement!(Json, SerdePretty);
