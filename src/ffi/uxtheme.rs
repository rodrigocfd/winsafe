//! Raw bindings to uxtheme.lib functions.

use crate::ffi::{BOOL, HANDLE, HRESULT, PCSTR, PCVOID, PVOID};

#[link(name = "uxtheme")]
extern "system" {
	pub fn CloseThemeData(hTheme: HANDLE) -> HRESULT;
	pub fn DrawThemeBackground(hTheme: HANDLE, hdc: HANDLE, iPartId: i32, iStateId: i32, pRect: PCVOID, pClipRect: PCVOID) -> HRESULT;
	pub fn GetThemeAppProperties() -> u32;
	pub fn GetThemeBackgroundContentRect(hTheme: HANDLE, hdc: HANDLE, iPartId: i32, iStateId: i32, pBoundingRect: PCVOID, pContentRect: PVOID) -> HRESULT;
	pub fn GetThemeBackgroundExtent(hTheme: HANDLE, hdc: HANDLE, iPartId: i32, iStateId: i32, pContentRect: PCVOID, pExtentRect: PVOID) -> HRESULT;
	pub fn GetThemeBackgroundRegion(hTheme: HANDLE, hdc: HANDLE, iPartId: i32, iStateId: i32, pRect: PCVOID, pRegion: PVOID) -> HRESULT;
	pub fn IsAppThemed() -> BOOL;
	pub fn IsCompositionActive() -> BOOL;
	pub fn IsThemeActive() -> BOOL;
	pub fn IsThemeDialogTextureEnabled() -> BOOL;
	pub fn OpenThemeData(hwnd: HANDLE, pszClassList: PCSTR) -> HANDLE;
}
