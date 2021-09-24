//! Raw bindings to uxtheme.lib functions.

use crate::ffi::{BOOL, HANDLE, HRESULT, PCSTR, PCVOID, PVOID};

extern_sys! { "uxtheme",
	CloseThemeData, HANDLE, => HRESULT
	DrawThemeBackground, HANDLE, HANDLE, i32, i32, PCVOID, PCVOID, => HRESULT
	GetThemeAppProperties, => u32
	GetThemeBackgroundContentRect, HANDLE, HANDLE, i32, i32, PCVOID, PVOID, => HRESULT
	GetThemeBackgroundExtent, HANDLE, HANDLE, i32, i32, PCVOID, PVOID, => HRESULT
	GetThemeBackgroundRegion, HANDLE, HANDLE, i32, i32, PCVOID, PVOID, => HRESULT
	GetThemeColor, HANDLE, i32, i32, i32, *mut u32, => HRESULT
	IsAppThemed, => BOOL
	IsCompositionActive, => BOOL
	IsThemeActive, => BOOL
	IsThemeBackgroundPartiallyTransparent, HANDLE, i32, i32, => BOOL
	IsThemeDialogTextureEnabled, => BOOL
	IsThemePartDefined, HANDLE, i32, i32, => BOOL
	OpenThemeData, HANDLE, PCSTR, => HANDLE
}
