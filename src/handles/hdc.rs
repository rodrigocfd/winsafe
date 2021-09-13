#![allow(non_snake_case)]

use crate::aliases::WinResult;
use crate::co;
use crate::ffi::{gdi32, msimg32};
use crate::funcs::GetLastError;
use crate::handles::{HBITMAP, HBRUSH, HFONT, HPEN, HRGN};
use crate::privs::{bool_to_winresult, CLR_INVALID, GDI_ERROR};
use crate::structs::{COLORREF, POINT, RECT, SIZE, TEXTMETRIC};
use crate::various::WString;

pub_struct_handle! {
	/// Handle to a
	/// [device context](https://docs.microsoft.com/en-us/windows/win32/winprog/windows-data-types#hdc).
	HDC
}

impl HDC {
	/// [`AborthPath`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-abortpath)
	/// method.
	pub fn AbortPath(self) -> WinResult<()> {
		bool_to_winresult(unsafe { gdi32::AbortPath(self.ptr) })
	}

	/// [`AngleArc`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-anglearc)
	/// method.
	pub fn AngleArc(self,
		center: POINT, radius: u32,
		start_angle: f32, sweep_angle: f32) -> WinResult<()>
	{
		bool_to_winresult(
			unsafe {
				gdi32::AngleArc(
					self.ptr,
					center.x, center.y,
					radius, start_angle, sweep_angle,
				)
			},
		)
	}

	/// [`BeginPath`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-beginpath)
	/// method.
	pub fn BeginPath(self) -> WinResult<()> {
		bool_to_winresult(unsafe { gdi32::BeginPath(self.ptr) })
	}

	/// [`BitBlt`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-bitblt)
	/// method.
	pub fn BitBlt(self,
		dest_top_left: POINT, sz: SIZE,
		hdc_src: HDC, src_top_left: POINT, rop: co::ROP) -> WinResult<()>
	{
		bool_to_winresult(
			unsafe {
				gdi32::BitBlt(
					self.ptr,
					dest_top_left.x, dest_top_left.y,
					sz.cx, sz.cy,
					hdc_src.ptr,
					src_top_left.x, src_top_left.y,
					rop.0,
				)
			},
		)
	}

	/// [`CancelDC`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-canceldc)
	/// method.
	pub fn CancelDC(self) -> WinResult<()> {
		bool_to_winresult(unsafe { gdi32::CancelDC(self.ptr) })
	}

	/// [`Chord`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-chord)
	/// method.
	pub fn Chord(self,
		bounds: RECT, start_radial: POINT, end_radial: POINT) -> WinResult<()>
	{
		bool_to_winresult(
			unsafe {
				gdi32::Chord(
					self.ptr,
					bounds.left, bounds.top, bounds.right, bounds.bottom,
					start_radial.x, start_radial.y,
					end_radial.x, end_radial.y,
				)
			},
		)
	}

	/// [`CreateCompatibleBitmap`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-createcompatiblebitmap)
	/// method.
	///
	/// **Note:** Must be paired with an
	/// [`HBITMAP::DeleteObject`](crate::HBITMAP::DeleteObject) call.
	pub fn CreateCompatibleBitmap(self, cx: i32, cy: i32) -> WinResult<HBITMAP> {
		unsafe { gdi32::CreateCompatibleBitmap(self.ptr, cx, cy).as_mut() }
			.map(|ptr| HBITMAP { ptr })
			.ok_or_else(|| GetLastError())
	}

	/// [`CreateCompatibleDC`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-createcompatibledc)
	/// method.
	///
	/// **Note:** Must be paired with an [`HDC::DeleteDC`](crate::HDC::DeleteDC)
	/// call.
	pub fn CreateCompatibleDC(self) -> WinResult<HDC> {
		unsafe { gdi32::CreateCompatibleDC(self.ptr).as_mut() }
			.map(|ptr| Self { ptr })
			.ok_or_else(|| GetLastError())
	}

	/// [`DeleteDC`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-deletedc)
	/// method.
	pub fn DeleteDC(self) -> WinResult<()> {
		bool_to_winresult(unsafe { gdi32::DeleteDC(self.ptr) })
	}

	/// [`EndPath`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-endpath)
	/// method.
	pub fn EndPath(self) -> WinResult<()> {
		bool_to_winresult(unsafe { gdi32::EndPath(self.ptr) })
	}

	/// [`FillPath`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-fillpath)
	/// method.
	pub fn FillPath(self) -> WinResult<()> {
		bool_to_winresult(unsafe { gdi32::FillPath(self.ptr) })
	}

	/// [`FillRect`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-fillrect)
	/// method.
	pub fn FillRect(self, rc: RECT, hbr: HBRUSH) -> WinResult<()> {
		match unsafe {
			gdi32::FillRect(self.ptr, &rc as *const _ as _, hbr.ptr)
		} {
			0 => Err(GetLastError()),
			_ => Ok(()),
		}
	}

	/// [`GetDCBrushColor`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-getdcbrushcolor)
	/// method.
	pub fn GetDCBrushColor(self) -> WinResult<COLORREF> {
		match unsafe { gdi32::GetDCBrushColor(self.ptr) } {
			CLR_INVALID => Err(GetLastError()),
			color => Ok(COLORREF(color)),
		}
	}

	/// [`GetDCPenColor`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-getdcpencolor)
	/// method.
	pub fn GetDCPenColor(self) -> WinResult<COLORREF> {
		match unsafe { gdi32::GetDCPenColor(self.ptr) } {
			CLR_INVALID => Err(GetLastError()),
			color => Ok(COLORREF(color)),
		}
	}

	/// [`GetDeviceCaps`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-getdevicecaps)
	/// method.
	pub fn GetDeviceCaps(self, index: co::GDC) -> i32 {
		unsafe { gdi32::GetDeviceCaps(self.ptr, index.0) }
	}

	/// [`GetTextColor`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-gettextcolor)
	/// method.
	pub fn GetTextColor(self) -> WinResult<COLORREF> {
		match unsafe { gdi32::GetTextColor(self.ptr) } {
			CLR_INVALID => Err(GetLastError()),
			color => Ok(COLORREF(color)),
		}
	}

	/// [`GetTextExtentPoint32`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-gettextextentpoint32w)
	/// method.
	pub fn GetTextExtentPoint32(self, text: &str) -> WinResult<SIZE> {
		let mut sz = SIZE::default();
		bool_to_winresult(
			unsafe {
				gdi32::GetTextExtentPoint32W(
					self.ptr,
					WString::from_str(text).as_ptr(),
					text.chars().count() as _,
					&mut sz as *mut _ as _,
				)
			},
		).map(|_| sz)
	}

	/// [`GetTextMetrics`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-gettextmetricsw)
	/// method.
	pub fn GetTextMetrics(self, tm: &mut TEXTMETRIC) -> WinResult<()> {
		bool_to_winresult(
			unsafe { gdi32::GetTextMetricsW(self.ptr, tm as *mut _ as _) },
		)
	}

	/// [`LineTo`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-lineto)
	/// method.
	pub fn LineTo(self, x: i32, y: i32) -> WinResult<()> {
		bool_to_winresult(unsafe { gdi32::LineTo(self.ptr, x, y) })
	}

	/// [`MoveToEx`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-movetoex)
	/// method.
	pub fn MoveToEx(self,
		x: i32, y: i32, pt: Option<&mut POINT>) -> WinResult<()>
	{
		bool_to_winresult(
			unsafe {
				gdi32::MoveToEx(
					self.ptr,
					x, y,
					pt.map_or(std::ptr::null_mut(), |lp| lp as *mut _ as _),
				)
			},
		)
	}

	/// [`PatBlt`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-patblt)
	/// method.
	pub fn PatBlt(self,
		top_left: POINT, sz: SIZE, rop: co::ROP) -> WinResult<()>
	{
		bool_to_winresult(
			unsafe {
				gdi32::PatBlt(self.ptr, top_left.x, top_left.y, sz.cx, sz.cy, rop.0)
			},
		)
	}

	/// [`PathToRegion`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-pathtoregion)
	/// method.
	///
	/// **Note:** Must be paired with an
	/// [`HRGN::DeleteObject`](crate::HRGN::DeleteObject) call.
	pub fn PathToRegion(self) -> WinResult<HRGN> {
		unsafe { gdi32::PathToRegion(self.ptr).as_mut() }
			.map(|ptr| HRGN { ptr })
			.ok_or_else(|| GetLastError())
	}

	/// [`Pie`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-pie)
	/// method.
	pub fn Pie(self,
		bounds: RECT, radial_1: POINT, radial_2: POINT) -> WinResult<()>
	{
		bool_to_winresult(
			unsafe {
				gdi32::Pie(
					self.ptr,
					bounds.left, bounds.top, bounds.right, bounds.bottom,
					radial_1.x, radial_1.y,
					radial_2.y, radial_2.y,
				)
			},
		)
	}

	/// [`PolyBezier`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-polybezier)
	/// method.
	pub fn PolyBezier(self, pts: &[POINT]) -> WinResult<()> {
		bool_to_winresult(
			unsafe {
				gdi32::PolyBezier(
					self.ptr,
					pts.as_ptr() as _,
					pts.len() as _,
				)
			},
		)
	}

	/// [`PolyBezierTo`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-polybezierto)
	/// method.
	pub fn PolyBezierTo(self, pts: &[POINT]) -> WinResult<()> {
		bool_to_winresult(
			unsafe {
				gdi32::PolyBezierTo(
					self.ptr,
					pts.as_ptr() as _,
					pts.len() as _,
				)
			},
		)
	}

	/// [`Polyline`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-polyline)
	/// method.
	pub fn Polyline(self, pts: &[POINT]) -> WinResult<()> {
		bool_to_winresult(
			unsafe {
				gdi32::Polyline(
					self.ptr,
					pts.as_ptr() as _,
					pts.len() as _,
				)
			},
		)
	}

	/// [`PolylineTo`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-polylineto)
	/// method.
	pub fn PolylineTo(self, pts: &[POINT]) -> WinResult<()> {
		bool_to_winresult(
			unsafe {
				gdi32::PolylineTo(
					self.ptr,
					pts.as_ptr() as _,
					pts.len() as _,
				)
			},
		)
	}

	/// [`PtVisible`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-ptvisible)
	/// method.
	pub fn PtVisible(self, x: i32, y: i32) -> WinResult<bool> {
		match unsafe { gdi32::PtVisible(self.ptr, x, y) } {
			-1 => Err(GetLastError()),
			0 => Ok(false),
			_ => Ok(true),
		}
	}

	/// [`Rectangle`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-rectangle)
	/// method.
	pub fn Rectangle(self, bounds: RECT) -> WinResult<()> {
		bool_to_winresult(
			unsafe {
				gdi32::Rectangle(self.ptr,
					bounds.left, bounds.top, bounds.right, bounds.bottom)
			},
		)
	}

	/// [`RestoreDC`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-restoredc)
	/// method.
	pub fn RestoreDC(self, saved_dc: i32) -> WinResult<()> {
		bool_to_winresult(unsafe { gdi32::RestoreDC(self.ptr, saved_dc) })
	}

	/// [`RoundRect`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-roundrect)
	/// method.
	pub fn RoundRect(self, bounds: RECT, sz: SIZE) -> WinResult<()> {
		bool_to_winresult(
			unsafe {
				gdi32::RoundRect(
					self.ptr,
					bounds.left, bounds.top, bounds.right, bounds.bottom,
					sz.cx, sz.cy,
				)
			},
		)
	}

	/// [`SaveDC`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-savedc)
	/// method.
	pub fn SaveDC(self) -> WinResult<i32> {
		match unsafe { gdi32::SaveDC(self.ptr) } {
			0 => Err(GetLastError()),
			v => Ok(v),
		}
	}

	/// [`SelectObject`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-selectobject)
	/// method for [`HBITMAP`](crate::HBITMAP).
	pub fn SelectObjectBitmap(self, hbmp: HBITMAP) -> WinResult<HBITMAP> {
		unsafe { gdi32::SelectObject(self.ptr, hbmp.ptr).as_mut() }
			.map(|ptr| HBITMAP { ptr })
			.ok_or_else(|| GetLastError())
	}

	/// [`SelectObject`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-selectobject)
	/// method for [`HBRUSH`](crate::HBRUSH).
	pub fn SelectObjectBrush(self, hbr: HBRUSH) -> WinResult<HBRUSH> {
		unsafe { gdi32::SelectObject(self.ptr, hbr.ptr).as_mut() }
			.map(|ptr| HBRUSH { ptr })
			.ok_or_else(|| GetLastError())
	}

	/// [`SelectObject`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-selectobject)
	/// method for [`HFONT`](crate::HFONT).
	pub fn SelectObjectFont(self, hfont: HFONT) -> WinResult<HFONT> {
		unsafe { gdi32::SelectObject(self.ptr, hfont.ptr).as_mut() }
			.map(|ptr| HFONT { ptr })
			.ok_or_else(|| GetLastError())
	}

	/// [`SelectObject`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-selectobject)
	/// method for [`HPEN`](crate::HPEN).
	pub fn SelectObjectPen(self, hpen: HPEN) -> WinResult<HPEN> {
		unsafe { gdi32::SelectObject(self.ptr, hpen.ptr).as_mut() }
			.map(|ptr| HPEN { ptr })
			.ok_or_else(|| GetLastError())
	}

	/// [`SelectObject`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-selectobject)
	/// method for [`HRGN`](crate::HRGN).
	pub fn SelectObjectRgn(self, hrgn: HRGN) -> WinResult<co::REGION> {
		unsafe { gdi32::SelectObject(self.ptr, hrgn.ptr).as_mut() }
			.map(|ptr| co::REGION(ptr as *mut _ as _))
			.ok_or_else(|| GetLastError())
	}

	/// [`SetArcDirection`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-setarcdirection)
	/// method.
	pub fn SetArcDirection(self, dir: co::AD) -> WinResult<co::AD> {
		match unsafe { gdi32::SetArcDirection(self.ptr, dir.0) } {
			0 => Err(GetLastError()),
			v => Ok(co::AD(v)),
		}
	}

	/// [`SetBkMode`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-setbkmode)
	/// method.
	pub fn SetBkMode(self, mode: co::BKMODE) -> WinResult<co::BKMODE> {
		match unsafe { gdi32::SetBkMode(self.ptr, mode.0) } {
			0 => Err(GetLastError()),
			v => Ok(co::BKMODE(v)),
		}
	}

	/// [`SetDCBrushColor`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-setdcbrushcolor)
	pub fn SetDCBrushColor(self, color: COLORREF) -> WinResult<COLORREF> {
		match unsafe { gdi32::SetDCBrushColor(self.ptr, color.0) } {
			CLR_INVALID => Err(GetLastError()),
			old => Ok(COLORREF(old)),
		}
	}

	/// [`SetDCPenColor`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-setdcpencolor)
	pub fn SetDCPenColor(self, color: COLORREF) -> WinResult<COLORREF> {
		match unsafe { gdi32::SetDCPenColor(self.ptr, color.0) } {
			CLR_INVALID => Err(GetLastError()),
			old => Ok(COLORREF(old)),
		}
	}

	/// [`SetGraphicsMode`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-setgraphicsmode)
	/// method.
	pub fn SetGraphicsMode(self, mode: co::GM) -> WinResult<co::GM> {
		match unsafe { gdi32::SetGraphicsMode(self.ptr, mode.0) } {
			0 => Err(GetLastError()),
			v => Ok(co::GM(v))
		}
	}

	/// [`SetTextAlign`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-settextalign)
	/// method.
	pub fn SetTextAlign(self, align: co::TA) -> WinResult<co::TA> {
		match unsafe { gdi32::SetTextAlign(self.ptr, align.0) } {
			GDI_ERROR => Err(GetLastError()),
			ta => Ok(co::TA(ta)),
		}
	}

	/// [`SetTextColor`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-settextcolor)
	/// method.
	pub fn SetTextColor(self, color: COLORREF) -> WinResult<COLORREF> {
		match unsafe { gdi32::SetTextColor(self.ptr, color.0) } {
			CLR_INVALID => Err(GetLastError()),
			old => Ok(COLORREF(old)),
		}
	}

	/// [`SetTextJustification`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-settextjustification)
	/// method.
	pub fn SetTextJustification(self, extra: i32, count: i32) -> WinResult<()> {
		bool_to_winresult(
			unsafe { gdi32::SetTextJustification(self.ptr, extra, count) },
		)
	}

	/// [`SetViewportExtEx`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-setviewportextex)
	/// method.
	pub fn SetViewportExtEx(self, x: i32, y: i32) -> WinResult<SIZE> {
		let mut sz = SIZE::default();
		bool_to_winresult(
			unsafe {
				gdi32::SetViewportExtEx(self.ptr, x, y, &mut sz as *mut _ as _)
			}
		).map(|_| sz)
	}

	/// [`SetViewportOrgEx`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-setviewportorgex)
	/// method.
	pub fn SetViewportOrgEx(self, x: i32, y: i32) -> WinResult<POINT> {
		let mut pt = POINT::default();
		bool_to_winresult(
			unsafe {
				gdi32::SetViewportOrgEx(self.ptr, x, y, &mut pt as *mut _ as _)
			}
		).map(|_| pt)
	}

	/// [`SetWindowExtEx`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-setwindowextex)
	/// method.
	pub fn SetWindowExtEx(self, x: i32, y: i32) -> WinResult<SIZE> {
		let mut sz = SIZE::default();
		bool_to_winresult(
			unsafe {
				gdi32::SetWindowExtEx(self.ptr, x, y, &mut sz as *mut _ as _)
			}
		).map(|_| sz)
	}

	/// [`SetWindowOrgEx`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-setwindoworgex)
	/// method.
	pub fn SetWindowOrgEx(self, x: i32, y: i32) -> WinResult<POINT> {
		let mut pt = POINT::default();
		bool_to_winresult(
			unsafe {
				gdi32::SetWindowOrgEx(self.ptr, x, y, &mut pt as *mut _ as _)
			}
		).map(|_| pt)
	}

	/// [`StrokeAndFillPath`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-strokeandfillpath)
	/// method.
	pub fn StrokeAndFillPath(self) -> WinResult<()> {
		bool_to_winresult(unsafe { gdi32::StrokeAndFillPath(self.ptr) })
	}

	/// [`StrokePath`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-strokepath)
	/// method.
	pub fn StrokePath(self) -> WinResult<()> {
		bool_to_winresult(unsafe { gdi32::StrokePath(self.ptr) })
	}

	/// [`TextOut`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-textoutw)
	/// method.
	pub fn TextOut(self, x: i32, y: i32, text: &str) -> WinResult<()> {
		bool_to_winresult(
			unsafe {
				gdi32::TextOutW(
					self.ptr,
					x, y,
					WString::from_str(text).as_ptr(),
					text.len() as _,
				)
			},
		)
	}

	/// [`TransparentBlt`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-transparentblt)
	/// method.
	pub fn TransparentBlt(self,
		dest_top_left: POINT, dest_sz: SIZE,
		hdc_src: HDC,
		src_top_left: POINT, src_sz: SIZE,
		color_transparent: COLORREF) -> WinResult<()>
	{
		bool_to_winresult(
			unsafe {
				msimg32::TransparentBlt(
					self.ptr,
					dest_top_left.x, dest_top_left.y,
					dest_sz.cx, dest_sz.cy,
					hdc_src.ptr,
					src_top_left.x, src_top_left.y,
					src_sz.cx, src_sz.cy,
					color_transparent.0,
				)
			},
		)
	}

	/// [`WidenPath`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-widenpath)
	/// method.
	pub fn WidenPath(self) -> WinResult<()>  {
		bool_to_winresult(unsafe { gdi32::WidenPath(self.ptr) })
	}
}
