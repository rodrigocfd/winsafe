//! Raw bindings to uxtheme.lib functions.

use crate::ffi::{BOOL, HANDLE, HRESULT, PCSTR, PCVOID, PVOID};

#[link(name = "uxtheme")]
extern "system" {
	pub fn CloseThemeData(_: HANDLE) -> HRESULT;
	pub fn DrawThemeBackground(_: HANDLE, _: HANDLE, _: i32, _: i32, _: PCVOID, _: PCVOID) -> HRESULT;
	pub fn GetThemeAppProperties() -> u32;
	pub fn GetThemeBackgroundContentRect(_: HANDLE, _: HANDLE, _: i32, _: i32, _: PCVOID, _: PVOID) -> HRESULT;
	pub fn GetThemeBackgroundExtent(_: HANDLE, _: HANDLE, _: i32, _: i32, _: PCVOID, _: PVOID) -> HRESULT;
	pub fn GetThemeBackgroundRegion(_: HANDLE, _: HANDLE, _: i32, _: i32, _: PCVOID, _: PVOID) -> HRESULT;
	pub fn GetThemeColor(_: HANDLE, _: i32, _: i32, _: i32, _: *mut u32) -> HRESULT;
	pub fn IsAppThemed() -> BOOL;
	pub fn IsCompositionActive() -> BOOL;
	pub fn IsThemeActive() -> BOOL;
	pub fn IsThemeBackgroundPartiallyTransparent(_: HANDLE, _: i32, _: i32) -> BOOL;
	pub fn IsThemeDialogTextureEnabled() -> BOOL;
	pub fn IsThemePartDefined(_: HANDLE, _: i32, _: i32) -> BOOL;
	pub fn OpenThemeData(_: HANDLE, _: PCSTR) -> HANDLE;
}
