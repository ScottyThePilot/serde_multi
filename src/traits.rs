#![allow(unused_variables)]
//! Traits for standardizing the API.

use serde::de::{Deserialize, DeserializeOwned};
use serde::ser::Serialize;
use std::io::{Read, Write};

use crate::Error;

/// A trait for serialization/deserialization to and from UTF-8 strings.
/// This trait is meant to be implemented for non-binary formats.
pub trait SerdeText: SerdeBytes {
  /// Pretty-print serialize a value to a `String`.
  /// If this format does not support pretty-printing, this will defer to `to_string` instead.
  #[inline]
  fn to_string_pretty<T>(&self, value: &T) -> Result<String, Error>
  where T: Serialize, Self: Sized {
    self.to_string(value)
  }

  /// Serialize a value to a `String`.
  fn to_string<T>(&self, value: &T) -> Result<String, Error>
  where T: Serialize, Self: Sized;

  /// Deserialize a value from a `str`.
  fn from_str<'d, T>(&self, data: &'d str) -> Result<T, Error>
  where T: Deserialize<'d>, Self: Sized;
}

/// A trait for serialization/deserialization to and from binary data.
/// This trait is meant to be implemented for every format.
pub trait SerdeBytes {
  /// Pretty-print serialize a value to a `String`.
  /// If this format does not support pretty-printing, this will defer to `to_vec` instead.
  #[inline]
  fn to_vec_pretty<T>(&self, value: &T) -> Result<Vec<u8>, Error>
  where T: Serialize, Self: Sized {
    self.to_vec(value)
  }

  /// Serialize a value to a `Vec<u8>`.
  fn to_vec<T>(&self, value: &T) -> Result<Vec<u8>, Error>
  where T: Serialize, Self: Sized;

  /// Deserialize this value from a `&[u8]`.
  fn from_slice<'d, T>(&self, data: &'d [u8]) -> Result<T, Error>
  where T: Deserialize<'d>, Self: Sized;
}

/// A trait for serialization/deserialization to and from `Read` and `Write` streams.
pub trait SerdeStream: SerdeBytes {
  /// Pretty-print serialize a value into a `Write` streams.
  /// If this format does not support pretty-printing, this will defer to `to_writer` instead.
  #[inline]
  fn to_writer_pretty<W, T>(&self, writer: W, value: &T) -> Result<(), Error>
  where W: Write, T: Serialize, Self: Sized {
    self.to_writer(writer, value)
  }

  /// Serialize a value into a `Write` stream.
  fn to_writer<W, T>(&self, writer: W, value: &T) -> Result<(), Error>
  where W: Write, T: Serialize, Self: Sized;

  /// Deserialize a value from a `Read` stream.
  fn from_reader<R, T>(&self, reader: R) -> Result<T, Error>
  where R: Read, T: DeserializeOwned, Self: Sized;
}
