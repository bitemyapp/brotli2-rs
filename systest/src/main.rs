#![allow(bad_style)]

extern crate brotli_sys;
extern crate libc;

use libc::*;
use brotli_sys::*;

include!(concat!(env!("OUT_DIR"), "/all.rs"));
