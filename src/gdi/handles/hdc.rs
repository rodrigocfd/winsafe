#![allow(non_snake_case)]

use crate::{co, gdi};
use crate::gdi::decl::{HFONT, HPEN, TEXTMETRIC};
use crate::gdi::privs::{CLR_INVALID, GDI_ERROR, LF_FACESIZE};
use crate::kernel::decl::{GetLastError, WinResult, WString};
use crate::kernel::privs::bool_to_winresult;
use crate::prelude::Handle;
use crate::user::decl::{
	COLORREF, HBITMAP, HBRUSH, HDC, HRGN, POINT, RECT, SIZE,
};

impl GdiHdc for HDC {}

/// [`HDC`](crate::HDC) methods from `gdi` feature.
#[cfg_attr(docsrs, doc(cfg(feature = "gdi")))]
pub trait GdiHdc: Handle {
	/// [`AborthPath`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-abortpath)
	/// method.
	fn AbortPath(self) -> WinResult<()> {
		bool_to_winresult(unsafe { gdi::ffi::AbortPath(self.as_ptr()) })
	}

	/// [`AngleArc`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-anglearc)
	/// method.
	fn AngleArc(self,
		center: POINT, radius: u32,
		start_angle: f32, sweep_angle: f32) -> WinResult<()>
	{
		bool_to_winresult(
			unsafe {
				gdi::ffi::AngleArc(
					self.as_ptr(),
					center.x, center.y,
					radius, start_angle, sweep_angle,
				)
			},
		)
	}

	/// [`BeginPath`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-beginpath)
	/// method.
	fn BeginPath(self) -> WinResult<()> {
		bool_to_winresult(unsafe { gdi::ffi::BeginPath(self.as_ptr()) })
	}

	/// [`BitBlt`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-bitblt)
	/// method.
	fn BitBlt(self,
		dest_pos: POINT, sz: SIZE,
		hdc_src: HDC, src_src: POINT, rop: co::ROP) -> WinResult<()>
	{
		bool_to_winresult(
			unsafe {
				gdi::ffi::BitBlt(
					self.as_ptr(),
					dest_pos.x, dest_pos.y,
					sz.cx, sz.cy,
					hdc_src.0,
					src_src.x, src_src.y,
					rop.0,
				)
			},
		)
	}

	/// [`CancelDC`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-canceldc)
	/// method.
	fn CancelDC(self) -> WinResult<()> {
		bool_to_winresult(unsafe { gdi::ffi::CancelDC(self.as_ptr()) })
	}

	/// [`Chord`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-chord)
	/// method.
	fn Chord(self,
		bounds: RECT, start_radial: POINT, end_radial: POINT) -> WinResult<()>
	{
		bool_to_winresult(
			unsafe {
				gdi::ffi::Chord(
					self.as_ptr(),
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
	/// [`HBITMAP::DeleteObject`](crate::prelude::HandleGdi::DeleteObject) call.
	#[must_use]
	fn CreateCompatibleBitmap(self, cx: i32, cy: i32) -> WinResult<HBITMAP> {
		unsafe {
			gdi::ffi::CreateCompatibleBitmap(self.as_ptr(), cx, cy).as_mut()
		}.map(|ptr| HBITMAP(ptr))
			.ok_or_else(|| GetLastError())
	}

	/// [`CreateCompatibleDC`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-createcompatibledc)
	/// method.
	///
	/// **Note:** Must be paired with an
	/// [`HDC::DeleteDC`](crate::prelude::GdiHdc::DeleteDC) call.
	#[must_use]
	fn CreateCompatibleDC(self) -> WinResult<HDC> {
		unsafe { gdi::ffi::CreateCompatibleDC(self.as_ptr()).as_mut() }
			.map(|ptr| HDC(ptr))
			.ok_or_else(|| GetLastError())
	}

	/// [`DeleteDC`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-deletedc)
	/// method.
	fn DeleteDC(self) -> WinResult<()> {
		bool_to_winresult(unsafe { gdi::ffi::DeleteDC(self.as_ptr()) })
	}

	/// [`EndPath`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-endpath)
	/// method.
	fn EndPath(self) -> WinResult<()> {
		bool_to_winresult(unsafe { gdi::ffi::EndPath(self.as_ptr()) })
	}


	/// [`FillPath`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-fillpath)
	/// method.
	fn FillPath(self) -> WinResult<()> {
		bool_to_winresult(unsafe { gdi::ffi::FillPath(self.as_ptr()) })
	}

	/// [`FillRect`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-fillrect)
	/// method.
	fn FillRect(self, rc: RECT, hbr: HBRUSH) -> WinResult<()> {
		match unsafe {
			gdi::ffi::FillRect(self.as_ptr(), &rc as *const _ as _, hbr.0)
		} {
			0 => Err(GetLastError()),
			_ => Ok(()),
		}
	}

	/// [`GetBkMode`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-getbkmode)
	/// method.
	#[must_use]
	fn GetBkMode(self) -> WinResult<co::BKMODE> {
		match unsafe { gdi::ffi::GetBkMode(self.as_ptr()) } {
			0 => Err(GetLastError()),
			v => Ok(co::BKMODE(v)),
		}
	}

	/// [`GetDCBrushColor`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-getdcbrushcolor)
	/// method.
	#[must_use]
	fn GetDCBrushColor(self) -> WinResult<COLORREF> {
		match unsafe { gdi::ffi::GetDCBrushColor(self.as_ptr()) } {
			CLR_INVALID => Err(GetLastError()),
			color => Ok(COLORREF(color)),
		}
	}

	/// [`GetDCPenColor`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-getdcpencolor)
	/// method.
	#[must_use]
	fn GetDCPenColor(self) -> WinResult<COLORREF> {
		match unsafe { gdi::ffi::GetDCPenColor(self.as_ptr()) } {
			CLR_INVALID => Err(GetLastError()),
			color => Ok(COLORREF(color)),
		}
	}

	/// [`GetDeviceCaps`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-getdevicecaps)
	/// method.
	#[must_use]
	fn GetDeviceCaps(self, index: co::GDC) -> i32 {
		unsafe { gdi::ffi::GetDeviceCaps(self.as_ptr(), index.0) }
	}

	/// [`GetStretchBltMode`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-getstretchbltmode)
	/// method.
	#[must_use]
	fn GetStretchBltMode(self) -> WinResult<co::STRETCH_MODE> {
		match unsafe { gdi::ffi::GetStretchBltMode(self.as_ptr()) } {
			0 => Err(GetLastError()),
			sm => Ok(co::STRETCH_MODE(sm)),
		}
	}

	/// [`GetTextColor`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-gettextcolor)
	/// method.
	#[must_use]
	fn GetTextColor(self) -> WinResult<COLORREF> {
		match unsafe { gdi::ffi::GetTextColor(self.as_ptr()) } {
			CLR_INVALID => Err(GetLastError()),
			color => Ok(COLORREF(color)),
		}
	}

	/// [`GetTextExtentPoint32`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-gettextextentpoint32w)
	/// method.
	#[must_use]
	fn GetTextExtentPoint32(self, text: &str) -> WinResult<SIZE> {
		let mut sz = SIZE::default();
		bool_to_winresult(
			unsafe {
				gdi::ffi::GetTextExtentPoint32W(
					self.as_ptr(),
					WString::from_str(text).as_ptr(),
					text.chars().count() as _,
					&mut sz as *mut _ as _,
				)
			},
		).map(|_| sz)
	}

	/// [`GetTextFace`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-gettextfacew)
	/// method.
	#[must_use]
	fn GetTextFace(self) -> WinResult<String> {
		let mut buf = WString::new_alloc_buffer(LF_FACESIZE + 1);
		match unsafe {
			gdi::ffi::GetTextFaceW(self.as_ptr(), buf.len() as _, buf.as_mut_ptr())
		} {
			0 => Err(GetLastError()),
			v => Ok(v),
		}.map(|_| buf.to_string())
	}

	/// [`GetTextMetrics`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-gettextmetricsw)
	/// method.
	fn GetTextMetrics(self, tm: &mut TEXTMETRIC) -> WinResult<()> {
		bool_to_winresult(
			unsafe { gdi::ffi::GetTextMetricsW(self.as_ptr(), tm as *mut _ as _) },
		)
	}

	/// [`GetViewportExtEx`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-getviewportextex)
	/// method.
	#[must_use]
	fn GetViewportExtEx(self) -> WinResult<SIZE> {
		let mut sz = SIZE::default();
		bool_to_winresult(
			unsafe {
				gdi::ffi::GetViewportExtEx(self.as_ptr(), &mut sz as *mut _ as _)
			},
		).map(|_| sz)
	}

	/// [`GetViewportOrgEx`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-getviewportorgex)
	/// method.
	#[must_use]
	fn GetViewportOrgEx(self) -> WinResult<POINT> {
		let mut pt = POINT::default();
		bool_to_winresult(
			unsafe {
				gdi::ffi::GetViewportOrgEx(self.as_ptr(), &mut pt as *mut _ as _)
			},
		).map(|_| pt)
	}

	/// [`GetWindowExtEx`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-getwindowextex)
	/// method.
	#[must_use]
	fn GetWindowExtEx(self) -> WinResult<SIZE> {
		let mut sz = SIZE::default();
		bool_to_winresult(
			unsafe {
				gdi::ffi::GetWindowExtEx(self.as_ptr(), &mut sz as *mut _ as _)
			},
		).map(|_| sz)
	}

	/// [`GetWindowOrgEx`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-getwindoworgex)
	/// method.
	#[must_use]
	fn GetWindowOrgEx(self) -> WinResult<POINT> {
		let mut pt = POINT::default();
		bool_to_winresult(
			unsafe {
				gdi::ffi::GetWindowOrgEx(self.as_ptr(), &mut pt as *mut _ as _)
			},
		).map(|_| pt)
	}

	/// [`LineTo`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-lineto)
	/// method.
	fn LineTo(self, x: i32, y: i32) -> WinResult<()> {
		bool_to_winresult(unsafe { gdi::ffi::LineTo(self.as_ptr(), x, y) })
	}

	/// [`MoveToEx`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-movetoex)
	/// method.
	fn MoveToEx(self, x: i32, y: i32, pt: Option<&mut POINT>) -> WinResult<()> {
		bool_to_winresult(
			unsafe {
				gdi::ffi::MoveToEx(
					self.as_ptr(),
					x, y,
					pt.map_or(std::ptr::null_mut(), |lp| lp as *mut _ as _),
				)
			},
		)
	}

	/// [`PatBlt`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-patblt)
	/// method.
	fn PatBlt(self, top_left: POINT, sz: SIZE, rop: co::ROP) -> WinResult<()> {
		bool_to_winresult(
			unsafe {
				gdi::ffi::PatBlt(
					self.as_ptr(), top_left.x, top_left.y, sz.cx, sz.cy, rop.0,
				)
			},
		)
	}

	/// [`PathToRegion`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-pathtoregion)
	/// method.
	///
	/// **Note:** Must be paired with an
	/// [`HRGN::DeleteObject`](crate::prelude::HandleGdi::DeleteObject) call.
	#[must_use]
	fn PathToRegion(self) -> WinResult<HRGN> {
		unsafe { gdi::ffi::PathToRegion(self.as_ptr()).as_mut() }
			.map(|ptr| HRGN(ptr))
			.ok_or_else(|| GetLastError())
	}

	/// [`Pie`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-pie)
	/// method.
	fn Pie(self,
		bounds: RECT, radial_1: POINT, radial_2: POINT) -> WinResult<()>
	{
		bool_to_winresult(
			unsafe {
				gdi::ffi::Pie(
					self.as_ptr(),
					bounds.left, bounds.top, bounds.right, bounds.bottom,
					radial_1.x, radial_1.y,
					radial_2.y, radial_2.y,
				)
			},
		)
	}

	/// [`PolyBezier`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-polybezier)
	/// method.
	fn PolyBezier(self, pts: &[POINT]) -> WinResult<()> {
		bool_to_winresult(
			unsafe {
				gdi::ffi::PolyBezier(
					self.as_ptr(), pts.as_ptr() as _, pts.len() as _,
				)
			},
		)
	}

	/// [`PolyBezierTo`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-polybezierto)
	/// method.
	fn PolyBezierTo(self, pts: &[POINT]) -> WinResult<()> {
		bool_to_winresult(
			unsafe {
				gdi::ffi::PolyBezierTo(
					self.as_ptr(), pts.as_ptr() as _, pts.len() as _,
				)
			},
		)
	}

	/// [`Polyline`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-polyline)
	/// method.
	fn Polyline(self, pts: &[POINT]) -> WinResult<()> {
		bool_to_winresult(
			unsafe {
				gdi::ffi::Polyline(self.as_ptr(), pts.as_ptr() as _, pts.len() as _)
			},
		)
	}

	/// [`PolylineTo`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-polylineto)
	/// method.
	fn PolylineTo(self, pts: &[POINT]) -> WinResult<()> {
		bool_to_winresult(
			unsafe {
				gdi::ffi::PolylineTo(
					self.as_ptr(), pts.as_ptr() as _, pts.len() as _,
				)
			},
		)
	}

	/// [`PtVisible`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-ptvisible)
	/// method.
	#[must_use]
	fn PtVisible(self, x: i32, y: i32) -> WinResult<bool> {
		match unsafe { gdi::ffi::PtVisible(self.as_ptr(), x, y) } {
			-1 => Err(GetLastError()),
			0 => Ok(false),
			_ => Ok(true),
		}
	}

	/// [`RealizePalette`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-realizepalette)
	/// method.
	fn RealizePalette(self) -> WinResult<u32> {
		match unsafe { gdi::ffi::RealizePalette(self.as_ptr()) } {
			GDI_ERROR => Err(GetLastError()),
			num => Ok(num),
		}
	}

	/// [`Rectangle`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-rectangle)
	/// method.
	fn Rectangle(self, bounds: RECT) -> WinResult<()> {
		bool_to_winresult(
			unsafe {
				gdi::ffi::Rectangle(self.as_ptr(),
					bounds.left, bounds.top, bounds.right, bounds.bottom)
			},
		)
	}

	/// [`RestoreDC`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-restoredc)
	/// method.
	fn RestoreDC(self, saved_dc: i32) -> WinResult<()> {
		bool_to_winresult(unsafe { gdi::ffi::RestoreDC(self.as_ptr(), saved_dc) })
	}

	/// [`RoundRect`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-roundrect)
	/// method.
	fn RoundRect(self, bounds: RECT, sz: SIZE) -> WinResult<()> {
		bool_to_winresult(
			unsafe {
				gdi::ffi::RoundRect(
					self.as_ptr(),
					bounds.left, bounds.top, bounds.right, bounds.bottom,
					sz.cx, sz.cy,
				)
			},
		)
	}

	/// [`SaveDC`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-savedc)
	/// method.
	fn SaveDC(self) -> WinResult<i32> {
		match unsafe { gdi::ffi::SaveDC(self.as_ptr()) } {
			0 => Err(GetLastError()),
			v => Ok(v),
		}
	}

	/// [`SelectObject`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-selectobject)
	/// method for [`HBITMAP`](crate::HBITMAP).
	fn SelectObjectBitmap(self, hbmp: HBITMAP) -> WinResult<HBITMAP> {
		unsafe { gdi::ffi::SelectObject(self.as_ptr(), hbmp.0).as_mut() }
			.map(|ptr| HBITMAP(ptr))
			.ok_or_else(|| GetLastError())
	}

	/// [`SelectObject`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-selectobject)
	/// method for [`HBRUSH`](crate::HBRUSH).
	fn SelectObjectBrush(self, hbr: HBRUSH) -> WinResult<HBRUSH> {
		unsafe { gdi::ffi::SelectObject(self.as_ptr(), hbr.0).as_mut() }
			.map(|ptr| HBRUSH(ptr))
			.ok_or_else(|| GetLastError())
	}

	/// [`SelectObject`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-selectobject)
	/// method for [`HFONT`](crate::HFONT).
	fn SelectObjectFont(self, hfont: HFONT) -> WinResult<HFONT> {
		unsafe { gdi::ffi::SelectObject(self.as_ptr(), hfont.0).as_mut() }
			.map(|ptr| HFONT(ptr))
			.ok_or_else(|| GetLastError())
	}

	/// [`SelectObject`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-selectobject)
	/// method for [`HPEN`](crate::HPEN).
	fn SelectObjectPen(self, hpen: HPEN) -> WinResult<HPEN> {
		unsafe { gdi::ffi::SelectObject(self.as_ptr(), hpen.0).as_mut() }
			.map(|ptr| HPEN(ptr))
			.ok_or_else(|| GetLastError())
	}

	/// [`SelectObject`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-selectobject)
	/// method for [`HRGN`](crate::HRGN).
	fn SelectObjectRgn(self, hrgn: HRGN) -> WinResult<co::REGION> {
		unsafe { gdi::ffi::SelectObject(self.as_ptr(), hrgn.0).as_mut() }
			.map(|ptr| co::REGION(ptr as *mut _ as _))
			.ok_or_else(|| GetLastError())
	}

	/// [`SetArcDirection`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-setarcdirection)
	/// method.
	fn SetArcDirection(self, dir: co::AD) -> WinResult<co::AD> {
		match unsafe { gdi::ffi::SetArcDirection(self.as_ptr(), dir.0) } {
			0 => Err(GetLastError()),
			v => Ok(co::AD(v)),
		}
	}

	/// [`SetBkMode`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-setbkmode)
	/// method.
	fn SetBkMode(self, mode: co::BKMODE) -> WinResult<co::BKMODE> {
		match unsafe { gdi::ffi::SetBkMode(self.as_ptr(), mode.0) } {
			0 => Err(GetLastError()),
			v => Ok(co::BKMODE(v)),
		}
	}

	/// [`SetBrushOrgEx`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-setbrushorgex)
	/// method.
	fn SetBrushOrgEx(self, new_origin: POINT) -> WinResult<POINT> {
		let mut old_origin = POINT::default();
		bool_to_winresult(
			unsafe {
				gdi::ffi::SetBrushOrgEx(
					self.as_ptr(),
					new_origin.x, new_origin.y,
					&mut old_origin as *mut _ as _,
				)
			},
		).map(|_| old_origin)
	}

	/// [`SetDCBrushColor`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-setdcbrushcolor)
	/// method.
	fn SetDCBrushColor(self, color: COLORREF) -> WinResult<COLORREF> {
		match unsafe { gdi::ffi::SetDCBrushColor(self.as_ptr(), color.0) } {
			CLR_INVALID => Err(GetLastError()),
			old => Ok(COLORREF(old)),
		}
	}

	/// [`SetDCPenColor`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-setdcpencolor)
	/// method.
	fn SetDCPenColor(self, color: COLORREF) -> WinResult<COLORREF> {
		match unsafe { gdi::ffi::SetDCPenColor(self.as_ptr(), color.0) } {
			CLR_INVALID => Err(GetLastError()),
			old => Ok(COLORREF(old)),
		}
	}

	/// [`SetGraphicsMode`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-setgraphicsmode)
	/// method.
	fn SetGraphicsMode(self, mode: co::GM) -> WinResult<co::GM> {
		match unsafe { gdi::ffi::SetGraphicsMode(self.as_ptr(), mode.0) } {
			0 => Err(GetLastError()),
			v => Ok(co::GM(v))
		}
	}

	/// [`SetStretchBltMode`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-setstretchbltmode)
	/// method.
	fn SetStretchBltMode(self,
		mode: co::STRETCH_MODE) -> WinResult<co::STRETCH_MODE>
	{
		match co::ERROR(
			unsafe { gdi::ffi::SetStretchBltMode(self.as_ptr(), mode.0) } as _,
		) {
			co::ERROR::INVALID_PARAMETER => Err(co::ERROR::INVALID_PARAMETER),
			err_val => Ok(co::STRETCH_MODE(err_val.0 as _)),
		}
	}

	/// [`SetTextAlign`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-settextalign)
	/// method.
	fn SetTextAlign(self, align: co::TA) -> WinResult<co::TA> {
		match unsafe { gdi::ffi::SetTextAlign(self.as_ptr(), align.0) } {
			GDI_ERROR => Err(GetLastError()),
			ta => Ok(co::TA(ta)),
		}
	}

	/// [`SetTextColor`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-settextcolor)
	/// method.
	fn SetTextColor(self, color: COLORREF) -> WinResult<COLORREF> {
		match unsafe { gdi::ffi::SetTextColor(self.as_ptr(), color.0) } {
			CLR_INVALID => Err(GetLastError()),
			old => Ok(COLORREF(old)),
		}
	}

	/// [`SetTextJustification`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-settextjustification)
	/// method.
	fn SetTextJustification(self, extra: i32, count: i32) -> WinResult<()> {
		bool_to_winresult(
			unsafe { gdi::ffi::SetTextJustification(self.as_ptr(), extra, count) },
		)
	}

	/// [`SetViewportExtEx`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-setviewportextex)
	/// method.
	fn SetViewportExtEx(self, x: i32, y: i32) -> WinResult<SIZE> {
		let mut sz = SIZE::default();
		bool_to_winresult(
			unsafe {
				gdi::ffi::SetViewportExtEx(
					self.as_ptr(), x, y, &mut sz as *mut _ as _,
				)
			},
		).map(|_| sz)
	}

	/// [`SetViewportOrgEx`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-setviewportorgex)
	/// method.
	fn SetViewportOrgEx(self, x: i32, y: i32) -> WinResult<POINT> {
		let mut pt = POINT::default();
		bool_to_winresult(
			unsafe {
				gdi::ffi::SetViewportOrgEx(
					self.as_ptr(), x, y, &mut pt as *mut _ as _,
				)
			},
		).map(|_| pt)
	}

	/// [`SetWindowExtEx`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-setwindowextex)
	/// method.
	fn SetWindowExtEx(self, x: i32, y: i32) -> WinResult<SIZE> {
		let mut sz = SIZE::default();
		bool_to_winresult(
			unsafe {
				gdi::ffi::SetWindowExtEx(
					self.as_ptr(), x, y, &mut sz as *mut _ as _,
				)
			},
		).map(|_| sz)
	}

	/// [`SetWindowOrgEx`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-setwindoworgex)
	/// method.
	fn SetWindowOrgEx(self, x: i32, y: i32) -> WinResult<POINT> {
		let mut pt = POINT::default();
		bool_to_winresult(
			unsafe {
				gdi::ffi::SetWindowOrgEx(
					self.as_ptr(), x, y, &mut pt as *mut _ as _,
				)
			},
		).map(|_| pt)
	}

	/// [`StretchBlt`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-stretchblt)
	/// method.
	fn StretchBlt(self,
		pos_dest: POINT, sz_dest: SIZE,
		hdc_src: HDC,
		pt_src: POINT, sz_src: SIZE,
		rop: co::ROP) -> WinResult<()>
	{
		bool_to_winresult(
			unsafe {
				gdi::ffi::StretchBlt(
					self.as_ptr(),
					pos_dest.x, pos_dest.y,
					sz_dest.cx, sz_dest.cy,
					hdc_src.0,
					pt_src.x, pt_src.y,
					sz_src.cx, sz_src.cy,
					rop.0,
				)
			},
		)
	}

	/// [`StrokeAndFillPath`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-strokeandfillpath)
	/// method.
	fn StrokeAndFillPath(self) -> WinResult<()> {
		bool_to_winresult(unsafe { gdi::ffi::StrokeAndFillPath(self.as_ptr()) })
	}

	/// [`StrokePath`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-strokepath)
	/// method.
	fn StrokePath(self) -> WinResult<()> {
		bool_to_winresult(unsafe { gdi::ffi::StrokePath(self.as_ptr()) })
	}

	/// [`TextOut`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-textoutw)
	/// method.
	fn TextOut(self, x: i32, y: i32, text: &str) -> WinResult<()> {
		bool_to_winresult(
			unsafe {
				gdi::ffi::TextOutW(
					self.as_ptr(),
					x, y,
					WString::from_str(text).as_ptr(),
					text.len() as _,
				)
			},
		)
	}

	/// [`UpdateColors`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-updatecolors)
	/// method.
	fn UpdateColors(self) -> WinResult<()> {
		bool_to_winresult(unsafe { gdi::ffi::UpdateColors(self.as_ptr()) })
	}

	/// [`WidenPath`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-widenpath)
	/// method.
	fn WidenPath(self) -> WinResult<()>  {
		bool_to_winresult(unsafe { gdi::ffi::WidenPath(self.as_ptr()) })
	}
}
