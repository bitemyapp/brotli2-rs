extern crate ctest;

use std::env;

fn main() {
    let dec_include = env::var("DEP_BROTLI_DEC_INCLUDE").unwrap();
    let enc_include = env::var("DEP_BROTLI_ENC_INCLUDE").unwrap();
    let mut cfg = ctest::TestGenerator::new();

    if env::var("TARGET").unwrap().contains("msvc") {
        cfg.flag("/wd2220"); // allow "no object file was generated"
        cfg.flag("/wd4127"); // allow "conditional expression is constant"
        cfg.flag("/wd4464"); // allow "relative include path contains '..'"
    }
    cfg.header("command.h")
       .header("decode.h")
       .header("encode.h")
       .header("hash.h")
       .header("ringbuffer.h")
       .header("workaround.h");  // this #includes state.h, first undefining BROTLI_ALLOC and BROTLI_FREE to avoid multiple definition warnings.
    cfg.include(&dec_include).include(&enc_include).include(".");
    cfg.type_name(|s, _| {
        if s == "BrotliStateStruct" {
            format!("struct BrotliStateStruct")
        } else {
            s.to_string()
        }
    });
    cfg.skip_field(|s, field| {
        s == "RingBuffer" && match field {
            "size_" | "mask_" | "tail_size_" | "total_size_" => true,
            _ => false
        }
    });
    cfg.skip_type(|n| n == "__enum_ty" || n == "BrotliEncoderState" || n == "BrotliEncoderStreamState");
    cfg.skip_signededness(|s| s.ends_with("_func") || s == "BrotliState");
    cfg.skip_struct(|s| s == "BrotliEncoderStateStruct");
    cfg.generate("../brotli-sys/src/lib.rs", "all.rs");
}
