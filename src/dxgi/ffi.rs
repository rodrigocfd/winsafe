use crate::kernel::ffi_types::{COMPTR, HRES, PCVOID};

extern_sys! { "dxgi";
	CreateDXGIFactory(PCVOID, *mut COMPTR) -> HRES
}
