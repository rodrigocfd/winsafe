//! Raw bindings to gdi32.lib functions.

use crate::ffi::{BOOL, HANDLE, PCSTR, PCVOID, PVOID};

#[link(name = "gdi32")]
extern "system" {
	pub fn AbortPath(hdc: HANDLE) -> BOOL;
	pub fn AngleArc(hdc: HANDLE, x: i32, y: i32, r: u32, StartAngle: f32, SweepAngle: f32) -> BOOL;
	pub fn BeginPath(hdc: HANDLE) -> BOOL;
	pub fn BitBlt(hdc: HANDLE, x: i32, y: i32, cx: i32, cy: i32, hdcSrc: HANDLE, x1: i32, y1: i32, rop: u32) -> BOOL;
	pub fn CancelDC(hdc:HANDLE) -> BOOL;
	pub fn Chord(hdc: HANDLE, x1: i32, y1: i32, x2: i32, y2: i32, x3: i32, y3: i32, x4: i32, y4: i32) -> BOOL;
	pub fn CreateCompatibleDC(hdc: HANDLE) -> HANDLE;
	pub fn CreateFontIndirectW(lplf: PCVOID) -> HANDLE;
	pub fn CreateFontW(cHeight: i32, cWidth: i32, cEscapement: i32, cOrientation: i32, cWeight: i32, bItalic: u32, bUnderline: u32, bStrikeOut: u32, iCharSet: u32, iOutPrecision: u32, iClipPrecision: u32, iQuality: u32, iPitchAndFamily: u32, pszFaceName: PCSTR) -> HANDLE;
	pub fn DeleteDC(hdc: HANDLE) -> BOOL;
	pub fn DeleteObject(ho: HANDLE) -> BOOL;
	pub fn EndPath(hdc: HANDLE) -> BOOL;
	pub fn FillPath(hdc: HANDLE) -> BOOL;
	pub fn GetDeviceCaps(hdc: HANDLE, index: i32) -> i32;
	pub fn GetTextColor(hdc: HANDLE) -> u32;
	pub fn GetTextExtentPoint32W(hdc: HANDLE, lpString: PCSTR, c: i32, psizl: PVOID) -> BOOL;
	pub fn GetTextMetricsW(hdc: HANDLE, lptm: PVOID) -> BOOL;
	pub fn LineTo(hdc: HANDLE, x: i32, y: i32) -> BOOL;
	pub fn MoveToEx(hdc: HANDLE, x: i32, y: i32, lppt: PVOID) -> BOOL;
	pub fn OffsetClipRgn(hrgn: HANDLE, x: i32, y: i32) -> i32;
	pub fn OffsetRgn(hrgn: HANDLE, x: i32, y: i32) -> i32;
	pub fn PathToRegion(hdc: HANDLE) -> HANDLE;
	pub fn Pie(hdc: HANDLE, left: i32, top: i32, right: i32, bottom: i32, xr1: i32, yr1: i32, xr2: i32, yr2: i32) -> BOOL;
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
	pub fn SetArcDirection(hdc: HANDLE, dir: i32) -> i32;
	pub fn SetBkMode(hdc: HANDLE, mode: i32) -> i32;
	pub fn SetDCPenColor(hdc: HANDLE, color: u32) -> u32;
	pub fn SetGraphicsMode(hdc: HANDLE, iMode: i32) -> i32;
	pub fn SetTextAlign(hdc: HANDLE, align: u32) -> u32;
	pub fn SetTextColor(hdc: HANDLE, color: u32) -> u32;
	pub fn SetTextJustification(hdc: HANDLE, extra: i32, count: i32) -> BOOL;
	pub fn SetViewportExtEx(hdc: HANDLE, x: i32, y: i32, lpsz: PVOID) -> BOOL;
	pub fn SetViewportOrgEx(hdc: HANDLE, x: i32, y: i32, lppt: PVOID) -> BOOL;
	pub fn SetWindowExtEx(hdc: HANDLE, x: i32, y: i32, lpsz: PVOID) -> BOOL;
	pub fn SetWindowOrgEx(hdc: HANDLE, x: i32, y: i32, lppt: PVOID) -> BOOL;
	pub fn StrokeAndFillPath(hdc: HANDLE) -> BOOL;
	pub fn StrokePath(hdc: HANDLE) -> BOOL;
	pub fn TextOutW(hdc: HANDLE, x: i32, y: i32, lpString: PCSTR, c: i32) -> BOOL;
	pub fn WidenPath(hdc: HANDLE) -> BOOL;
}
