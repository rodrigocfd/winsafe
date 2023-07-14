use crate::kernel::ffi_types::{COMPTR, HRES};

extern_sys! { "mf";
	MFCreateMediaSession(COMPTR, *mut COMPTR) -> HRES
	MFCreateTopology(*mut COMPTR) -> HRES
	MFCreateTopologyNode(u32, *mut COMPTR) -> HRES
}

extern_sys! { "mfplat";
	MFStartup(u32, u32) -> HRES
}
