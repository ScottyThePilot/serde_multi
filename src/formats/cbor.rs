//! CBOR serialization/deserialization, courtesy of the [`serde_cbor`] crate.
//! 
//! [`serde_cbor`]: https://crates.io/crates/serde_cbor

use serde::de::{Deserialize, DeserializeOwned};
use serde::ser::Serialize;
use std::io::{Read, Write};

use crate::traits::{SerdeBytes, SerdeStream};

#[derive(Copy, Clone)]
pub struct Cbor;

function!(to_vec, super::map_err, serde_cbor::to_vec);
function!(from_slice, super::map_err, serde_cbor::from_slice);
function!(to_writer, super::map_err, serde_cbor::to_writer);
function!(from_reader, super::map_err, serde_cbor::from_reader);

implement!(Cbor, SerdeBytes);
implement!(Cbor, SerdeStream);
