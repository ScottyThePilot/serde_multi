//! RON serialization/deserialization, courtesy of the [`ron`] crate.
//! 
//! [`ron`]: https://crates.io/crates/ron

use serde::de::{Deserialize, DeserializeOwned};
use serde::ser::Serialize;
use std::io::{Read, Write};

use crate::traits::{SerdeBytes, SerdeStream, SerdeText};

#[derive(Copy, Clone)]
pub struct Ron;

function!(to_string_pretty, super::map_err, |value| serde_ron::ser::to_string_pretty(value, pretty()));
function!(to_string, super::map_err, serde_ron::ser::to_string);
function!(from_str, super::map_err, serde_ron::de::from_str);
function!(to_vec_pretty, super::map_err, |value| serde_ron::ser::to_string_pretty(value, pretty()).map(String::into_bytes));
function!(to_vec, super::map_err, |value| serde_ron::ser::to_string(value).map(String::into_bytes));
function!(from_slice, super::map_err, serde_ron::de::from_bytes);
function!(to_writer_pretty, super::map_err, |writer, value| serde_ron::ser::to_writer_pretty(writer, value, pretty()));
function!(to_writer, super::map_err, serde_ron::ser::to_writer);
function!(from_reader, super::map_err, serde_ron::de::from_reader);

implement!(Ron, SerdeTextPretty);
implement!(Ron, SerdeBytesPretty);
implement!(Ron, SerdeStreamPretty);

#[inline]
fn pretty() -> serde_ron::ser::PrettyConfig {
  serde_ron::ser::PrettyConfig::new().with_indentor("  ".to_owned())
}
