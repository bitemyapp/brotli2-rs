//! Reader-based compression/decompression streams

use std::io::prelude::*;
use std::io::{self, BufReader};

use bufread;

// /// A compression stream which wraps an uncompressed stream of data. Compressed
// /// data will be read from the stream.
// pub struct BrotliEncoder<R: Read> {
//     inner: bufread::BrotliEncoder<BufReader<R>>,
// }

/// A decompression stream which wraps a compressed stream of data. Decompressed
/// data will be read from the stream.
pub struct BrotliDecoder<R: Read> {
    inner: bufread::BrotliDecoder<BufReader<R>>,
}

// impl<R: Read> BrotliEncoder<R> {
//     /// Create a new compression stream which will compress at the given level
//     /// to read compress output to the give output stream.
//     ///
//     /// The `level` argument here is typically 0-9 with 6 being a good default.
//     pub fn new(r: R, level: u32) -> BrotliEncoder<R> {
//         BrotliEncoder {
//             inner: bufread::BrotliEncoder::new(BufReader::new(r), level),
//         }
//     }
//
//     /// Creates a new encoder with a custom `Stream`.
//     ///
//     /// The `Stream` can be pre-configured for multithreaded encoding, different
//     /// compression options/tuning, etc.
//     pub fn new_stream(r: R, stream: Stream) -> BrotliEncoder<R> {
//         BrotliEncoder {
//             inner: bufread::BrotliEncoder::new_stream(BufReader::new(r), stream),
//         }
//     }
//
//     /// Acquires a reference to the underlying stream
//     pub fn get_ref(&self) -> &R {
//         self.inner.get_ref().get_ref()
//     }
//
//     /// Acquires a mutable reference to the underlying stream
//     ///
//     /// Note that mutation of the stream may result in surprising results if
//     /// this encoder is continued to be used.
//     pub fn get_mut(&mut self) -> &mut R {
//         self.inner.get_mut().get_mut()
//     }
//
//     /// Unwrap the underlying writer, finishing the compression stream.
//     pub fn into_inner(self) -> R {
//         self.inner.into_inner().into_inner()
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
//         self.inner.total_out()
//     }
//
//     /// Returns the number of bytes consumed by the compressor
//     /// (e.g. the number of bytes read from the underlying stream)
//     pub fn total_in(&self) -> u64 {
//         self.inner.total_in()
//     }
// }
//
// impl<R: Read> Read for BrotliEncoder<R> {
//     fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
//         self.inner.read(buf)
//     }
// }

impl<R: Read> BrotliDecoder<R> {
    /// Create a new decompression stream, which will read compressed
    /// data from the given input stream and decompress it.
    pub fn new(r: R) -> BrotliDecoder<R> {
        BrotliDecoder {
            inner: bufread::BrotliDecoder::new(BufReader::new(r)),
        }
    }

    /// Acquires a reference to the underlying stream
    pub fn get_ref(&self) -> &R {
        self.inner.get_ref().get_ref()
    }

    /// Acquires a mutable reference to the underlying stream
    ///
    /// Note that mutation of the stream may result in surprising results if
    /// this encoder is continued to be used.
    pub fn get_mut(&mut self) -> &mut R {
        self.inner.get_mut().get_mut()
    }

    /// Unwrap the underlying writer, finishing the compression stream.
    pub fn into_inner(self) -> R {
        self.inner.into_inner().into_inner()
    }
}

impl<R: Read> Read for BrotliDecoder<R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.inner.read(buf)
    }
}

// #[cfg(test)]
// mod tests {
//     use std::io::prelude::*;
//     use read::{BrotliEncoder, BrotliDecoder};
//     use rand::{thread_rng, Rng};
//
//     #[test]
//     fn smoke() {
//         let m: &[u8] = &[1, 2, 3, 4, 5, 6, 7, 8];
//         let mut c = BrotliEncoder::new(m, 6);
//         let mut data = vec![];
//         c.read_to_end(&mut data).unwrap();
//         let mut d = BrotliDecoder::new(&data[..]);
//         let mut data2 = Vec::new();
//         d.read_to_end(&mut data2).unwrap();
//         assert_eq!(data2, m);
//     }
//
//     #[test]
//     fn smoke2() {
//         let m: &[u8] = &[1, 2, 3, 4, 5, 6, 7, 8];
//         let c = BrotliEncoder::new(m, 6);
//         let mut d = BrotliDecoder::new(c);
//         let mut data = vec![];
//         d.read_to_end(&mut data).unwrap();
//         assert_eq!(data, [1, 2, 3, 4, 5, 6, 7, 8]);
//     }
//
//     #[test]
//     fn smoke3() {
//         let m = vec![3u8; 128 * 1024 + 1];
//         let c = BrotliEncoder::new(&m[..], 6);
//         let mut d = BrotliDecoder::new(c);
//         let mut data = vec![];
//         d.read_to_end(&mut data).unwrap();
//         assert!(data == &m[..]);
//     }
//
//     #[test]
//     fn self_terminating() {
//         let m = vec![3u8; 128 * 1024 + 1];
//         let mut c = BrotliEncoder::new(&m[..], 6);
//
//         let mut result = Vec::new();
//         c.read_to_end(&mut result).unwrap();
//
//         let v = thread_rng().gen_iter::<u8>().take(1024).collect::<Vec<_>>();
//         for _ in 0..200 {
//             result.extend(v.iter().map(|x| *x));
//         }
//
//         let mut d = BrotliDecoder::new(&result[..]);
//         let mut data = Vec::with_capacity(m.len());
//         unsafe { data.set_len(m.len()); }
//         assert!(d.read(&mut data).unwrap() == m.len());
//         assert!(data == &m[..]);
//     }
//
//     #[test]
//     fn zero_length_read_at_eof() {
//         let m = Vec::new();
//         let mut c = BrotliEncoder::new(&m[..], 6);
//
//         let mut result = Vec::new();
//         c.read_to_end(&mut result).unwrap();
//
//         let mut d = BrotliDecoder::new(&result[..]);
//         let mut data = Vec::new();
//         assert!(d.read(&mut data).unwrap() == 0);
//     }
//
//     #[test]
//     fn zero_length_read_with_data() {
//         let m = vec![3u8; 128 * 1024 + 1];
//         let mut c = BrotliEncoder::new(&m[..], 6);
//
//         let mut result = Vec::new();
//         c.read_to_end(&mut result).unwrap();
//
//         let mut d = BrotliDecoder::new(&result[..]);
//         let mut data = Vec::new();
//         assert!(d.read(&mut data).unwrap() == 0);
//     }
// }
//
//
//
