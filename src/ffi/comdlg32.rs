//! Raw bindings to comdlg32.lib functions.

use crate::ffi::{BOOL, PVOID};

#[link(name = "comdlg32")]
extern "system" {
	pub fn ChooseColorW(_: PVOID) -> BOOL;
	pub fn CommDlgExtendedError() -> u32;
}
