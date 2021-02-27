# serde_multi

This library exposes a standardized API across a number of file formats,
traits to make this API easily implementable, and a `Format` enum for
dynamically choosing the file format for Serialization/Deserialization.
This crate does not do any substantial extra work, it simply re-organizes
the APIs of a number of other serde file format crates.

Each file format is toggled via feature, all of which are disabled by default.

Currently the only supported file formats are:
- Bincode (via [`bincode`](https://crates.io/crates/bincode))
- CBOR (via [`serde_cbor`](https://crates.io/crates/serde_cbor))
- JSON (via [`serde_json`](https://crates.io/crates/serde_json))
- MessagePack (via [`rmp`](https://crates.io/crates/rmp) and [`rmp-serde`](https://crates.io/crates/rmp-serde))
- RON (via [`ron`](https://crates.io/crates/ron))
- TOML (via [`toml`](https://crates.io/crates/toml))
- XML (via [`serde-xml-rs`](https://crates.io/crates/serde-xml-rs))
