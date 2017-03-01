//! I/O streams for wrapping `BufRead` types as encoders/decoders

use std::cmp;
use std::io::prelude::*;
use std::io::{self, Cursor};

use stream::{Decompress, Status, Compress, CompressParams};

/// A brotli encoder, or compressor.
///
/// This structure implements a `BufRead` interface and will read uncompressed
/// data from an underlying stream and emit a stream of compressed data.
pub struct BrotliEncoder<R: BufRead> {
    obj: R,
    data: Compress,
    buf: Cursor<Vec<u8>>,
    max: usize,
    cur: usize,
    done: bool,
}

/// A brotli decoder, or decompressor.
///
/// This structure implements a `BufRead` interface and takes a stream of
/// compressed data as input, providing the decompressed data when read from.
pub struct BrotliDecoder<R: BufRead> {
    obj: R,
    data: Decompress,
}

impl<R: BufRead> BrotliEncoder<R> {
    /// Creates a new encoder which will read uncompressed data from the given
    /// stream and emit the compressed stream.
    ///
    /// The `level` argument here is typically 0-11.
    pub fn new(r: R, level: u32) -> BrotliEncoder<R> {
        let mut data = Compress::new();
        data.set_params(CompressParams::new().quality(level));
        BrotliEncoder {
            buf: Cursor::new(Vec::new()),
            obj: r,
            max: data.input_block_size(),
            cur: 0,
            data: data,
            done: false,
        }
    }

    /// Creates a new encoder with a custom `CompressParams`.
    pub fn from_params(r: R, params: &CompressParams) -> BrotliEncoder<R> {
        let mut data = Compress::new();
        data.set_params(params);
        BrotliEncoder {
            buf: Cursor::new(Vec::with_capacity(params.get_lgwin())),
            obj: r,
            max: data.input_block_size(),
            cur: 0,
            data: data,
            done: false,
        }
    }

    /// Acquires a reference to the underlying stream
    pub fn get_ref(&self) -> &R {
        &self.obj
    }

    /// Acquires a mutable reference to the underlying stream
    ///
    /// Note that mutation of the stream may result in surprising results if
    /// this encoder is continued to be used.
    pub fn get_mut(&mut self) -> &mut R {
        &mut self.obj
    }

    /// Consumes this encoder, returning the underlying reader.
    pub fn into_inner(self) -> R {
        self.obj
    }
}

impl<R: BufRead> Read for BrotliEncoder<R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        if buf.len() == 0 {
            return Ok(0)
        }

        match self.buf.read(buf) {
            Ok(0) if self.done => return Ok(0),
            Ok(0) => {
                self.buf.get_mut().truncate(0);
                self.buf.set_position(0);
            }
            other => return other,
        }

        loop {
            assert!(self.cur < self.max);
            let (amt_in, mut out) = {
                let input = try!(self.obj.fill_buf());
                let amt = cmp::min(input.len(), self.max - self.cur);
                self.data.copy_input(&input[..amt]);
                self.cur += amt;
                (amt, try!(self.data.compress(amt == 0, false)))
            };
            self.cur = 0;
            self.obj.consume(amt_in);
            if amt_in == 0 {
                self.done = true;
            }

            if out.len() == 0 {
                assert!(!self.done);
                continue
            }
            let ret = try!(out.read(buf));
            if out.len() > 0 {
                self.buf.get_mut().extend_from_slice(out);
            }
            return Ok(ret)
        }
    }
}

impl<R: BufRead> BrotliDecoder<R> {
    /// Creates a new decoder which will decompress data read from the given
    /// stream.
    pub fn new(r: R) -> BrotliDecoder<R> {
        BrotliDecoder {
            data: Decompress::new(),
            obj: r,
        }
    }

    /// Acquires a reference to the underlying stream
    pub fn get_ref(&self) -> &R {
        &self.obj
    }

    /// Acquires a mutable reference to the underlying stream
    ///
    /// Note that mutation of the stream may result in surprising results if
    /// this encoder is continued to be used.
    pub fn get_mut(&mut self) -> &mut R {
        &mut self.obj
    }

    /// Consumes this decoder, returning the underlying reader.
    pub fn into_inner(self) -> R {
        self.obj
    }
}

impl<R: BufRead> Read for BrotliDecoder<R> {
    fn read(&mut self, mut buf: &mut [u8]) -> io::Result<usize> {
        if buf.len() == 0 {
            return Ok(0)
        }

        loop {
            let (status, amt_in, amt_out) = {
                let mut input = try!(self.obj.fill_buf());
                let input_len = input.len();
                let buf_len = buf.len();
                let status = try!(self.data.decompress(&mut input, &mut buf));
                (status, input_len - input.len(), buf_len - buf.len())
            };
            self.obj.consume(amt_in);

            if amt_in == 0 && status == Status::NeedInput {
                return Err(io::Error::new(io::ErrorKind::Other,
                                          "corrupted brotli stream"))
            }
            if amt_out == 0 && status != Status::Finished {
                continue
            }

            return Ok(amt_out)
        }
    }
}


