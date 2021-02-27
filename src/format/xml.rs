use serde::de::{Deserialize, DeserializeOwned};
use serde::ser::Serialize;
use std::io::{Read, Write};

use crate::traits::{Serde, SerdeBytes, SerdeStream, SerdeText};

#[derive(Copy, Clone)]
pub struct Xml;

function!(to_string, serde_xml_rs::to_string);
function!(from_str, serde_xml_rs::from_str);
function!(to_vec, |value| serde_xml_rs::to_string(value).map(String::into_bytes));
function!(from_slice, |data| serde_xml_rs::from_reader(data));
function!(to_writer, serde_xml_rs::to_writer);
function!(from_reader, serde_xml_rs::from_reader);

implement!(Xml, SerdeText);
implement!(Xml, SerdeBytes);
implement!(Xml, SerdeStream);
implement!(Xml, Serde);

error_struct!(serde_xml_rs::Error);

#[doc(hide)]
impl From<serde_xml_rs::Error> for Error {
  #[inline]
  fn from(inner: serde_xml_rs::Error) -> Error {
    Error { inner }
  }
}
