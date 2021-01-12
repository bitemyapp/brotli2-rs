extern crate cc;

use std::env;
use std::process::Command;
use std::path::Path;

fn main() {
    if !Path::new("brotli/.git").exists() {
        let _ = Command::new("git").args(&["submodule", "update", "--init"])
                                   .status();
    }

    let src = env::current_dir().unwrap();
    println!("cargo:include={}", src.join("brotli/c/include").display());

    cc::Build::new()
        .include("brotli/c/include")
        .warnings(false)
        .file("brotli/c/common/dictionary.c")
        .file("brotli/c/common/transform.c")
        .file("brotli/c/dec/bit_reader.c")
        .file("brotli/c/dec/decode.c")
        .file("brotli/c/dec/huffman.c")
        .file("brotli/c/dec/state.c")
        .file("brotli/c/enc/backward_references.c")
        .file("brotli/c/enc/backward_references_hq.c")
        .file("brotli/c/enc/bit_cost.c")
        .file("brotli/c/enc/block_splitter.c")
        .file("brotli/c/enc/brotli_bit_stream.c")
        .file("brotli/c/enc/cluster.c")
        .file("brotli/c/enc/compress_fragment.c")
        .file("brotli/c/enc/compress_fragment_two_pass.c")
        .file("brotli/c/enc/dictionary_hash.c")
        .file("brotli/c/enc/encoder_dict.c")
        .file("brotli/c/enc/encode.c")
        .file("brotli/c/enc/entropy_encode.c")
        .file("brotli/c/enc/histogram.c")
        .file("brotli/c/enc/literal_cost.c")
        .file("brotli/c/enc/memory.c")
        .file("brotli/c/enc/metablock.c")
        .file("brotli/c/enc/static_dict.c")
        .file("brotli/c/enc/utf8_util.c")
        .compile("libbrotli.a");
}
