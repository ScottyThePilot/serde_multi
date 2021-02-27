use serde::de::{Deserialize, DeserializeOwned};
use serde::ser::Serialize;
use std::io::{Read, Write};

use crate::traits::{Serde, SerdeBytes, SerdeStream};

#[derive(Copy, Clone)]
pub struct Bincode;

function!(to_vec, |value| serde_bincode::serialize(value));
function!(from_slice, |data| serde_bincode::deserialize(data));
function!(to_writer, |writer, value| serde_bincode::serialize_into(writer, value));
function!(from_reader, |reader| serde_bincode::deserialize_from(reader));

implement!(Bincode, SerdeBytes);
implement!(Bincode, SerdeStream);
implement!(Bincode, Serde);

error_struct!(serde_bincode::ErrorKind);

#[doc(hide)]
impl From<serde_bincode::Error> for Error {
  #[inline]
  fn from(inner: serde_bincode::Error) -> Error {
    Error { inner: *inner }
  }
}
