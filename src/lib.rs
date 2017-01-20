//! Brotli Compression/Decompression for Rust
//!
//! This crate is a binding to the [official brotli implementation][brotli] and
//! provides in-memory and I/O streams for Rust wrappers.
//!
//! [brotli]: https://github.com/google/brotli
//!
//! # Examples
//!
//! ```
//! use std::io::prelude::*;
//! use brotli2::read::{BrotliEncoder, BrotliDecoder};
//!
//! // Round trip some bytes from a byte source, into a compressor, into a
//! // decompressor, and finally into a vector.
//! let data = "Hello, World!".as_bytes();
//! let compressor = BrotliEncoder::new(data, 9);
//! let mut decompressor = BrotliDecoder::new(compressor);
//!
//! let mut contents = String::new();
//! decompressor.read_to_string(&mut contents).unwrap();
//! assert_eq!(contents, "Hello, World!");
//! ```

#![deny(missing_docs)]
#![doc(html_root_url = "https://docs.rs/brotli2/0.2")]

extern crate brotli_sys;
extern crate libc;

#[cfg(test)]
extern crate rand;
#[cfg(test)]
extern crate quickcheck;

pub mod stream;
pub mod bufread;
pub mod read;
pub mod write;
