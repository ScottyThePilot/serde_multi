//! XML serialization/deserialization, courtesy of the [`serde-xml-rs`] crate.
//! 
//! [`serde-xml-rs`]: https://crates.io/crates/serde-xml-rs

use serde::de::{Deserialize, DeserializeOwned};
use serde::ser::Serialize;
use std::io::{Read, Write};

use crate::traits::{SerdeBytes, SerdeStream, SerdeText};

#[derive(Copy, Clone, Default)]
pub struct Xml;

function!(to_string, super::map_err, serde_xml_rs::to_string);
function!(from_str, super::map_err, serde_xml_rs::from_str);
function!(to_vec, super::map_err, |value| serde_xml_rs::to_string(value).map(String::into_bytes));
function!(from_slice, super::map_err, |data| serde_xml_rs::from_reader(data));
function!(to_writer, super::map_err, serde_xml_rs::to_writer);
function!(from_reader, super::map_err, serde_xml_rs::from_reader);

implement!(Xml, SerdeText);
implement!(Xml, SerdeBytes);
implement!(Xml, SerdeStream);
