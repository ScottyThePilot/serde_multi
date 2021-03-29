//! TOML serialization/deserialization, courtesy of the [`toml`] crate.
//! 
//! Note: the [`toml`] crate does not expose any stream functions, so
//! the writer functions here may be inefficient.
//! 
//! [`toml`]: https://crates.io/crates/toml
//! [`SerdeStream`]: ../../traits/trait.SerdeStream.html

use serde::de::{Deserialize, DeserializeOwned};
use serde::ser::Serialize;
use std::io::{Read, Write};

use crate::traits::{SerdeBytes, SerdeStream, SerdeText};

#[derive(Copy, Clone, Default)]
pub struct Toml;

function!(to_string_pretty, super::map_err, serde_toml::to_string_pretty);
function!(to_string, super::map_err, serde_toml::to_string);
function!(from_str, super::map_err, |data| serde_toml::from_str::<serde_toml::Value>(data)?.try_into());
function!(to_vec_pretty, super::map_err, |value| serde_toml::to_string_pretty(value).map(String::into_bytes));
function!(to_vec, super::map_err, serde_toml::to_vec);
function!(from_slice, super::map_err, |data| serde_toml::from_slice::<serde_toml::Value>(data)?.try_into());
function!(to_writer_pretty, |t| t, |writer, value| {
  let value = serde_toml::to_string_pretty(value)?;
  {writer}.write_all(value.as_bytes())?;
  Ok(())
});
function!(to_writer, |t| t, |writer, value| {
  let value = serde_toml::to_string(value)?;
  {writer}.write_all(value.as_bytes())?;
  Ok(())
});
function!(from_reader, |t| t, |reader| {
  let mut data = Vec::new();
  {reader}.read_to_end(&mut data)?;
  Ok(serde_toml::from_slice::<serde_toml::Value>(&data)?.try_into()?)
});

implement!(Toml, SerdeTextPretty);
implement!(Toml, SerdeBytesPretty);
implement!(Toml, SerdeStreamPretty);
