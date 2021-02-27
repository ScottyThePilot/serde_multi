//! XML serialization/deserialization, courtesy of the [`serde-xml-rs`] crate.
//! 
//! [`serde-xml-rs`]: https://crates.io/crates/serde-xml-rs

use serde::de::{Deserialize, DeserializeOwned};
use serde::ser::Serialize;
use std::io::{Read, Write};

use crate::traits::{Serde, SerdeBytes, SerdeStream, SerdeText};

#[derive(Copy, Clone)]
pub struct Xml;

function!(to_string, XmlError, serde_xml_rs::to_string);
function!(from_str, XmlError, serde_xml_rs::from_str);
function!(to_vec, XmlError, |value| serde_xml_rs::to_string(value).map(String::into_bytes));
function!(from_slice, XmlError, |data| serde_xml_rs::from_reader(data));
function!(to_writer, XmlError, serde_xml_rs::to_writer);
function!(from_reader, XmlError, serde_xml_rs::from_reader);

implement!(Xml, SerdeText);
implement!(Xml, SerdeBytes);
implement!(Xml, SerdeStream);
implement!(Xml, Serde, XmlError);

error_struct!(serde_xml_rs::Error, XmlError);

#[doc(hidden)]
impl From<serde_xml_rs::Error> for XmlError {
  #[inline]
  fn from(inner: serde_xml_rs::Error) -> XmlError {
    XmlError { inner }
  }
}
