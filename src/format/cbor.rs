use serde::de::{Deserialize, DeserializeOwned};
use serde::ser::Serialize;
use std::io::{Read, Write};

use crate::traits::{Serde, SerdeBytes, SerdeStream};

#[derive(Copy, Clone)]
pub struct Cbor;

function!(to_vec, serde_cbor::to_vec);
function!(from_slice, serde_cbor::from_slice);
function!(to_writer, serde_cbor::to_writer);
function!(from_reader, serde_cbor::from_reader);

implement!(Cbor, SerdeBytes);
implement!(Cbor, SerdeStream);
implement!(Cbor, Serde);

error_struct!(serde_cbor::error::Error);

#[doc(hide)]
impl From<serde_cbor::error::Error> for Error {
  #[inline]
  fn from(inner: serde_cbor::error::Error) -> Error {
    Error { inner }
  }
}
