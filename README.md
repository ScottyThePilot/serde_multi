# Serde Multi
[![Crate](https://img.shields.io/crates/v/serde_multi.svg)](https://crates.io/crates/serde_multi)
[![API](https://docs.rs/serde_multi/badge.svg)](https://docs.rs/serde_multi)

This library exposes a standardized API across a number of file formats, as well as providing traits to make it
possible to dynamically choose file serialization/deserialization format at runtime or based on generics.

Each file format is toggled via feature, all of which are disabled by default.

Currently the only supported file formats are:
- Bincode (via [`bincode`](https://crates.io/crates/bincode))
- CBOR (via [`serde_cbor`](https://crates.io/crates/serde_cbor))
- JSON (via [`serde_json`](https://crates.io/crates/serde_json))
- MessagePack (via [`rmp`](https://crates.io/crates/rmp) and [`rmp-serde`](https://crates.io/crates/rmp-serde))
- RON (via [`ron`](https://crates.io/crates/ron))
- TOML (via [`toml`](https://crates.io/crates/toml))
- XML (via [`serde-xml-rs`](https://crates.io/crates/serde-xml-rs))

If you would like to add more file formats, feel free to make a pull request.

## Example Usage
```rust
use serde_multi::{SerdeText, Error};
use serde_multi::formats::json::Json;
use std::collections::HashMap;

let mut value: HashMap<String, i32> = HashMap::new();
value.insert("foo".to_owned(), -193);
value.insert("bar".to_owned(), 3058);

// `Json` can be swapped out for any value that implements `SerdeText` here.
let s = Json.to_string(&value).expect("failed to serialize");
println!("serialized: {}", s);
```
