//! Raw bindings to gdi32.lib functions.

use crate::ffi::{BOOL, HANDLE, PCSTR, PCVOID, PVOID};

#[link(name = "gdi32")]
extern "system" {
	pub fn CreateFontW(cHeight: i32, cWidth: i32, cEscapement: i32, cOrientation: i32, cWeight: i32, bItalic: u32, bUnderline: u32, bStrikeOut: u32, iCharSet: u32, iOutPrecision: u32, iClipPrecision: u32, iQuality: u32, iPitchAndFamily: u32, pszFaceName: PCSTR) -> HANDLE;
	pub fn CreateFontIndirectW(lplf: PCVOID) -> HANDLE;
	pub fn DeleteObject(ho: HANDLE) -> BOOL;
	pub fn GetDeviceCaps(hdc: HANDLE, index: i32) -> i32;
	pub fn GetTextExtentPoint32W(hdc: HANDLE, lpString: PCSTR, c: i32, psizl: PVOID) -> BOOL;
	pub fn LineTo(hdc: HANDLE, x: i32, y: i32) -> BOOL;
	pub fn MoveToEx(hdc: HANDLE, x: i32, y: i32, lppt: PVOID) -> BOOL;
	pub fn PolyBezier(hdc: HANDLE, apt: PCVOID, cpt: u32) -> BOOL;
	pub fn PolyBezierTo(hdc: HANDLE, apt: PCVOID, cpt: u32) -> BOOL;
	pub fn Polyline(hdc: HANDLE, apt: PCVOID, cpt: u32) -> BOOL;
	pub fn PolylineTo(hdc: HANDLE, apt: PCVOID, cpt: u32) -> BOOL;
	pub fn PtInRegion(hdc: HANDLE, x: i32, y: i32) -> BOOL;
	pub fn PtVisible(hdc: HANDLE, x: i32, y: i32) -> BOOL;
	pub fn Rectangle(hdc: HANDLE, left: i32, top: i32, right: i32, bottom: i32) -> BOOL;
	pub fn RectInRegion(hrgn: HANDLE, lprect: PCVOID) -> BOOL;
	pub fn RestoreDC(hdc: HANDLE, nSavedDC: i32) -> BOOL;
	pub fn RoundRect(hdc: HANDLE, left: i32, top: i32, right: i32, bottom: i32, width: i32, height: i32) -> BOOL;
	pub fn SaveDC(hdc: HANDLE) -> i32;
	pub fn SelectObject(hdc: HANDLE, h: HANDLE) -> HANDLE;
	pub fn SetBkMode(hdc: HANDLE, mode: i32) -> i32;
}
