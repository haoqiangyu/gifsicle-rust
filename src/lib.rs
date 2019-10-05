//! The methods are mostly undocumented. You will have to see [the original C sources](https://github.com/kohler/gifsicle) for clues.
//!
//! See also [gifski](https://lib.rs/gifski) for example usage.

// 1. edit gif.h to expose Gif_Writer from gifwrite.c
// 2. bindgen vendor/src/gifsicle.h -o src/ffi.rs --no-layout-tests --opaque-type=FILE --default-enum-style=rust --whitelist-type="[gG]if.*" --whitelist-function="[gG]if.*"  --whitelist-var="[gG]if.*" -- -I vendor/include -I vendor/src
#[allow(bad_style)]
mod ffi;
pub use ffi::*;

use std::os::raw::{c_int, c_char};

pub enum Disposal {
    None       = 0,
    Asis       = 1,
    Background = 2,
    Previous   = 3,
}

extern "C" {
    pub fn gifsicle_main(argc: c_int, argv: *const *const c_char) -> c_int;
}

#[test]
fn ensure_writer_fields_are_public() {
    let mut w: Gif_Writer = unsafe { std::mem::zeroed() };
    w.pos = 0;
    w.v = std::ptr::null_mut();
}
