//! Raw bindings to gdi32.lib functions.

use std::ffi::c_void;

use crate::ffi::HANDLE;

#[link(name = "gdi32")]
extern "system" {
	pub fn CreateFontIndirectW(lplf: *const c_void) -> HANDLE;
	pub fn DeleteObject(ho: HANDLE) -> u32;
	pub fn GetTextExtentPoint32W(hdc: HANDLE, lpString: *const u16, c: i32, psizl: *mut c_void) -> u32;
	pub fn LineTo(hdc: HANDLE, x: i32, y: i32) -> u32;
	pub fn MoveToEx(hdc: HANDLE, x: i32, y: i32, lppt: *const c_void) -> u32;
	pub fn PolyBezier(hdc: HANDLE, apt: *const c_void, cpt: u32) -> u32;
	pub fn PolyBezierTo(hdc: HANDLE, apt: *const c_void, cpt: u32) -> u32;
	pub fn Polyline(hdc: HANDLE, apt: *const c_void, cpt: u32) -> u32;
	pub fn PolylineTo(hdc: HANDLE, apt: *const c_void, cpt: u32) -> u32;
	pub fn PtInRegion(hdc: HANDLE, x: i32, y: i32) -> u32;
	pub fn PtVisible(hdc: HANDLE, x: i32, y: i32) -> i32;
	pub fn Rectangle(hdc: HANDLE, left: i32, top: i32, right: i32, bottom: i32) -> u32;
	pub fn RectInRegion(hrgn: HANDLE, lprect: *const c_void) -> u32;
	pub fn RestoreDC(hdc: HANDLE, nSavedDC: i32) -> i32;
	pub fn RoundRect(hdc: HANDLE, left: i32, top: i32, right: i32, bottom: i32, width: i32, height: i32) -> u32;
	pub fn SaveDC(hdc: HANDLE) -> i32;
	pub fn SelectObject(hdc: HANDLE, h: HANDLE) -> HANDLE;
	pub fn SetBkMode(hdc: HANDLE, mode: i32) -> i32;
}