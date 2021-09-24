//! //! Raw bindings to msimg32.lib functions.

use crate::ffi::{BOOL,HANDLE};

extern_sys! { "msimg32",
	TransparentBlt, HANDLE, i32, i32, i32, i32, HANDLE, i32, i32, i32, i32, u32, => BOOL
}
