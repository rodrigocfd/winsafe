use crate::ffi_types::{BOOL, HANDLE, HRES, PCSTR, PCVOID, PVOID};

extern_sys! { "uxtheme";
	CloseThemeData(HANDLE) -> HRES
	DrawThemeBackground(HANDLE, HANDLE, i32, i32, PCVOID, PCVOID) -> HRES
	GetThemeAppProperties() -> u32
	GetThemeBackgroundContentRect(HANDLE, HANDLE, i32, i32, PCVOID, PVOID) -> HRES
	GetThemeBackgroundExtent(HANDLE, HANDLE, i32, i32, PCVOID, PVOID) -> HRES
	GetThemeBackgroundRegion(HANDLE, HANDLE, i32, i32, PCVOID, PVOID) -> HRES
	GetThemeColor(HANDLE, i32, i32, i32, *mut u32) -> HRES
	IsAppThemed() -> BOOL
	IsCompositionActive() -> BOOL
	IsThemeActive() -> BOOL
	IsThemeBackgroundPartiallyTransparent(HANDLE, i32, i32) -> BOOL
	IsThemeDialogTextureEnabled() -> BOOL
	IsThemePartDefined(HANDLE, i32, i32) -> BOOL
	OpenThemeData(HANDLE, PCSTR) -> HANDLE
}
