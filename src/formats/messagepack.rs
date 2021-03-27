//! MessagePack serialization/deserialization, courtesy of the [`rmp`] and [`rmp-serde`] crate.
//! 
//! [`rmp`]: https://crates.io/crates/rmp
//! [`rmp-serde`]: https://crates.io/crates/rmp-serde

use serde::de::{Deserialize, DeserializeOwned};
use serde::ser::Serialize;
use std::io::{Read, Write};

use crate::traits::{SerdeBytes, SerdeStream};

#[derive(Copy, Clone)]
pub struct MessagePack;

function!(to_vec, super::map_err, |value| rmp_serde::to_vec(value));
function!(from_slice, super::map_err, |data| rmp_serde::from_slice(data));
function!(to_writer, super::map_err, |writer, value| {
  let mut writer = writer;
  rmp_serde::encode::write(&mut writer, value)
});
function!(from_reader, super::map_err, |reader| rmp_serde::decode::from_read(reader));

implement!(MessagePack, SerdeBytes);
implement!(MessagePack, SerdeStream);
