//! //! Raw bindings to msimg32.lib functions.

use crate::ffi::{BOOL,HANDLE};

#[link(name = "msimg32")]
extern "system" {
	pub fn TransparentBlt(hdcDest: HANDLE, xoriginDest: i32, yoriginDest: i32, wDest: i32, hDest: i32, hdcSrc: HANDLE, xoriginSrc: i32, yoriginSrc: i32, wSrc: i32, hSrc: i32, crTransparent: u32) -> BOOL;
}
