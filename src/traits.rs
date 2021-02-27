use serde::de::{Deserialize, DeserializeOwned};
use serde::ser::Serialize;
use std::io::{Read, Write};

pub trait SerdeText: Serde {
  #[allow(unused_variables)]
  fn to_string_pretty<T>(&self, value: &T) -> Result<String, Self::Error>
  where T: Serialize, Self: SerdePretty {
    unimplemented!()
  }

  fn to_string<T>(&self, value: &T) -> Result<String, Self::Error>
  where T: Serialize;

  fn from_str<'d, T>(&self, data: &'d str) -> Result<T, Self::Error>
  where T: Deserialize<'d>;
}

pub trait SerdeBytes: Serde {
  #[allow(unused_variables)]
  fn to_vec_pretty<T>(&self, value: &T) -> Result<Vec<u8>, Self::Error>
  where T: Serialize, Self: SerdePretty {
    unimplemented!()
  }

  fn to_vec<T>(&self, value: &T) -> Result<Vec<u8>, Self::Error>
  where T: Serialize;

  fn from_slice<'d, T>(&self, data: &'d [u8]) -> Result<T, Self::Error>
  where T: Deserialize<'d>;
}

pub trait SerdeStream: Serde {
  #[allow(unused_variables)]
  fn to_writer_pretty<W, T>(&self, writer: W, value: &T) -> Result<(), Self::Error>
  where W: Write, T: Serialize, Self: SerdePretty {
    unimplemented!()
  }

  fn to_writer<W, T>(&self, writer: W, value: &T) -> Result<(), Self::Error>
  where W: Write, T: Serialize;

  fn from_reader<R, T>(&self, reader: R) -> Result<T, Self::Error>
  where R: Read, T: DeserializeOwned;
}

pub trait SerdePretty: Serde {}

pub trait Serde {
  type Error;
}
