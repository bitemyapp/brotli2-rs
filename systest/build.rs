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
    cfg.header("decode.h")
       .header("encode.h")
       .header("state.h");
    cfg.include(&dec_include).include(&enc_include);
    cfg.type_name(|s, _| {
        if s == "BrotliStateStruct" {
            format!("struct BrotliStateStruct")
        } else {
            s.to_string()
        }
    });
    cfg.skip_type(|n| n == "__enum_ty");
    cfg.skip_signededness(|s| s.ends_with("_func") || s == "BrotliState");
    cfg.generate("../brotli-sys/src/lib.rs", "all.rs");
}
