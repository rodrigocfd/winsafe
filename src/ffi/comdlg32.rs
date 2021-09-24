//! Raw bindings to comdlg32.lib functions.

use crate::ffi::{BOOL, PVOID};

extern_sys! { "comdlg32",
	ChooseColorW, PVOID, => BOOL
	CommDlgExtendedError, => u32
}
