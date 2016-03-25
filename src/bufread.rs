//! I/O streams for wrapping `BufRead` types as encoders/decoders

use std::io::prelude::*;
use std::io;

use stream::{Decompress, Status};

// /// An xz encoder, or compressor.
// ///
// /// This structure implements a `BufRead` interface and will read uncompressed
// /// data from an underlying stream and emit a stream of compressed data.
// pub struct BrotliEncoder<R: BufRead> {
//     obj: R,
//     data: Stream,
// }

/// A brotli decoder, or decompressor.
///
/// This structure implements a `BufRead` interface and takes a stream of
/// compressed data as input, providing the decompressed data when read from.
pub struct BrotliDecoder<R: BufRead> {
    obj: R,
    data: Decompress,
}

// impl<R: BufRead> BrotliEncoder<R> {
//     /// Creates a new encoder which will read uncompressed data from the given
//     /// stream and emit the compressed stream.
//     ///
//     /// The `level` argument here is typically 0-9 with 6 being a good default.
//     pub fn new(r: R, level: u32) -> BrotliEncoder<R> {
//         let stream = Stream::new_easy_encoder(level, Check::Crc64).unwrap();
//         BrotliEncoder::new_stream(r, stream)
//     }
//
//     /// Creates a new encoder with a custom `Stream`.
//     ///
//     /// The `Stream` can be pre-configured for multithreaded encoding, different
//     /// compression options/tuning, etc.
//     pub fn new_stream(r: R, stream: Stream) -> BrotliEncoder<R> {
//         BrotliEncoder {
//             obj: r,
//             data: stream,
//         }
//     }
//
//     /// Acquires a reference to the underlying stream
//     pub fn get_ref(&self) -> &R {
//         &self.obj
//     }
//
//     /// Acquires a mutable reference to the underlying stream
//     ///
//     /// Note that mutation of the stream may result in surprising results if
//     /// this encoder is continued to be used.
//     pub fn get_mut(&mut self) -> &mut R {
//         &mut self.obj
//     }
//
//     /// Consumes this encoder, returning the underlying reader.
//     pub fn into_inner(self) -> R {
//         self.obj
//     }
//
//     /// Returns the number of bytes produced by the compressor
//     /// (e.g. the number of bytes read from this stream)
//     ///
//     /// Note that, due to buffering, this only bears any relation to
//     /// total_in() when the compressor chooses to flush its data
//     /// (unfortunately, this won't happen this won't happen in general
//     /// at the end of the stream, because the compressor doesn't know
//     /// if there's more data to come).  At that point,
//     /// `total_out() / total_in()` would be the compression ratio.
//     pub fn total_out(&self) -> u64 {
//         self.data.total_out()
//     }
//
//     /// Returns the number of bytes consumed by the compressor
//     /// (e.g. the number of bytes read from the underlying stream)
//     pub fn total_in(&self) -> u64 {
//         self.data.total_in()
//     }
// }
// impl<R: BufRead> Read for BrotliEncoder<R> {
//     fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
//         loop {
//             let (read, consumed, eof, ret);
//             {
//                 let input = try!(self.obj.fill_buf());
//                 eof = input.is_empty();
//                 let before_out = self.data.total_out();
//                 let before_in = self.data.total_in();
//                 let action = if eof {Action::Finish} else {Action::Run};
//                 ret = self.data.process(input, buf, action);
//                 read = (self.data.total_out() - before_out) as usize;
//                 consumed = (self.data.total_in() - before_in) as usize;
//             }
//             self.obj.consume(consumed);
//
//             ret.unwrap();
//
//             // If we haven't ready any data and we haven't hit EOF yet, then we
//             // need to keep asking for more data because if we return that 0
//             // bytes of data have been read then it will be interpreted as EOF.
//             if read == 0 && !eof && buf.len() > 0 {
//                 continue
//             }
//             return Ok(read)
//         }
//     }
// }

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
        loop {
            let (status, amt_in, amt_out) = {
                let mut input = try!(self.obj.fill_buf());
                let input_len = input.len();
                let buf_len = buf.len();
                let status = try!(self.data.decompress(&mut input, &mut buf));
                (status, input_len - input.len(), buf_len - buf.len())
            };
            self.obj.consume(amt_in);

            if amt_out == 0 && status != Status::Finished {
                continue
            }

            return Ok(amt_out)
        }
    }
}


