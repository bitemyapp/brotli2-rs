extern crate ctest;

use std::env;

fn main() {
    let include = env::var("DEP_BROTLI_INCLUDE").unwrap();
    let mut cfg = ctest::TestGenerator::new();

    if env::var("TARGET").unwrap().contains("msvc") {
        cfg.flag("/wd2220"); // allow "no object file was generated"
        cfg.flag("/wd4127"); // allow "conditional expression is constant"
        cfg.flag("/wd4464"); // allow "relative include path contains '..'"
    } else {
        cfg.flag("-Wno-deprecated-declarations");
    }
    cfg.header("brotli/decode.h")
       .header("brotli/encode.h");
    cfg.include(&include);
    cfg.type_name(|s, _| s.to_string());
    cfg.skip_type(|n| n == "__enum_ty" || n == "__enum_ty_s");
    cfg.skip_signededness(|s| s.ends_with("_func"));
    cfg.generate("../brotli-sys/src/lib.rs", "all.rs");
}
