extern crate gcc;
#[cfg(windows)]
extern crate winapi;
#[cfg(windows)]
extern crate winreg;

use std::env;
use std::process::Command;
use std::path::Path;

#[cfg(windows)]
mod find_git {
    use std::ffi::OsStr;
    use std::io::Result;
    use std::path::PathBuf;
    use std::process::Command;
    use winapi::HKEY;
    use winreg::RegKey;
    use winreg::enums::{HKEY_CURRENT_USER, HKEY_LOCAL_MACHINE, KEY_READ};

    fn try_git_path<P: AsRef<OsStr>>(predefined_key: HKEY,
                                     subkey_path: P,
                                     value: P)
                                     -> Option<PathBuf> {
        let root = RegKey::predef(predefined_key);
        if let Ok(subkey) = root.open_subkey_with_flags(subkey_path, KEY_READ) {
            let subkey_value: Result<String> = subkey.get_value(value);
            if let Ok(install_path) = subkey_value {
                let binary_path = PathBuf::from(&install_path).join("bin").join("git.exe");
                if Command::new(&binary_path).args(&["--version"]).status().is_ok() {
                    return Some(binary_path);
                }
            }
        }
        None
    }

    pub fn git_path() -> PathBuf {
        if Command::new("git").args(&["--version"]).status().is_ok() {
            return PathBuf::from("git");
        }
        try_git_path(HKEY_LOCAL_MACHINE, "Software\\GitForWindows", "InstallPath").or_else(|| {
            try_git_path(HKEY_CURRENT_USER,
                         "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\Git_is1",
                         "InstallLocation")
        }).or_else(|| {
            try_git_path(HKEY_CURRENT_USER,
                         "SOFTWARE\\Wow6432Node\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\Git_is1",
                         "InstallLocation")
        }).or_else(|| {
            try_git_path(HKEY_LOCAL_MACHINE,
                         "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\Git_is1",
                         "InstallLocation")
        }).or_else(|| {
            try_git_path(HKEY_LOCAL_MACHINE,
                         "SOFTWARE\\Wow6432Node\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\Git_is1",
                         "InstallLocation")
        }).expect("Failed to locate Git executable.")
    }
}

#[cfg(windows)]
use find_git::git_path;

#[cfg(not(windows))]
fn git_path() -> ::std::path::PathBuf {
    ::std::path::PathBuf::from("git")
}

fn main() {
    if !Path::new("brotli/.git").exists() {
        let _ = Command::new(git_path())
                    .args(&["submodule", "update", "--init"])
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
