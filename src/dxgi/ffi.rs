use crate::kernel::ffi_types::{HRES, PCVOID, PVOID};

extern_sys! { "dxgi";
	CreateDXGIFactory(PCVOID, *mut PVOID) -> HRES
}
