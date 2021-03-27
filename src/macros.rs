macro_rules! implement {
  ($type:ty, SerdePretty) => {
    impl SerdePretty for $type {}
  };
  ($type:ty, SerdeText) => {
    impl SerdeText for $type {
      #[inline]
      fn to_string<T>(&self, value: &T) -> Result<String, crate::Error>
      where T: Serialize {
        to_string(value)
      }

      #[inline]
      fn from_str<'d, T>(&self, data: &'d str) -> Result<T, crate::Error>
      where T: Deserialize<'d> {
        from_str(data)
      }
    }
  };
  ($type:ty, SerdeTextPretty) => {
    impl SerdeText for $type {
      #[inline]
      fn to_string_pretty<T>(&self, value: &T) -> Result<String, crate::Error>
      where T: Serialize, Self: SerdePretty {
        to_string_pretty(value)
      }

      #[inline]
      fn to_string<T>(&self, value: &T) -> Result<String, crate::Error>
      where T: Serialize {
        to_string(value)
      }

      #[inline]
      fn from_str<'d, T>(&self, data: &'d str) -> Result<T, crate::Error>
      where T: Deserialize<'d> {
        from_str(data)
      }
    }
  };
  ($type:ty, SerdeBytes) => {
    impl SerdeBytes for $type {
      #[inline]
      fn to_vec<T>(&self, value: &T) -> Result<Vec<u8>, crate::Error>
      where T: Serialize {
        to_vec(value)
      }

      #[inline]
      fn from_slice<'d, T>(&self, data: &'d [u8]) -> Result<T, crate::Error>
      where T: Deserialize<'d> {
        from_slice(data)
      }
    }
  };
  ($type:ty, SerdeBytesPretty) => {
    impl SerdeBytes for $type {
      #[inline]
      fn to_vec_pretty<T>(&self, value: &T) -> Result<Vec<u8>, crate::Error>
      where T: Serialize, Self: SerdePretty {
        to_vec_pretty(value)
      }

      #[inline]
      fn to_vec<T>(&self, value: &T) -> Result<Vec<u8>, crate::Error>
      where T: Serialize {
        to_vec(value)
      }

      #[inline]
      fn from_slice<'d, T>(&self, data: &'d [u8]) -> Result<T, crate::Error>
      where T: Deserialize<'d> {
        from_slice(data)
      }
    }
  };
  ($type:ty, SerdeStream) => {
    impl SerdeStream for $type {
      #[inline]
      fn to_writer<W, T>(&self, writer: W, value: &T) -> Result<(), crate::Error>
      where W: Write, T: Serialize {
        to_writer(writer, value)
      }

      #[inline]
      fn from_reader<R, T>(&self, reader: R) -> Result<T, crate::Error>
      where R: Read, T: DeserializeOwned {
        from_reader(reader)
      }
    }
  };
  ($type:ty, SerdeStreamPretty) => {
    impl SerdeStream for $type {
      #[inline]
      fn to_writer_pretty<W, T>(&self, writer: W, value: &T) -> Result<(), crate::Error>
      where W: Write, T: Serialize, Self: SerdePretty {
        to_writer_pretty(writer, value)
      }

      #[inline]
      fn to_writer<W, T>(&self, writer: W, value: &T) -> Result<(), crate::Error>
      where W: Write, T: Serialize {
        to_writer(writer, value)
      }

      #[inline]
      fn from_reader<R, T>(&self, reader: R) -> Result<T, crate::Error>
      where R: Read, T: DeserializeOwned {
        from_reader(reader)
      }
    }
  };
}

macro_rules! function {
  (to_string_pretty, $map_error:expr, $path:path) => {
    function!(to_string_pretty, $map_error, |value| $path(value));
  };
  (to_string_pretty, $map_error:expr, |$value:ident| $expr:expr) => {
    #[inline]
    pub fn to_string_pretty<T>($value: &T) -> Result<String, $crate::Error>
    where T: Serialize {
      $expr.map_err($map_error)
    }
  };
  (to_string, $map_error:expr, $path:path) => {
    function!(to_string, $map_error, |value| $path(value));
  };
  (to_string, $map_error:expr, |$value:ident| $expr:expr) => {
    #[inline]
    pub fn to_string<T>($value: &T) -> Result<String, $crate::Error>
    where T: Serialize {
      $expr.map_err($map_error)
    }
  };
  (from_str, $map_error:expr, $path:path) => {
    function!(from_str, $map_error, |data| $path(data));
  };
  (from_str, $map_error:expr, |$data:ident| $expr:expr) => {
    #[inline]
    pub fn from_str<'d, T>($data: &'d str) -> Result<T, $crate::Error>
    where T: Deserialize<'d> {
      $expr.map_err($map_error)
    }
  };
  (to_vec_pretty, $map_error:expr, $path:path) => {
    function!(to_vec_pretty, $map_error, |value| $path(value));
  };
  (to_vec_pretty, $map_error:expr, |$value:ident| $expr:expr) => {
    #[inline]
    pub fn to_vec_pretty<T>($value: &T) -> Result<Vec<u8>, $crate::Error>
    where T: Serialize {
      $expr.map_err($map_error)
    }
  };
  (to_vec, $map_error:expr, $path:path) => {
    function!(to_vec, $map_error, |value| $path(value));
  };
  (to_vec, $map_error:expr, |$value:ident| $expr:expr) => {
    #[inline]
    pub fn to_vec<T>($value: &T) -> Result<Vec<u8>, $crate::Error>
    where T: Serialize {
      $expr.map_err($map_error)
    }
  };
  (from_slice, $map_error:expr, $path:path) => {
    function!(from_slice, $map_error, |data| $path(data));
  };
  (from_slice, $map_error:expr, |$data:ident| $expr:expr) => {
    #[inline]
    pub fn from_slice<'d, T>($data: &'d [u8]) -> Result<T, $crate::Error>
    where T: Deserialize<'d> {
      $expr.map_err($map_error)
    }
  };
  (to_writer_pretty, $map_error:expr, $path:path) => {
    function!(to_writer_pretty, $map_error, |writer, value| $path(writer, value));
  };
  (to_writer_pretty, $map_error:expr, |$writer:ident, $value:ident| $expr:expr) => {
    #[inline]
    pub fn to_writer_pretty<W, T>($writer: W, $value: &T) -> Result<(), $crate::Error>
    where W: Write, T: Serialize {
      $expr.map_err($map_error)
    }
  };
  (to_writer, $map_error:expr, $path:path) => {
    function!(to_writer, $map_error, |writer, value| $path(writer, value));
  };
  (to_writer, $map_error:expr, |$writer:ident, $value:ident| $expr:expr) => {
    #[inline]
    pub fn to_writer<W, T>($writer: W, $value: &T) -> Result<(), $crate::Error>
    where W: Write, T: Serialize {
      $expr.map_err($map_error)
    }
  };
  (from_reader, $map_error:expr, $path:path) => {
    function!(from_reader, $map_error, |reader| $path(reader));
  };
  (from_reader, $map_error:expr, |$reader:ident| $expr:expr) => {
    #[inline]
    pub fn from_reader<R, T>($reader: R) -> Result<T, $crate::Error>
    where R: Read, T: DeserializeOwned {
      $expr.map_err($map_error)
    }
  };
}
