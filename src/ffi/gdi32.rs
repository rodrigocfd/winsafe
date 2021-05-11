//! Raw bindings to gdi32.lib functions.

use crate::ffi::{BOOL, HANDLE, PCSTR, PCVOID, PVOID};

#[link(name = "gdi32")]
extern "system" {
	pub fn AbortPath(_: HANDLE) -> BOOL;
	pub fn AngleArc(_: HANDLE, _: i32, _: i32, _: u32, _: f32, _: f32) -> BOOL;
	pub fn BeginPath(_: HANDLE) -> BOOL;
	pub fn BitBlt(_: HANDLE, _: i32, _: i32, _: i32, _: i32, _: HANDLE, _: i32, _: i32, _: u32) -> BOOL;
	pub fn CancelDC(_:HANDLE) -> BOOL;
	pub fn Chord(_: HANDLE, _: i32, _: i32, _: i32, _: i32, _: i32, _: i32, _: i32, _: i32) -> BOOL;
	pub fn CreateBrushIndirect(_: PCVOID) -> HANDLE;
	pub fn CreateCompatibleDC(_: HANDLE) -> HANDLE;
	pub fn CreateFontIndirectW(_: PCVOID) -> HANDLE;
	pub fn CreateFontW(_: i32, _: i32, _: i32, _: i32, _: i32, _: u32, _: u32, _: u32, _: u32, _: u32, _: u32, _: u32, _: u32, _: PCSTR) -> HANDLE;
	pub fn CreateHatchBrush(_: i32, _: u32) -> HANDLE;
	pub fn CreatePatternBrush(_: HANDLE) -> HANDLE;
	pub fn CreateSolidBrush(_: u32) -> HANDLE;
	pub fn DeleteDC(_: HANDLE) -> BOOL;
	pub fn DeleteObject(_: HANDLE) -> BOOL;
	pub fn EndPath(_: HANDLE) -> BOOL;
	pub fn FillPath(_: HANDLE) -> BOOL;
	pub fn GetDeviceCaps(_: HANDLE, _: i32) -> i32;
	pub fn GetSysColorBrush(_: i32) -> HANDLE;
	pub fn GetTextColor(_: HANDLE) -> u32;
	pub fn GetTextExtentPoint32W(_: HANDLE, _: PCSTR, _: i32, _: PVOID) -> BOOL;
	pub fn GetTextMetricsW(_: HANDLE, _: PVOID) -> BOOL;
	pub fn LineTo(_: HANDLE, _: i32, _: i32) -> BOOL;
	pub fn MoveToEx(_: HANDLE, _: i32, _: i32, _: PVOID) -> BOOL;
	pub fn OffsetClipRgn(_: HANDLE, _: i32, _: i32) -> i32;
	pub fn OffsetRgn(_: HANDLE, _: i32, _: i32) -> i32;
	pub fn PatBlt(_: HANDLE, _: i32, _: i32, _: i32, _: i32, _: u32) -> BOOL;
	pub fn PathToRegion(_: HANDLE) -> HANDLE;
	pub fn Pie(_: HANDLE, _: i32, _: i32, _: i32, _: i32, _: i32, _: i32, _: i32, _: i32) -> BOOL;
	pub fn PolyBezier(_: HANDLE, _: PCVOID, _: u32) -> BOOL;
	pub fn PolyBezierTo(_: HANDLE, _: PCVOID, _: u32) -> BOOL;
	pub fn Polyline(_: HANDLE, _: PCVOID, _: u32) -> BOOL;
	pub fn PolylineTo(_: HANDLE, _: PCVOID, _: u32) -> BOOL;
	pub fn PtInRegion(_: HANDLE, _: i32, _: i32) -> BOOL;
	pub fn PtVisible(_: HANDLE, _: i32, _: i32) -> BOOL;
	pub fn Rectangle(_: HANDLE, _: i32, _: i32, _: i32, _: i32) -> BOOL;
	pub fn RectInRegion(_: HANDLE, _: PCVOID) -> BOOL;
	pub fn RestoreDC(_: HANDLE, _: i32) -> BOOL;
	pub fn RoundRect(_: HANDLE, _: i32, _: i32, _: i32, _: i32, _: i32, _: i32) -> BOOL;
	pub fn SaveDC(_: HANDLE) -> i32;
	pub fn SelectObject(_: HANDLE, _: HANDLE) -> HANDLE;
	pub fn SetArcDirection(_: HANDLE, _: i32) -> i32;
	pub fn SetBkMode(_: HANDLE, _: i32) -> i32;
	pub fn SetDCPenColor(_: HANDLE, _: u32) -> u32;
	pub fn SetGraphicsMode(_: HANDLE, _: i32) -> i32;
	pub fn SetTextAlign(_: HANDLE, _: u32) -> u32;
	pub fn SetTextColor(_: HANDLE, _: u32) -> u32;
	pub fn SetTextJustification(_: HANDLE, _: i32, _: i32) -> BOOL;
	pub fn SetViewportExtEx(_: HANDLE, _: i32, _: i32, _: PVOID) -> BOOL;
	pub fn SetViewportOrgEx(_: HANDLE, _: i32, _: i32, _: PVOID) -> BOOL;
	pub fn SetWindowExtEx(_: HANDLE, _: i32, _: i32, _: PVOID) -> BOOL;
	pub fn SetWindowOrgEx(_: HANDLE, _: i32, _: i32, _: PVOID) -> BOOL;
	pub fn StrokeAndFillPath(_: HANDLE) -> BOOL;
	pub fn StrokePath(_: HANDLE) -> BOOL;
	pub fn TextOutW(_: HANDLE, _: i32, _: i32, _: PCSTR, _: i32) -> BOOL;
	pub fn WidenPath(_: HANDLE) -> BOOL;
}
