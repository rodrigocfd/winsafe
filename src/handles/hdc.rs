#![allow(non_snake_case)]

use crate::aliases::WinResult;
use crate::co;
use crate::ffi::{gdi32, msimg32};
use crate::funcs::GetLastError;
use crate::handles::{HBITMAP, HBRUSH, HFONT, HPEN, HRGN};
use crate::privs::{
	bool_to_winresult,
	CLR_INVALID,
	GDI_ERROR,
	nonzero_to_winresult,
	ref_as_pcvoid,
	ref_as_pvoid,
};
use crate::structs::{COLORREF, POINT, RECT, SIZE, TEXTMETRIC};
use crate::WString;

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
		x: i32, y: i32,
		r: u32, StartAngle: f32, SweepAngle: f32) -> WinResult<()>
	{
		bool_to_winresult(
			unsafe { gdi32::AngleArc(self.ptr, x, y, r, StartAngle, SweepAngle) },
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
		x: i32, y: i32, cx: i32, cy: i32,
		hdcSrc: HDC,
		x1: i32, y1: i32,
		rop: co::ROP) -> WinResult<()>
	{
		bool_to_winresult(
			unsafe {
				gdi32::BitBlt(
					self.ptr,
					x, y, cx, cy,
					hdcSrc.ptr,
					x1, y1,
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
		x1: i32, y1: i32,
		x2: i32, y2: i32,
		x3: i32, y3: i32,
		x4: i32, y4: i32) -> WinResult<()>
	{
		bool_to_winresult(
			unsafe { gdi32::Chord(self.ptr, x1, y1, x2, y2, x3, y3, x4, y4) },
		)
	}

	/// [`CreateCompatibleDC`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-createcompatibledc)
	/// method.
	///
	/// **Note:** Must be paired with a [`DeleteDC`](crate::HDC::DeleteDC) call.
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
	pub fn FillRect(self, lprc: RECT, hbr: HBRUSH) -> WinResult<()> {
		nonzero_to_winresult(
			unsafe { gdi32::FillRect(self.ptr, ref_as_pcvoid(&lprc), hbr.ptr) },
		).map(|_| ())
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
	pub fn GetTextExtentPoint32(self, lpString: &str) -> WinResult<SIZE> {
		let mut sz = SIZE::default();
		bool_to_winresult(
			unsafe {
				gdi32::GetTextExtentPoint32W(
					self.ptr,
					WString::from_str(lpString).as_ptr(),
					lpString.chars().count() as _,
					ref_as_pvoid(&mut sz),
				)
			},
		).map(|_| sz)
	}

	/// [`GetTextMetrics`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-gettextmetricsw)
	/// method.
	pub fn GetTextMetrics(self, lptm: &mut TEXTMETRIC) -> WinResult<()> {
		bool_to_winresult(
			unsafe { gdi32::GetTextMetricsW(self.ptr, ref_as_pvoid(lptm)) },
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
		x: i32, y: i32, lppt: Option<&mut POINT>) -> WinResult<()>
	{
		bool_to_winresult(
			unsafe {
				gdi32::MoveToEx(
					self.ptr,
					x,
					y,
					lppt.map_or(std::ptr::null_mut(), |lp| ref_as_pvoid(lp)),
				)
			},
		)
	}

	/// [`PatBlt`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-patblt)
	/// method.
	pub fn PatBlt(self, x: i32, y: i32,
		w: i32, h: i32, rop: co::ROP) -> WinResult<()>
	{
		bool_to_winresult(unsafe { gdi32::PatBlt(self.ptr, x, y, w, h, rop.0) })
	}

	/// [`PathToRegion`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-pathtoregion)
	/// method.
	///
	/// **Note:** Must be paired with a
	/// [`DeleteObject`](crate::HRGN::DeleteObject) call.
	pub fn PathToRegion(self) -> WinResult<HRGN> {
		unsafe { gdi32::PathToRegion(self.ptr).as_mut() }
			.map(|ptr| HRGN { ptr })
			.ok_or_else(|| GetLastError())
	}

	/// [`Pie`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-pie)
	/// method.
	pub fn Pie(self,
		left: i32, top: i32, right: i32, bottom: i32,
		xr1: i32, yr1: i32, xr2: i32, yr2: i32) -> WinResult<()>
	{
		bool_to_winresult(
			unsafe {
				gdi32::Pie(self.ptr, left, top, right, bottom, xr1, yr1, xr2, yr2)
			},
		)
	}

	/// [`PolyBezier`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-polybezier)
	/// method.
	pub fn PolyBezier(self, apt: &[POINT]) -> WinResult<()> {
		bool_to_winresult(
			unsafe {
				gdi32::PolyBezier(
					self.ptr,
					ref_as_pcvoid(&apt[0]),
					apt.len() as _,
				)
			},
		)
	}

	/// [`PolyBezierTo`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-polybezierto)
	/// method.
	pub fn PolyBezierTo(self, apt: &[POINT]) -> WinResult<()> {
		bool_to_winresult(
			unsafe {
				gdi32::PolyBezierTo(
					self.ptr,
					ref_as_pcvoid(&apt[0]),
					apt.len() as _,
				)
			},
		)
	}

	/// [`Polyline`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-polyline)
	/// method.
	pub fn Polyline(self, apt: &[POINT]) -> WinResult<()> {
		bool_to_winresult(
			unsafe {
				gdi32::Polyline(
					self.ptr,
					ref_as_pcvoid(&apt[0]),
					apt.len() as _,
				)
			},
		)
	}

	/// [`PolylineTo`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-polylineto)
	/// method.
	pub fn PolylineTo(self, apt: &[POINT]) -> WinResult<()> {
		bool_to_winresult(
			unsafe {
				gdi32::PolylineTo(
					self.ptr,
					ref_as_pcvoid(&apt[0]),
					apt.len() as _,
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
	pub fn Rectangle(self,
		left: i32, top: i32, right: i32, bottom: i32) -> WinResult<()>
	{
		bool_to_winresult(
			unsafe { gdi32::Rectangle(self.ptr, left, top, right, bottom) },
		)
	}

	/// [`RestoreDC`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-restoredc)
	/// method.
	pub fn RestoreDC(self, nSavedDC: i32) -> WinResult<()> {
		bool_to_winresult(unsafe { gdi32::RestoreDC(self.ptr, nSavedDC) })
	}

	/// [`RoundRect`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-roundrect)
	/// method.
	pub fn RoundRect(self,
		left: i32, top: i32, right: i32, bottom: i32,
		width: i32, height: i32) -> WinResult<()>
	{
		bool_to_winresult(
			unsafe {
				gdi32::RoundRect(self.ptr, left, top, right, bottom, width, height)
			},
		)
	}

	/// [`SaveDC`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-savedc)
	/// method.
	pub fn SaveDC(self) -> WinResult<i32> {
		nonzero_to_winresult(unsafe { gdi32::SaveDC(self.ptr) })
	}

	/// [`SelectObject`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-selectobject)
	/// method for [`HBITMAP`](crate::HBITMAP).
	pub fn SelectObjectBitmap(self, h: HBITMAP) -> WinResult<HBITMAP> {
		unsafe { gdi32::SelectObject(self.ptr, h.ptr).as_mut() }
			.map(|ptr| HBITMAP { ptr })
			.ok_or_else(|| GetLastError())
	}

	/// [`SelectObject`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-selectobject)
	/// method for [`HBRUSH`](crate::HBRUSH).
	pub fn SelectObjectBrush(self, h: HBRUSH) -> WinResult<HBRUSH> {
		unsafe { gdi32::SelectObject(self.ptr, h.ptr).as_mut() }
			.map(|ptr| HBRUSH { ptr })
			.ok_or_else(|| GetLastError())
	}

	/// [`SelectObject`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-selectobject)
	/// method for [`HFONT`](crate::HFONT).
	pub fn SelectObjectFont(self, h: HFONT) -> WinResult<HFONT> {
		unsafe { gdi32::SelectObject(self.ptr, h.ptr).as_mut() }
			.map(|ptr| HFONT { ptr })
			.ok_or_else(|| GetLastError())
	}

	/// [`SelectObject`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-selectobject)
	/// method for [`HPEN`](crate::HPEN).
	pub fn SelectObjectPen(self, h: HPEN) -> WinResult<HPEN> {
		unsafe { gdi32::SelectObject(self.ptr, h.ptr).as_mut() }
			.map(|ptr| HPEN { ptr })
			.ok_or_else(|| GetLastError())
	}

	/// [`SelectObject`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-selectobject)
	/// method for [`HRGN`](crate::HRGN).
	pub fn SelectObjectRgn(self, h: HRGN) -> WinResult<co::REGION> {
		unsafe { gdi32::SelectObject(self.ptr, h.ptr).as_mut() }
			.map(|ptr| co::REGION(ptr as *mut _ as _))
			.ok_or_else(|| GetLastError())
	}

	/// [`SetArcDirection`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-setarcdirection)
	/// method.
	pub fn SetArcDirection(self, dir: co::AD) -> WinResult<co::AD> {
		nonzero_to_winresult(unsafe { gdi32::SetArcDirection(self.ptr, dir.0) })
			.map(|v| co::AD(v))
	}

	/// [`SetBkMode`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-setbkmode)
	/// method.
	pub fn SetBkMode(self, mode: co::BKMODE) -> WinResult<co::BKMODE> {
		nonzero_to_winresult(unsafe { gdi32::SetBkMode(self.ptr, mode.0) })
			.map(|v| co::BKMODE(v))
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
	pub fn SetGraphicsMode(self, iMode: co::GM) -> WinResult<co::GM> {
		nonzero_to_winresult(unsafe { gdi32::SetGraphicsMode(self.ptr, iMode.0) })
			.map(|v| co::GM(v))
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
		let mut lpsz = SIZE::default();
		bool_to_winresult(
			unsafe {
				gdi32::SetViewportExtEx(self.ptr, x, y, ref_as_pvoid(&mut lpsz))
			}
		).map(|_| lpsz)
	}

	/// [`SetViewportOrgEx`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-setviewportorgex)
	/// method.
	pub fn SetViewportOrgEx(self, x: i32, y: i32) -> WinResult<POINT> {
		let mut lppt = POINT::default();
		bool_to_winresult(
			unsafe {
				gdi32::SetViewportOrgEx(self.ptr, x, y, ref_as_pvoid(&mut lppt))
			}
		).map(|_| lppt)
	}

	/// [`SetWindowExtEx`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-setwindowextex)
	/// method.
	pub fn SetWindowExtEx(self, x: i32, y: i32) -> WinResult<SIZE> {
		let mut lpsz = SIZE::default();
		bool_to_winresult(
			unsafe {
				gdi32::SetWindowExtEx(self.ptr, x, y, ref_as_pvoid(&mut lpsz))
			}
		).map(|_| lpsz)
	}

	/// [`SetWindowOrgEx`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-setwindoworgex)
	/// method.
	pub fn SetWindowOrgEx(self, x: i32, y: i32) -> WinResult<POINT> {
		let mut lppt = POINT::default();
		bool_to_winresult(
			unsafe {
				gdi32::SetWindowOrgEx(self.ptr, x, y, ref_as_pvoid(&mut lppt))
			}
		).map(|_| lppt)
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
	pub fn TextOut(self, x: i32, y: i32, lpString: &str) -> WinResult<()> {
		bool_to_winresult(
			unsafe {
				gdi32::TextOutW(
					self.ptr,
					x, y,
					WString::from_str(lpString).as_ptr(),
					lpString.len() as _,
				)
			},
		)
	}

	/// [`TransparentBlt`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-transparentblt)
	/// method.
	pub fn TransparentBlt(self,
		xoriginDest: i32, yoriginDest: i32, wDest: i32, hDest: i32,
		hdcSrc: HDC,
		xoriginSrc: i32, yoriginSrc: i32, wSrc: i32, hSrc: i32,
		crTransparent: COLORREF) -> WinResult<()>
	{
		bool_to_winresult(
			unsafe {
				msimg32::TransparentBlt(
					self.ptr,
					xoriginDest, yoriginDest, wDest, hDest,
					hdcSrc.ptr,
					xoriginSrc, yoriginSrc, wSrc, hSrc,
					crTransparent.0,
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
