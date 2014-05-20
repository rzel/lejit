#![crate_id = "jit#0.1-pre"]

#![license = "MPL v2.0"]
#![crate_type = "dylib"]
#![crate_type = "rlib"]

#![feature(globs, macro_rules)]

extern crate libc;

pub use jit::*;

pub mod region;
pub mod jit;

