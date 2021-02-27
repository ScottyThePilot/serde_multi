//! Bincode serialization/deserialization, courtesy of the [`bincode`] crate.
//! 
//! [`bincode`]: https://crates.io/crates/bincode

use serde::de::{Deserialize, DeserializeOwned};
use serde::ser::Serialize;
use std::io::{Read, Write};

use crate::traits::{Serde, SerdeBytes, SerdeStream};

#[derive(Copy, Clone)]
pub struct Bincode;

function!(to_vec, BincodeError, |value| serde_bincode::serialize(value));
function!(from_slice, BincodeError, |data| serde_bincode::deserialize(data));
function!(to_writer, BincodeError, |writer, value| serde_bincode::serialize_into(writer, value));
function!(from_reader, BincodeError, |reader| serde_bincode::deserialize_from(reader));

implement!(Bincode, SerdeBytes);
implement!(Bincode, SerdeStream);
implement!(Bincode, Serde, BincodeError);

error_struct!(serde_bincode::ErrorKind, BincodeError);

#[doc(hidden)]
impl From<serde_bincode::Error> for BincodeError {
  #[inline]
  fn from(inner: serde_bincode::Error) -> BincodeError {
    BincodeError { inner: *inner }
  }
}
