use crate::kernel::ffi_types::*;

extern_sys! { "dxgi";
	CreateDXGIFactory(PCVOID, *mut COMPTR) -> HRES
	CreateDXGIFactory1(PCVOID, *mut COMPTR) -> HRES
}
