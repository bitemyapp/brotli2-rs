extern crate gcc;

use std::env;
use std::process::Command;
use std::path::Path;

fn main() {
    if !Path::new("brotli/.git").exists() {
        let _ = Command::new("git").args(&["submodule", "update", "--init"])
                                   .status();
    }

    let src = env::current_dir().unwrap();
    println!("cargo:include={}", src.join("brotli/dec").display());
    println!("cargo:myinclude={}", src.join("src").display());

    gcc::Config::new()
        .include("brotli/dec")
        .file("brotli/dec/decode.c")
        .file("brotli/dec/bit_reader.c")
        .file("brotli/dec/decode.c")
        .file("brotli/dec/dictionary.c")
        .file("brotli/dec/huffman.c")
        .file("brotli/dec/state.c")
        .compile("libbrotli-dec.a");
    gcc::Config::new()
        .cpp(true)
        .include("brotli/enc")
        .include("src")
        .file("src/brotli_capi.cc")
        .file("brotli/enc/backward_references.cc")
        .file("brotli/enc/block_splitter.cc")
        .file("brotli/enc/brotli_bit_stream.cc")
        .file("brotli/enc/compress_fragment.cc")
        .file("brotli/enc/compress_fragment_two_pass.cc")
        .file("brotli/enc/dictionary.cc")
        .file("brotli/enc/encode.cc")
        .file("brotli/enc/encode_parallel.cc")
        .file("brotli/enc/entropy_encode.cc")
        .file("brotli/enc/histogram.cc")
        .file("brotli/enc/literal_cost.cc")
        .file("brotli/enc/metablock.cc")
        .file("brotli/enc/static_dict.cc")
        .file("brotli/enc/streams.cc")
        .file("brotli/enc/utf8_util.cc")
        .compile("libbrotli-enc.a");
}
