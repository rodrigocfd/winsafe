use crate::kernel::ffi_types::{BOOL, PVOID};

extern_sys! { "comdlg32";
	ChooseColorW(PVOID) -> BOOL
	CommDlgExtendedError() -> u32
}
