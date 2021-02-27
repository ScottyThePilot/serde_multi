//! CBOR serialization/deserialization, courtesy of the [`serde_cbor`] crate.
//! 
//! [`serde_cbor`]: https://crates.io/crates/serde_cbor

use serde::de::{Deserialize, DeserializeOwned};
use serde::ser::Serialize;
use std::io::{Read, Write};

use crate::traits::{Serde, SerdeBytes, SerdeStream};

#[derive(Copy, Clone)]
pub struct Cbor;

function!(to_vec, CborError, serde_cbor::to_vec);
function!(from_slice, CborError, serde_cbor::from_slice);
function!(to_writer, CborError, serde_cbor::to_writer);
function!(from_reader, CborError, serde_cbor::from_reader);

implement!(Cbor, SerdeBytes);
implement!(Cbor, SerdeStream);
implement!(Cbor, Serde, CborError);

error_struct!(serde_cbor::error::Error, CborError);

#[doc(hidden)]
impl From<serde_cbor::error::Error> for CborError {
  #[inline]
  fn from(inner: serde_cbor::error::Error) -> CborError {
    CborError { inner }
  }
}
