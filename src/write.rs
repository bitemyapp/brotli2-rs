//! Writer-based compression/decompression streams

use std::cmp;
use std::io::prelude::*;
use std::io;

use stream::{Decompress, Status, Compress, CompressParams};

/// A compression stream which will have uncompressed data written to it and
/// will write compressed data to an output stream.
pub struct BrotliEncoder<W: Write> {
    data: Compress,
    obj: Option<W>,
    max: usize,
    cur: usize,
}

/// A compression stream which will have compressed data written to it and
/// will write uncompressed data to an output stream.
pub struct BrotliDecoder<W: Write> {
    data: Decompress,
    obj: Option<W>,
    buf: Vec<u8>,
}

impl<W: Write> BrotliEncoder<W> {
    /// Create a new compression stream which will compress at the given level
    /// to write compress output to the give output stream.
    pub fn new(obj: W, level: u32) -> BrotliEncoder<W> {
        let mut data = Compress::new();
        data.set_params(CompressParams::new().quality(level));
        BrotliEncoder {
            max: data.input_block_size(),
            cur: 0,
            data: data,
            obj: Some(obj),
        }
    }

    /// Creates a new encoder with a custom `CompressParams`.
    pub fn from_params(obj: W, params: &CompressParams) -> BrotliEncoder<W> {
        let mut data = Compress::new();
        data.set_params(params);
        BrotliEncoder {
            max: data.input_block_size(),
            cur: 0,
            data: data,
            obj: Some(obj)
        }
    }


    fn do_finish(&mut self) -> io::Result<()> {
        let data = try!(self.data.compress(true, false));
        self.obj.as_mut().unwrap().write_all(data)
    }

    /// Consumes this encoder, flushing the output stream.
    ///
    /// This will flush the underlying data stream and then return the contained
    /// writer if the flush succeeded.
    pub fn finish(mut self) -> io::Result<W> {
        try!(self.do_finish());
        Ok(self.obj.take().unwrap())
    }
}

impl<W: Write> Write for BrotliEncoder<W> {
    fn write(&mut self, data: &[u8]) -> io::Result<usize> {
        if self.cur > 0 {
            let data = try!(self.data.compress(false, false));
            try!(self.obj.as_mut().unwrap().write_all(data));
            self.cur = 0;
        }
        let amt = cmp::min(data.len(), self.max);
        self.data.copy_input(&data[..amt]);
        self.cur = amt;
        Ok(amt)
    }

    fn flush(&mut self) -> io::Result<()> {
        let data = try!(self.data.compress(false, true));
        let obj = self.obj.as_mut().unwrap();
        obj.write_all(data).and_then(|_| obj.flush())
    }
}

impl<W: Write> Drop for BrotliEncoder<W> {
    fn drop(&mut self) {
        if self.obj.is_some() {
            let _ = self.do_finish();
        }
    }
}

impl<W: Write> BrotliDecoder<W> {
    /// Creates a new decoding stream which will decode all input written to it
    /// into `obj`.
    pub fn new(obj: W) -> BrotliDecoder<W> {
        BrotliDecoder {
            data: Decompress::new(),
            obj: Some(obj),
            buf: Vec::with_capacity(32 * 1024),
        }
    }

    fn dump(&mut self) -> io::Result<()> {
        if self.buf.len() > 0 {
            try!(self.obj.as_mut().unwrap().write_all(&self.buf));
            self.buf.truncate(0);
        }
        Ok(())
    }

    fn do_finish(&mut self) -> io::Result<()> {
        loop {
            try!(self.dump());
            let res = try!(self.data.decompress_vec(&mut &[][..],
                                                    &mut self.buf));
            if res == Status::Finished {
                break
            }

        }
        self.dump()
    }

    /// Unwrap the underlying writer, finishing the compression stream.
    pub fn finish(&mut self) -> io::Result<W> {
        try!(self.do_finish());
        Ok(self.obj.take().unwrap())
    }
}

impl<W: Write> Write for BrotliDecoder<W> {
    fn write(&mut self, mut data: &[u8]) -> io::Result<usize> {
        loop {
            try!(self.dump());

            let data_len = data.len();
            let res = try!(self.data.decompress_vec(&mut data, &mut self.buf));
            let written = data_len - data.len();

            if written > 0 || data.len() == 0 || res == Status::Finished {
                return Ok(written)
            }
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        try!(self.dump());
        self.obj.as_mut().unwrap().flush()
    }
}

impl<W: Write> Drop for BrotliDecoder<W> {
    fn drop(&mut self) {
        if self.obj.is_some() {
            let _ = self.do_finish();
        }
    }
}

#[cfg(test)]
mod tests {
    use std::io::prelude::*;
    use std::iter::repeat;
    use super::{BrotliEncoder, BrotliDecoder};

    #[test]
    fn smoke() {
        let d = BrotliDecoder::new(Vec::new());
        let mut c = BrotliEncoder::new(d, 6);
        c.write_all(b"12834").unwrap();
        let s = repeat("12345").take(100000).collect::<String>();
        c.write_all(s.as_bytes()).unwrap();
        let data = c.finish().unwrap().finish().unwrap();
        assert_eq!(&data[0..5], b"12834");
        assert_eq!(data.len(), 500005);
        assert!(format!("12834{}", s).as_bytes() == &*data);
    }

    #[test]
    fn write_empty() {
        let d = BrotliDecoder::new(Vec::new());
        let mut c = BrotliEncoder::new(d, 6);
        c.write(b"").unwrap();
        let data = c.finish().unwrap().finish().unwrap();
        assert_eq!(&data[..], b"");
    }

    #[test]
    fn qc() {
        ::quickcheck::quickcheck(test as fn(_) -> _);

        fn test(v: Vec<u8>) -> bool {
            let w = BrotliDecoder::new(Vec::new());
            let mut w = BrotliEncoder::new(w, 6);
            w.write_all(&v).unwrap();
            v == w.finish().unwrap().finish().unwrap()
        }
    }
}
