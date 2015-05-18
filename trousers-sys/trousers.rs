extern crate libc;

use tspi::*;

#[link(name = "tspi")]
extern "C" {
    pub fn Trspi_Error_String(result: TSS_RESULT) -> *mut libc::c_char;
}
