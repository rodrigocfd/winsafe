//! Raw bindings to uxtheme.lib functions.

use crate::ffi::{BOOL, HANDLE, PCSTR, PCVOID, PVOID};

#[link(name = "uxtheme")]
extern "system" {
	pub fn CloseThemeData(hTheme: HANDLE) -> u32;
	pub fn DrawThemeBackground(hTheme: HANDLE, hdc: HANDLE, iPartId: i32, iStateId: i32, pRect: PCVOID, pClipRect: PCVOID) -> u32;
	pub fn GetThemeAppProperties() -> u32;
	pub fn GetThemeBackgroundContentRect(hTheme: HANDLE, hdc: HANDLE, iPartId: i32, iStateId: i32, pBoundingRect: PCVOID, pContentRect: PVOID) -> u32;
	pub fn GetThemeBackgroundExtent(hTheme: HANDLE, hdc: HANDLE, iPartId: i32, iStateId: i32, pContentRect: PCVOID, pExtentRect: PVOID) -> u32;
	pub fn GetThemeBackgroundRegion(hTheme: HANDLE, hdc: HANDLE, iPartId: i32, iStateId: i32, pRect: PCVOID, pRegion: PVOID) -> u32;
	pub fn IsAppThemed() -> BOOL;
	pub fn IsCompositionActive() -> BOOL;
	pub fn IsThemeActive() -> BOOL;
	pub fn IsThemeDialogTextureEnabled() -> BOOL;
	pub fn OpenThemeData(hwnd: HANDLE, pszClassList: PCSTR) -> HANDLE;
}
