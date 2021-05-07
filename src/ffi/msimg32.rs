//! //! Raw bindings to msimg32.lib functions.

use crate::ffi::{BOOL,HANDLE};

#[link(name = "msimg32")]
extern "system" {
	pub fn TransparentBlt(_: HANDLE, _: i32, _: i32, _: i32, _: i32, _: HANDLE, _: i32, _: i32, _: i32, _: i32, _: u32) -> BOOL;
}
