//! Traits for standardizing the API.

use serde::de::{Deserialize, DeserializeOwned};
use serde::ser::Serialize;
use std::io::{Read, Write};

/// A trait for serialization/deserialization to and from UTF-8 strings.
/// This trait is meant to be implemented for non-binary formats.
pub trait SerdeText: Serde {
  /// Pretty-print serialize a value to a `String`.
  /// 
  /// This function should ONLY be implemented for types that also implement
  /// [`SerdePretty`], which make this function usable in the first place.
  /// 
  /// [`SerdePretty`]: ./trait.SerdePretty.html
  fn to_string_pretty<T>(&self, value: &T) -> Result<String, Self::Error>
  where T: Serialize, Self: SerdePretty {
    unimplemented!()
  }

  /// Serialize a value to a `String`.
  fn to_string<T>(&self, value: &T) -> Result<String, Self::Error>
  where T: Serialize;

  /// Deserialize a value from a `str`.
  fn from_str<'d, T>(&self, data: &'d str) -> Result<T, Self::Error>
  where T: Deserialize<'d>;
}

/// A trait for serialization/deserialization to and from binary data.
/// This trait is meant to be implemented for every format.
pub trait SerdeBytes: Serde {
  /// Pretty-print serialize a value to a `String`.
  /// 
  /// This function should ONLY be implemented for types that also implement
  /// [`SerdePretty`], which make this function usable in the first place.
  /// 
  /// [`SerdePretty`]: ./trait.SerdePretty.html
  fn to_vec_pretty<T>(&self, value: &T) -> Result<Vec<u8>, Self::Error>
  where T: Serialize, Self: SerdePretty {
    unimplemented!()
  }

  /// Serialize a value to a `Vec<u8>`.
  fn to_vec<T>(&self, value: &T) -> Result<Vec<u8>, Self::Error>
  where T: Serialize;

  /// Deserialize this value from a `&[u8]`.
  fn from_slice<'d, T>(&self, data: &'d [u8]) -> Result<T, Self::Error>
  where T: Deserialize<'d>;
}

/// A trait for serialization/deserialization to and from `Read` and `Write` streams.
pub trait SerdeStream: Serde {
  /// Pretty-print serialize a value into a `Write` streams.
  /// 
  /// This function should ONLY be implemented for types that also implement
  /// [`SerdePretty`], which make this function usable in the first place.
  /// 
  /// [`SerdePretty`]: ./trait.SerdePretty.html
  fn to_writer_pretty<W, T>(&self, writer: W, value: &T) -> Result<(), Self::Error>
  where W: Write, T: Serialize, Self: SerdePretty {
    unimplemented!()
  }

  /// Serialize a value into a `Write` stream.
  fn to_writer<W, T>(&self, writer: W, value: &T) -> Result<(), Self::Error>
  where W: Write, T: Serialize;

  /// Deserialize a value from a `Read` stream.
  fn from_reader<R, T>(&self, reader: R) -> Result<T, Self::Error>
  where R: Read, T: DeserializeOwned;
}

/// A trait for pretty-print serialization.
/// 
/// The other trait's `to_x_pretty()` functions should ONLY be implemented
/// when this trait is also implemented, otherwise Rust's type system will not
/// let you call that function.
pub trait SerdePretty: Serde {}

/// The core serde trait.
/// This defines the error type that all of the sub-traits may return.
pub trait Serde {
  type Error;
}
