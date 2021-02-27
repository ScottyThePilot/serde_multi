macro_rules! implement {
  ($type:ty, Serde) => {
    impl Serde for $type {
      type Error = Error;
    }
  };
  ($type:ty, SerdePretty) => {
    impl SerdePretty for $type {}
  };
  ($type:ty, SerdeText) => {
    impl SerdeText for $type {
      #[inline]
      fn to_string<T>(&self, value: &T) -> Result<String, Self::Error>
      where T: Serialize {
        to_string(value)
      }

      #[inline]
      fn from_str<'d, T>(&self, data: &'d str) -> Result<T, Self::Error>
      where T: Deserialize<'d> {
        from_str(data)
      }
    }
  };
  ($type:ty, SerdeTextPretty) => {
    impl SerdeText for $type {
      #[inline]
      fn to_string_pretty<T>(&self, value: &T) -> Result<String, Self::Error>
      where T: Serialize, Self: SerdePretty {
        to_string_pretty(value)
      }

      #[inline]
      fn to_string<T>(&self, value: &T) -> Result<String, Self::Error>
      where T: Serialize {
        to_string(value)
      }

      #[inline]
      fn from_str<'d, T>(&self, data: &'d str) -> Result<T, Self::Error>
      where T: Deserialize<'d> {
        from_str(data)
      }
    }
  };
  ($type:ty, SerdeBytes) => {
    impl SerdeBytes for $type {
      #[inline]
      fn to_vec<T>(&self, value: &T) -> Result<Vec<u8>, Self::Error>
      where T: Serialize {
        to_vec(value)
      }

      #[inline]
      fn from_slice<'d, T>(&self, data: &'d [u8]) -> Result<T, Self::Error>
      where T: Deserialize<'d> {
        from_slice(data)
      }
    }
  };
  ($type:ty, SerdeBytesPretty) => {
    impl SerdeBytes for $type {
      #[inline]
      fn to_vec_pretty<T>(&self, value: &T) -> Result<Vec<u8>, Self::Error>
      where T: Serialize, Self: SerdePretty {
        to_vec_pretty(value)
      }

      #[inline]
      fn to_vec<T>(&self, value: &T) -> Result<Vec<u8>, Self::Error>
      where T: Serialize {
        to_vec(value)
      }

      #[inline]
      fn from_slice<'d, T>(&self, data: &'d [u8]) -> Result<T, Self::Error>
      where T: Deserialize<'d> {
        from_slice(data)
      }
    }
  };
  ($type:ty, SerdeStream) => {
    impl SerdeStream for $type {
      #[inline]
      fn to_writer<W, T>(&self, writer: W, value: &T) -> Result<(), Self::Error>
      where W: Write, T: Serialize {
        to_writer(writer, value)
      }

      #[inline]
      fn from_reader<R, T>(&self, reader: R) -> Result<T, Self::Error>
      where R: Read, T: DeserializeOwned {
        from_reader(reader)
      }
    }
  };
  ($type:ty, SerdeStreamPretty) => {
    impl SerdeStream for $type {
      #[inline]
      fn to_writer_pretty<W, T>(&self, writer: W, value: &T) -> Result<(), Self::Error>
      where W: Write, T: Serialize, Self: SerdePretty {
        to_writer_pretty(writer, value)
      }

      #[inline]
      fn to_writer<W, T>(&self, writer: W, value: &T) -> Result<(), Self::Error>
      where W: Write, T: Serialize {
        to_writer(writer, value)
      }

      #[inline]
      fn from_reader<R, T>(&self, reader: R) -> Result<T, Self::Error>
      where R: Read, T: DeserializeOwned {
        from_reader(reader)
      }
    }
  };
}

macro_rules! function {
  (to_string_pretty, $path:path) => {
    function!(to_string_pretty, |value| $path(value));
  };
  (to_string_pretty, |$value:ident| $expr:expr) => {
    #[inline]
    pub fn to_string_pretty<T>($value: &T) -> Result<String, Error>
    where T: Serialize {
      $expr.map_err(From::from)
    }
  };
  (to_string, $path:path) => {
    function!(to_string, |value| $path(value));
  };
  (to_string, |$value:ident| $expr:expr) => {
    #[inline]
    pub fn to_string<T>($value: &T) -> Result<String, Error>
    where T: Serialize {
      $expr.map_err(From::from)
    }
  };
  (from_str, $path:path) => {
    function!(from_str, |data| $path(data));
  };
  (from_str, |$data:ident| $expr:expr) => {
    #[inline]
    pub fn from_str<'d, T>($data: &'d str) -> Result<T, Error>
    where T: Deserialize<'d> {
      $expr.map_err(From::from)
    }
  };
  (to_vec_pretty, $path:path) => {
    function!(to_vec_pretty, |value| $path(value));
  };
  (to_vec_pretty, |$value:ident| $expr:expr) => {
    #[inline]
    pub fn to_vec_pretty<T>($value: &T) -> Result<Vec<u8>, Error>
    where T: Serialize {
      $expr.map_err(From::from)
    }
  };
  (to_vec, $path:path) => {
    function!(to_vec, |value| $path(value));
  };
  (to_vec, |$value:ident| $expr:expr) => {
    #[inline]
    pub fn to_vec<T>($value: &T) -> Result<Vec<u8>, Error>
    where T: Serialize {
      $expr.map_err(From::from)
    }
  };
  (from_slice, $path:path) => {
    function!(from_slice, |data| $path(data));
  };
  (from_slice, |$data:ident| $expr:expr) => {
    #[inline]
    pub fn from_slice<'d, T>($data: &'d [u8]) -> Result<T, Error>
    where T: Deserialize<'d> {
      $expr.map_err(From::from)
    }
  };
  (to_writer_pretty, $path:path) => {
    function!(to_writer_pretty, |writer, value| $path(writer, value));
  };
  (to_writer_pretty, |$writer:ident, $value:ident| $expr:expr) => {
    #[inline]
    pub fn to_writer_pretty<W, T>($writer: W, $value: &T) -> Result<(), Error>
    where W: Write, T: Serialize {
      $expr.map_err(From::from)
    }
  };
  (to_writer, $path:path) => {
    function!(to_writer, |writer, value| $path(writer, value));
  };
  (to_writer, |$writer:ident, $value:ident| $expr:expr) => {
    #[inline]
    pub fn to_writer<W, T>($writer: W, $value: &T) -> Result<(), Error>
    where W: Write, T: Serialize {
      $expr.map_err(From::from)
    }
  };
  (from_reader, $path:path) => {
    function!(from_reader, |reader| $path(reader));
  };
  (from_reader, |$reader:ident| $expr:expr) => {
    #[inline]
    pub fn from_reader<R, T>($reader: R) -> Result<T, Error>
    where R: Read, T: DeserializeOwned {
      $expr.map_err(From::from)
    }
  };
}

macro_rules! error_struct {
  ($Inner:ty) => {
    #[repr(transparent)]
    pub struct Error {
      inner: $Inner
    }

    impl std::fmt::Debug for Error {
      #[inline]
      fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.inner)
      }
    }

    impl std::fmt::Display for Error {
      #[inline]
      fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.inner)
      }
    }

    impl std::error::Error for Error {}
  };
}
