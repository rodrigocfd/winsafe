use crate::kernel::ffi_types::*;

extern_sys! { "comdlg32";
	ChooseColorW(PVOID) -> BOOL
	CommDlgExtendedError() -> u32
}
