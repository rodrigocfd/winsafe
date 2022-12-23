use crate::kernel::ffi_types::{BOOL, HANDLE, PSTR, PVOID};

extern_sys! { "ktmw32";
	CommitTransaction(HANDLE) -> BOOL
	CreateTransaction(PVOID, PVOID, u32, u32, u32, u32, PSTR) -> HANDLE
	GetTransactionId(HANDLE, PVOID) -> BOOL
	OpenTransaction(u32, PVOID) -> HANDLE
	RollbackTransaction(HANDLE) -> BOOL
}
