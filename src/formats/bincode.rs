//! Bincode serialization/deserialization, via the [`bincode`] crate.
//!
//! [`bincode`]: https://crates.io/crates/bincode

use serde::de::{Deserialize, DeserializeOwned};
use serde::ser::Serialize;
use std::io::{Read, Write};

use crate::traits::{SerdeBytes, SerdeStream};

#[derive(Debug, Copy, Clone, Default)]
pub struct Bincode;

function!(to_vec, map_err, |value| serde_bincode::serialize(value));
function!(from_slice, map_err, |data| serde_bincode::deserialize(data));
function!(to_writer, map_err, |writer, value| serde_bincode::serialize_into(writer, value));
function!(from_reader, map_err, |reader| serde_bincode::deserialize_from(reader));

implement!(Bincode, SerdeBytes);
implement!(Bincode, SerdeStream);

#[inline(always)]
fn map_err(err: serde_bincode::Error) -> crate::Error {
  Box::new(*err)
}
