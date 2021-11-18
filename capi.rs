///! Implements a C API for the Horn Form Knowledge base, with clauses specified by Unicode-encoded null terminated C strings.
mod hfkb;

use hfkb::HornFormKb;
use std::os::raw::c_char;
use std::ffi::CStr;

#[no_mangle]
pub extern "C" fn HornFormKb_new() -> *mut HornFormKb {
	Box::into_raw(Box::new(HornFormKb::new()))
}

#[no_mangle]
pub unsafe extern "C" fn HornFormKb_drop(hfkb: *mut HornFormKb) {
	Box::from_raw(hfkb);
}

#[no_mangle]
pub unsafe extern "C" fn HornFormKb_tell(hfkb: *mut HornFormKb, expr: *const c_char, symbol: *const c_char) {
	(*hfkb).tell(
		CStr::from_ptr(expr).to_str().unwrap(),
		CStr::from_ptr(symbol).to_str().unwrap()
	);
}

#[no_mangle]
pub unsafe extern "C" fn HornFormKb_ask(hfkb: *mut HornFormKb, question: *const c_char) -> u8 {
	if (*hfkb).ask(
		CStr::from_ptr(question).to_str().unwrap()
	) { 1 } else { 0 }
}
