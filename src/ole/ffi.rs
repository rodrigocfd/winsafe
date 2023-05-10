use crate::kernel::ffi_types::{BOOL, COMPTR, HANDLE, HRES, PCSTR, PCVOID, PSTR, PVOID};

extern_sys! { "ole32";
	CLSIDFromProgID(PCSTR, PVOID) -> HRES
	CLSIDFromProgIDEx(PCSTR, PVOID) -> HRES
	CLSIDFromString(PCSTR, PVOID) -> HRES
	CoCreateInstance(PCVOID, *mut COMPTR, u32, PCVOID, *mut COMPTR) -> HRES
	CoCreateInstanceEx(PCVOID, *mut COMPTR, u32, PCVOID, u32, PVOID) -> HRES
	CoInitializeEx(PVOID, u32) -> HRES
	CoLockObjectExternal(COMPTR, BOOL, BOOL) -> HRES
	CoTaskMemAlloc(usize) -> PVOID
	CoTaskMemFree(PVOID)
	CoTaskMemRealloc(PVOID, usize) -> PVOID
	CoUninitialize()
	CreateClassMoniker(PCVOID, *mut COMPTR) -> HRES
	CreateFileMoniker(PCSTR, *mut COMPTR) -> HRES
	CreateItemMoniker(PCSTR, PCSTR, *mut COMPTR) -> HRES
	CreateObjrefMoniker(COMPTR, *mut COMPTR) -> HRES
	CreatePointerMoniker(COMPTR, *mut COMPTR) -> HRES
	RegisterDragDrop(HANDLE, COMPTR) -> HRES
	RevokeDragDrop(HANDLE) -> HRES
	StringFromCLSID(PCVOID, *mut PSTR) -> HRES
}
