//! Brotli Compression/Decompression for Rust
//!
//! This crate is a binding to the [official brotli implementation][brotli] and
//! provides in-memory and I/O streams for Rust wrappers.
//!
//! [brotli]: https://github.com/google/brotli

#![deny(missing_docs)]

extern crate brotli_sys;
extern crate libc;

#[cfg(test)]
extern crate rand;

pub mod stream;
pub mod bufread;
pub mod read;
pub mod write;
