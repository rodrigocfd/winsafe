use crate::kernel::ffi_types::{HANDLE, PSTR, PVOID};

extern_sys! { "ktmw32";
	CreateTransaction(PVOID, PVOID, u32, u32, u32, u32, PSTR) -> HANDLE
}
