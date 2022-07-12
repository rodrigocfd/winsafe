use crate::ffi_types::{BOOL, HRES, PCSTR, PCVOID, PSTR, PVOID};

extern_sys! { "oleaut32";
	OleLoadPicture(PVOID, i32, BOOL, PCVOID, PVOID) -> HRES
	OleLoadPicturePath(PCSTR, *mut PVOID, u32, u32, PCVOID, *mut PVOID) -> HRES
}

extern_sys! { "propsys";
	PSGetNameFromPropertyKey(PCVOID, *mut PSTR) -> HRES
}
