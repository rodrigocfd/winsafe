use crate::kernel::ffi_types::{COMPTR, HRES};

extern_sys! { "mf";
	MFCreateMediaSession(COMPTR, *mut COMPTR) -> HRES
}
