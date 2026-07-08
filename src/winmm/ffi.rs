use crate::kernel::ffi_types::*;
use crate::macros::*;

extern_sys! { "winmm";
	PlaySoundW(PCSTR, HANDLE, u32) -> BOOL
}
