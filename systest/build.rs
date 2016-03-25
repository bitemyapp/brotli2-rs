extern crate ctest;

use std::env;

fn main() {
    let out = env::var("DEP_BROTLI_INCLUDE").unwrap();
    let myout = env::var("DEP_BROTLI_MYINCLUDE").unwrap();
    let mut cfg = ctest::TestGenerator::new();

    if env::var("TARGET").unwrap().contains("msvc") {
        cfg.flag("/wd2220"); // allow "no object file was generated"
        cfg.flag("/wd4127"); // allow "conditional expression is constant"
    }
    cfg.header("decode.h")
       .header("brotli_capi.h");
    cfg.include(&out).include(&myout);
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
