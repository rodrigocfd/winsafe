#![allow(non_camel_case_types, non_snake_case)]

use crate::{co, gdi};
use crate::gdi::decl::{BITMAPINFO, HFONT, HPEN, TEXTMETRIC};
use crate::gdi::privs::{CLR_INVALID, GDI_ERROR, LF_FACESIZE};
use crate::kernel::decl::{GetLastError, SysResult, WString};
use crate::kernel::privs::{bool_to_sysresult, invalidate_handle};
use crate::prelude::Handle;
use crate::user::decl::{
	COLORREF, HBITMAP, HBRUSH, HDC, HRGN, POINT, RECT, SIZE,
};

impl gdi_Hdc for HDC {}

/// This trait is enabled with the `gdi` feature, and provides methods for
/// [`HDC`](crate::HDC).
///
/// Prefer importing this trait through the prelude:
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// ```
#[cfg_attr(docsrs, doc(cfg(feature = "gdi")))]
pub trait gdi_Hdc: Handle {
	/// [`AborthPath`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-abortpath)
	/// method.
	fn AbortPath(&self) -> SysResult<()> {
		bool_to_sysresult(unsafe { gdi::ffi::AbortPath(self.as_ptr()) })
	}

	/// [`AngleArc`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-anglearc)
	/// method.
	fn AngleArc(&self,
		center: POINT, radius: u32,
		start_angle: f32, sweep_angle: f32) -> SysResult<()>
	{
		bool_to_sysresult(
			unsafe {
				gdi::ffi::AngleArc(
					self.as_ptr(),
					center.x, center.y,
					radius, start_angle, sweep_angle,
				)
			},
		)
	}

	/// [`Arc`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-arc)
	/// method.
	fn Arc(&self,
		bound: RECT, radialStart: POINT, radialEnd: POINT) -> SysResult<()>
	{
		bool_to_sysresult(
			unsafe {
				gdi::ffi::Arc(
					self.as_ptr(),
					bound.left, bound.top,
					bound.right, bound.bottom,
					radialStart.x, radialStart.y,
					radialEnd.x, radialEnd.y,
				)
			},
		)
	}

	/// [`ArcTo`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-arcto)
	/// method.
	fn ArcTo(&self,
		bound: RECT, radialStart: POINT, radialEnd: POINT) -> SysResult<()>
	{
		bool_to_sysresult(
			unsafe {
				gdi::ffi::ArcTo(
					self.as_ptr(),
					bound.left, bound.top,
					bound.right, bound.bottom,
					radialStart.x, radialStart.y,
					radialEnd.x, radialEnd.y,
				)
			},
		)
	}

	/// [`BeginPath`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-beginpath)
	/// method.
	fn BeginPath(&self) -> SysResult<()> {
		bool_to_sysresult(unsafe { gdi::ffi::BeginPath(self.as_ptr()) })
	}

	/// [`BitBlt`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-bitblt)
	/// method.
	fn BitBlt(&self,
		dest_pos: POINT, sz: SIZE,
		hdc_src: &HDC, src_src: POINT, rop: co::ROP) -> SysResult<()>
	{
		bool_to_sysresult(
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

	/// [`CancelDC`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-canceldc)
	/// method.
	fn CancelDC(&self) -> SysResult<()> {
		bool_to_sysresult(unsafe { gdi::ffi::CancelDC(self.as_ptr()) })
	}

	/// [`Chord`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-chord)
	/// method.
	fn Chord(&self,
		bounds: RECT, start_radial: POINT, end_radial: POINT) -> SysResult<()>
	{
		bool_to_sysresult(
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

	/// [`CloseFigure`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-closefigure)
	/// method.
	fn CloseFigure(&self) -> SysResult<()> {
		bool_to_sysresult(unsafe { gdi::ffi::CloseFigure(self.as_ptr()) })
	}

	/// [`CreateCompatibleBitmap`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-createcompatiblebitmap)
	/// method.
	///
	/// **Note:** Must be paired with an
	/// [`HBITMAP::DeleteObject`](crate::prelude::gdi_Hgdiobj::DeleteObject)
	/// call.
	#[must_use]
	fn CreateCompatibleBitmap(&self, cx: i32, cy: i32) -> SysResult<HBITMAP> {
		unsafe {
			gdi::ffi::CreateCompatibleBitmap(self.as_ptr(), cx, cy).as_mut()
		}.map(|ptr| HBITMAP(ptr))
			.ok_or_else(|| GetLastError())
	}

	/// [`CreateCompatibleDC`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-createcompatibledc)
	/// method.
	///
	/// **Note:** Must be paired with an
	/// [`HDC::DeleteDC`](crate::prelude::gdi_Hdc::DeleteDC) call.
	#[must_use]
	fn CreateCompatibleDC(&self) -> SysResult<HDC> {
		unsafe { gdi::ffi::CreateCompatibleDC(self.as_ptr()).as_mut() }
			.map(|ptr| HDC(ptr))
			.ok_or_else(|| GetLastError())
	}

	/// [`DeleteDC`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-deletedc)
	/// method.
	///
	/// After calling this method, the handle will be invalidated and further
	/// operations will fail with
	/// [`ERROR::INVALID_HANDLE`](crate::co::ERROR::INVALID_HANDLE) error code.
	fn DeleteDC(&self) -> SysResult<()> {
		let ret = bool_to_sysresult(unsafe { gdi::ffi::DeleteDC(self.as_ptr()) });
		invalidate_handle(self);
		ret
	}

	/// [`Ellipse`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-ellipse)
	/// method.
	fn Ellipse(&self, bound: RECT) -> SysResult<()> {
		bool_to_sysresult(
			unsafe {
				gdi::ffi::Ellipse(
					self.as_ptr(),
					bound.left, bound.top,
					bound.right, bound.bottom,
				)
			},
		)
	}

	/// [`EndPath`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-endpath)
	/// method.
	fn EndPath(&self) -> SysResult<()> {
		bool_to_sysresult(unsafe { gdi::ffi::EndPath(self.as_ptr()) })
	}

	/// [`FillPath`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-fillpath)
	/// method.
	fn FillPath(&self) -> SysResult<()> {
		bool_to_sysresult(unsafe { gdi::ffi::FillPath(self.as_ptr()) })
	}

	/// [`FillRect`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-fillrect)
	/// method.
	fn FillRect(&self, rc: RECT, hbr: &HBRUSH) -> SysResult<()> {
		match unsafe {
			gdi::ffi::FillRect(self.as_ptr(), &rc as *const _ as _, hbr.0)
		} {
			0 => Err(GetLastError()),
			_ => Ok(()),
		}
	}

	/// [`FillRgn`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-fillrgn)
	/// method.
	fn FillRgn(&self, rgn: &HRGN, brush: &HBRUSH) -> SysResult<()> {
		bool_to_sysresult(
			unsafe {
				gdi::ffi::FillRgn(self.as_ptr(), rgn.as_ptr(), brush.as_ptr())
			},
		)
	}

	/// [`FlattenPath`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-flattenpath)
	/// method.
	fn FlattenPath(&self) -> SysResult<()> {
		bool_to_sysresult(unsafe { gdi::ffi::FlattenPath(self.as_ptr()) })
	}

	/// [`FrameRgn`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-framergn)
	/// method.
	fn FrameRgn(&self,
		rgn: &HRGN, brush: &HBRUSH, w: i32, h: i32) -> SysResult<()>
	{
		bool_to_sysresult(
			unsafe {
				gdi::ffi::FrameRgn(
					self.as_ptr(),
					rgn.as_ptr(),
					brush.as_ptr(),
					w, h,
				)
			},
		)
	}

	/// [`GetBkMode`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-getbkmode)
	/// method.
	#[must_use]
	fn GetBkMode(&self) -> SysResult<co::BKMODE> {
		match unsafe { gdi::ffi::GetBkMode(self.as_ptr()) } {
			0 => Err(GetLastError()),
			v => Ok(co::BKMODE(v)),
		}
	}

	/// [`GetDCBrushColor`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-getdcbrushcolor)
	/// method.
	#[must_use]
	fn GetDCBrushColor(&self) -> SysResult<COLORREF> {
		match unsafe { gdi::ffi::GetDCBrushColor(self.as_ptr()) } {
			CLR_INVALID => Err(GetLastError()),
			color => Ok(COLORREF(color)),
		}
	}

	/// [`GetDCPenColor`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-getdcpencolor)
	/// method.
	#[must_use]
	fn GetDCPenColor(&self) -> SysResult<COLORREF> {
		match unsafe { gdi::ffi::GetDCPenColor(self.as_ptr()) } {
			CLR_INVALID => Err(GetLastError()),
			color => Ok(COLORREF(color)),
		}
	}

	/// [`GetDIBits`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-getdibits)
	/// method.
	///
	/// **Note:** If `bmpDataBuf` is smaller than needed, you'll have a buffer
	/// overflow.
	///
	/// # Examples
	///
	/// Taking a screenshot and saving to file:
	///
	/// ```rust,no_run
	/// use winsafe::{self as w, prelude::*, co};
	///
	/// let cx_screen = w::GetSystemMetrics(co::SM::CXSCREEN);
	/// let cy_screen = w::GetSystemMetrics(co::SM::CYSCREEN);
	///
	/// let hdc_screen = w::HWND::DESKTOP.GetDC()?;
	/// let hbmp = hdc_screen.CreateCompatibleBitmap(cx_screen, cy_screen)?;
	/// let hdc_mem = hdc_screen.CreateCompatibleDC()?;
	/// let hbmp_old = hdc_mem.SelectObjectBitmap(&hbmp)?;
	///
	/// hdc_mem.BitBlt(w::POINT::new(0, 0), w::SIZE::new(cx_screen, cy_screen),
	///     &hdc_screen, w::POINT::new(0, 0), co::ROP::SRCCOPY)?;
	///
	/// let mut bmp_obj = w::BITMAP::default();
	/// hbmp.GetObject(&mut bmp_obj)?;
	///
	/// let mut bi = w::BITMAPINFO::default();
	/// bi.bmiHeader.biWidth = cx_screen;
	/// bi.bmiHeader.biHeight = cy_screen;
	/// bi.bmiHeader.biPlanes = 1;
	/// bi.bmiHeader.biBitCount = 32;
	/// bi.bmiHeader.biCompression = co::BI::RGB;
	///
	/// let bmp_size = (bmp_obj.bmWidth * (bi.bmiHeader.biBitCount as i32) + 31)
	///     / 32 * 4 * bmp_obj.bmHeight;
	/// let mut data_buf = vec![0u8; bmp_size as _];
	///
	/// unsafe {
	///     hdc_screen.GetDIBits(&hbmp, 0, cy_screen as _,
	///         Some(&mut data_buf), &mut bi, co::DIB::RGB_COLORS)?;
	/// }
	///
	/// let mut bfh = w::BITMAPFILEHEADER::default();
	/// bfh.bfOffBits = (std::mem::size_of::<w::BITMAPFILEHEADER>()
	///     + std::mem::size_of::<w::BITMAPINFOHEADER>()) as _;
	/// bfh.bfSize = bfh.bfOffBits + (bmp_size as u32);
	///
	/// let fo = w::File::open("C:\\Temp\\foo.bmp", w::FileAccess::OpenOrCreateRW)?;
	/// fo.write(bfh.serialize())?;
	/// fo.write(bi.bmiHeader.serialize())?;
	/// fo.write(&data_buf)?;
	///
	/// hdc_mem.SelectObjectBitmap(&hbmp_old)?;
	/// hdc_mem.DeleteDC()?;
	/// hbmp.DeleteObject()?;
	/// # Ok::<_, co::ERROR>(())
	/// ```
	unsafe fn GetDIBits(&self,
		hbm: &HBITMAP,
		firstScanLine: u32,
		numScanLines: u32,
		bmpDataBuf: Option<&mut [u8]>,
		bmi: &mut BITMAPINFO,
		usage: co::DIB) -> SysResult<i32>
	{
		let ret = gdi::ffi::GetDIBits(
			self.as_ptr(),
			hbm.as_ptr(),
			firstScanLine, numScanLines,
			bmpDataBuf.map_or(std::ptr::null_mut(), |buf| buf.as_mut_ptr() as _),
			bmi as *const _ as _,
			usage.0,
		);

		if co::ERROR(ret as _) == co::ERROR::INVALID_PARAMETER {
			Err(co::ERROR::INVALID_PARAMETER)
		} else if ret == 0 {
			Err(GetLastError())
		} else {
			Ok(ret)
		}
	}

	/// [`GetDeviceCaps`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-getdevicecaps)
	/// method.
	#[must_use]
	fn GetDeviceCaps(&self, index: co::GDC) -> i32 {
		unsafe { gdi::ffi::GetDeviceCaps(self.as_ptr(), index.0) }
	}

	/// [`GetStretchBltMode`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-getstretchbltmode)
	/// method.
	#[must_use]
	fn GetStretchBltMode(&self) -> SysResult<co::STRETCH_MODE> {
		match unsafe { gdi::ffi::GetStretchBltMode(self.as_ptr()) } {
			0 => Err(GetLastError()),
			sm => Ok(co::STRETCH_MODE(sm)),
		}
	}

	/// [`GetTextColor`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-gettextcolor)
	/// method.
	#[must_use]
	fn GetTextColor(&self) -> SysResult<COLORREF> {
		match unsafe { gdi::ffi::GetTextColor(self.as_ptr()) } {
			CLR_INVALID => Err(GetLastError()),
			color => Ok(COLORREF(color)),
		}
	}

	/// [`GetTextExtentPoint32`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-gettextextentpoint32w)
	/// method.
	#[must_use]
	fn GetTextExtentPoint32(&self, text: &str) -> SysResult<SIZE> {
		let mut sz = SIZE::default();
		bool_to_sysresult(
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

	/// [`GetTextFace`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-gettextfacew)
	/// method.
	#[must_use]
	fn GetTextFace(&self) -> SysResult<String> {
		let mut buf = WString::new_alloc_buf(LF_FACESIZE + 1);
		match unsafe {
			gdi::ffi::GetTextFaceW(
				self.as_ptr(),
				buf.buf_len() as _,
				buf.as_mut_ptr(),
			)
		} {
			0 => Err(GetLastError()),
			v => Ok(v),
		}.map(|_| buf.to_string())
	}

	/// [`GetTextMetrics`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-gettextmetricsw)
	/// method.
	fn GetTextMetrics(&self, tm: &mut TEXTMETRIC) -> SysResult<()> {
		bool_to_sysresult(
			unsafe { gdi::ffi::GetTextMetricsW(self.as_ptr(), tm as *mut _ as _) },
		)
	}

	/// [`GetViewportExtEx`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-getviewportextex)
	/// method.
	#[must_use]
	fn GetViewportExtEx(&self) -> SysResult<SIZE> {
		let mut sz = SIZE::default();
		bool_to_sysresult(
			unsafe {
				gdi::ffi::GetViewportExtEx(self.as_ptr(), &mut sz as *mut _ as _)
			},
		).map(|_| sz)
	}

	/// [`GetViewportOrgEx`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-getviewportorgex)
	/// method.
	#[must_use]
	fn GetViewportOrgEx(&self) -> SysResult<POINT> {
		let mut pt = POINT::default();
		bool_to_sysresult(
			unsafe {
				gdi::ffi::GetViewportOrgEx(self.as_ptr(), &mut pt as *mut _ as _)
			},
		).map(|_| pt)
	}

	/// [`GetWindowExtEx`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-getwindowextex)
	/// method.
	#[must_use]
	fn GetWindowExtEx(&self) -> SysResult<SIZE> {
		let mut sz = SIZE::default();
		bool_to_sysresult(
			unsafe {
				gdi::ffi::GetWindowExtEx(self.as_ptr(), &mut sz as *mut _ as _)
			},
		).map(|_| sz)
	}

	/// [`GetWindowOrgEx`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-getwindoworgex)
	/// method.
	#[must_use]
	fn GetWindowOrgEx(&self) -> SysResult<POINT> {
		let mut pt = POINT::default();
		bool_to_sysresult(
			unsafe {
				gdi::ffi::GetWindowOrgEx(self.as_ptr(), &mut pt as *mut _ as _)
			},
		).map(|_| pt)
	}

	/// [`LineTo`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-lineto)
	/// method.
	fn LineTo(&self, x: i32, y: i32) -> SysResult<()> {
		bool_to_sysresult(unsafe { gdi::ffi::LineTo(self.as_ptr(), x, y) })
	}

	/// [`MoveToEx`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-movetoex)
	/// method.
	fn MoveToEx(&self, x: i32, y: i32, pt: Option<&mut POINT>) -> SysResult<()> {
		bool_to_sysresult(
			unsafe {
				gdi::ffi::MoveToEx(
					self.as_ptr(),
					x, y,
					pt.map_or(std::ptr::null_mut(), |lp| lp as *mut _ as _),
				)
			},
		)
	}

	/// [`PatBlt`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-patblt)
	/// method.
	fn PatBlt(&self, top_left: POINT, sz: SIZE, rop: co::ROP) -> SysResult<()> {
		bool_to_sysresult(
			unsafe {
				gdi::ffi::PatBlt(
					self.as_ptr(), top_left.x, top_left.y, sz.cx, sz.cy, rop.0,
				)
			},
		)
	}

	/// [`PathToRegion`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-pathtoregion)
	/// method.
	///
	/// **Note:** Must be paired with an
	/// [`HRGN::DeleteObject`](crate::prelude::gdi_Hgdiobj::DeleteObject) call.
	#[must_use]
	fn PathToRegion(&self) -> SysResult<HRGN> {
		unsafe { gdi::ffi::PathToRegion(self.as_ptr()).as_mut() }
			.map(|ptr| HRGN(ptr))
			.ok_or_else(|| GetLastError())
	}

	/// [`Pie`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-pie)
	/// method.
	fn Pie(&self,
		bounds: RECT, radial_1: POINT, radial_2: POINT) -> SysResult<()>
	{
		bool_to_sysresult(
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

	/// [`PolyBezier`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-polybezier)
	/// method.
	fn PolyBezier(&self, pts: &[POINT]) -> SysResult<()> {
		bool_to_sysresult(
			unsafe {
				gdi::ffi::PolyBezier(
					self.as_ptr(), pts.as_ptr() as _, pts.len() as _,
				)
			},
		)
	}

	/// [`PolyBezierTo`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-polybezierto)
	/// method.
	fn PolyBezierTo(&self, pts: &[POINT]) -> SysResult<()> {
		bool_to_sysresult(
			unsafe {
				gdi::ffi::PolyBezierTo(
					self.as_ptr(), pts.as_ptr() as _, pts.len() as _,
				)
			},
		)
	}

	/// [`Polyline`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-polyline)
	/// method.
	fn Polyline(&self, pts: &[POINT]) -> SysResult<()> {
		bool_to_sysresult(
			unsafe {
				gdi::ffi::Polyline(self.as_ptr(), pts.as_ptr() as _, pts.len() as _)
			},
		)
	}

	/// [`PolylineTo`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-polylineto)
	/// method.
	fn PolylineTo(&self, pts: &[POINT]) -> SysResult<()> {
		bool_to_sysresult(
			unsafe {
				gdi::ffi::PolylineTo(
					self.as_ptr(), pts.as_ptr() as _, pts.len() as _,
				)
			},
		)
	}

	/// [`PtVisible`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-ptvisible)
	/// method.
	#[must_use]
	fn PtVisible(&self, x: i32, y: i32) -> SysResult<bool> {
		match unsafe { gdi::ffi::PtVisible(self.as_ptr(), x, y) } {
			-1 => Err(GetLastError()),
			0 => Ok(false),
			_ => Ok(true),
		}
	}

	/// [`RealizePalette`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-realizepalette)
	/// method.
	fn RealizePalette(&self) -> SysResult<u32> {
		match unsafe { gdi::ffi::RealizePalette(self.as_ptr()) } {
			GDI_ERROR => Err(GetLastError()),
			num => Ok(num),
		}
	}

	/// [`Rectangle`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-rectangle)
	/// method.
	fn Rectangle(&self, bounds: RECT) -> SysResult<()> {
		bool_to_sysresult(
			unsafe {
				gdi::ffi::Rectangle(self.as_ptr(),
					bounds.left, bounds.top, bounds.right, bounds.bottom)
			},
		)
	}

	/// [`RestoreDC`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-restoredc)
	/// method.
	fn RestoreDC(&self, saved_dc: i32) -> SysResult<()> {
		bool_to_sysresult(unsafe { gdi::ffi::RestoreDC(self.as_ptr(), saved_dc) })
	}

	/// [`RoundRect`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-roundrect)
	/// method.
	fn RoundRect(&self, bounds: RECT, sz: SIZE) -> SysResult<()> {
		bool_to_sysresult(
			unsafe {
				gdi::ffi::RoundRect(
					self.as_ptr(),
					bounds.left, bounds.top, bounds.right, bounds.bottom,
					sz.cx, sz.cy,
				)
			},
		)
	}

	/// [`SaveDC`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-savedc)
	/// method.
	fn SaveDC(&self) -> SysResult<i32> {
		match unsafe { gdi::ffi::SaveDC(self.as_ptr()) } {
			0 => Err(GetLastError()),
			v => Ok(v),
		}
	}

	/// [`SelectObject`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-selectobject)
	/// method for [`HBITMAP`](crate::HBITMAP).
	fn SelectObjectBitmap(&self, hbmp: &HBITMAP) -> SysResult<HBITMAP> {
		unsafe { gdi::ffi::SelectObject(self.as_ptr(), hbmp.0).as_mut() }
			.map(|ptr| HBITMAP(ptr))
			.ok_or_else(|| GetLastError())
	}

	/// [`SelectObject`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-selectobject)
	/// method for [`HBRUSH`](crate::HBRUSH).
	fn SelectObjectBrush(&self, hbr: &HBRUSH) -> SysResult<HBRUSH> {
		unsafe { gdi::ffi::SelectObject(self.as_ptr(), hbr.0).as_mut() }
			.map(|ptr| HBRUSH(ptr))
			.ok_or_else(|| GetLastError())
	}

	/// [`SelectObject`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-selectobject)
	/// method for [`HFONT`](crate::HFONT).
	fn SelectObjectFont(&self, hfont: &HFONT) -> SysResult<HFONT> {
		unsafe { gdi::ffi::SelectObject(self.as_ptr(), hfont.0).as_mut() }
			.map(|ptr| HFONT(ptr))
			.ok_or_else(|| GetLastError())
	}

	/// [`SelectObject`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-selectobject)
	/// method for [`HPEN`](crate::HPEN).
	fn SelectObjectPen(&self, hpen: &HPEN) -> SysResult<HPEN> {
		unsafe { gdi::ffi::SelectObject(self.as_ptr(), hpen.0).as_mut() }
			.map(|ptr| HPEN(ptr))
			.ok_or_else(|| GetLastError())
	}

	/// [`SelectObject`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-selectobject)
	/// method for [`HRGN`](crate::HRGN).
	fn SelectObjectRgn(&self, hrgn: &HRGN) -> SysResult<co::REGION> {
		unsafe { gdi::ffi::SelectObject(self.as_ptr(), hrgn.0).as_mut() }
			.map(|ptr| co::REGION(ptr as *mut _ as _))
			.ok_or_else(|| GetLastError())
	}

	/// [`SetArcDirection`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-setarcdirection)
	/// method.
	fn SetArcDirection(&self, dir: co::AD) -> SysResult<co::AD> {
		match unsafe { gdi::ffi::SetArcDirection(self.as_ptr(), dir.0) } {
			0 => Err(GetLastError()),
			v => Ok(co::AD(v)),
		}
	}

	/// [`SetBkColor`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-setbkcolor)
	/// method.
	fn SetBkColor(&self, color: COLORREF) -> SysResult<COLORREF> {
		match unsafe { gdi::ffi::SetBkColor(self.as_ptr(), color.0) } {
			CLR_INVALID => Err(GetLastError()),
			old => Ok(COLORREF(old)),
		}
	}

	/// [`SetBkMode`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-setbkmode)
	/// method.
	fn SetBkMode(&self, mode: co::BKMODE) -> SysResult<co::BKMODE> {
		match unsafe { gdi::ffi::SetBkMode(self.as_ptr(), mode.0) } {
			0 => Err(GetLastError()),
			v => Ok(co::BKMODE(v)),
		}
	}

	/// [`SetBrushOrgEx`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-setbrushorgex)
	/// method.
	fn SetBrushOrgEx(&self, new_origin: POINT) -> SysResult<POINT> {
		let mut old_origin = POINT::default();
		bool_to_sysresult(
			unsafe {
				gdi::ffi::SetBrushOrgEx(
					self.as_ptr(),
					new_origin.x, new_origin.y,
					&mut old_origin as *mut _ as _,
				)
			},
		).map(|_| old_origin)
	}

	/// [`SetDCBrushColor`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-setdcbrushcolor)
	/// method.
	fn SetDCBrushColor(&self, color: COLORREF) -> SysResult<COLORREF> {
		match unsafe { gdi::ffi::SetDCBrushColor(self.as_ptr(), color.0) } {
			CLR_INVALID => Err(GetLastError()),
			old => Ok(COLORREF(old)),
		}
	}

	/// [`SetDCPenColor`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-setdcpencolor)
	/// method.
	fn SetDCPenColor(&self, color: COLORREF) -> SysResult<COLORREF> {
		match unsafe { gdi::ffi::SetDCPenColor(self.as_ptr(), color.0) } {
			CLR_INVALID => Err(GetLastError()),
			old => Ok(COLORREF(old)),
		}
	}

	/// [`SetGraphicsMode`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-setgraphicsmode)
	/// method.
	fn SetGraphicsMode(&self, mode: co::GM) -> SysResult<co::GM> {
		match unsafe { gdi::ffi::SetGraphicsMode(self.as_ptr(), mode.0) } {
			0 => Err(GetLastError()),
			v => Ok(co::GM(v))
		}
	}

	/// [`SetStretchBltMode`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-setstretchbltmode)
	/// method.
	fn SetStretchBltMode(&self,
		mode: co::STRETCH_MODE) -> SysResult<co::STRETCH_MODE>
	{
		match co::ERROR(
			unsafe { gdi::ffi::SetStretchBltMode(self.as_ptr(), mode.0) } as _,
		) {
			co::ERROR::INVALID_PARAMETER => Err(co::ERROR::INVALID_PARAMETER),
			err_val => Ok(co::STRETCH_MODE(err_val.0 as _)),
		}
	}

	/// [`SetTextAlign`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-settextalign)
	/// method.
	fn SetTextAlign(&self, align: co::TA) -> SysResult<co::TA> {
		match unsafe { gdi::ffi::SetTextAlign(self.as_ptr(), align.0) } {
			GDI_ERROR => Err(GetLastError()),
			ta => Ok(co::TA(ta)),
		}
	}

	/// [`SetTextColor`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-settextcolor)
	/// method.
	fn SetTextColor(&self, color: COLORREF) -> SysResult<COLORREF> {
		match unsafe { gdi::ffi::SetTextColor(self.as_ptr(), color.0) } {
			CLR_INVALID => Err(GetLastError()),
			old => Ok(COLORREF(old)),
		}
	}

	/// [`SetTextJustification`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-settextjustification)
	/// method.
	fn SetTextJustification(&self, extra: i32, count: i32) -> SysResult<()> {
		bool_to_sysresult(
			unsafe { gdi::ffi::SetTextJustification(self.as_ptr(), extra, count) },
		)
	}

	/// [`SetViewportExtEx`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-setviewportextex)
	/// method.
	fn SetViewportExtEx(&self, x: i32, y: i32) -> SysResult<SIZE> {
		let mut sz = SIZE::default();
		bool_to_sysresult(
			unsafe {
				gdi::ffi::SetViewportExtEx(
					self.as_ptr(), x, y, &mut sz as *mut _ as _,
				)
			},
		).map(|_| sz)
	}

	/// [`SetViewportOrgEx`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-setviewportorgex)
	/// method.
	fn SetViewportOrgEx(&self, x: i32, y: i32) -> SysResult<POINT> {
		let mut pt = POINT::default();
		bool_to_sysresult(
			unsafe {
				gdi::ffi::SetViewportOrgEx(
					self.as_ptr(), x, y, &mut pt as *mut _ as _,
				)
			},
		).map(|_| pt)
	}

	/// [`SetWindowExtEx`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-setwindowextex)
	/// method.
	fn SetWindowExtEx(&self, x: i32, y: i32) -> SysResult<SIZE> {
		let mut sz = SIZE::default();
		bool_to_sysresult(
			unsafe {
				gdi::ffi::SetWindowExtEx(
					self.as_ptr(), x, y, &mut sz as *mut _ as _,
				)
			},
		).map(|_| sz)
	}

	/// [`SetWindowOrgEx`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-setwindoworgex)
	/// method.
	fn SetWindowOrgEx(&self, x: i32, y: i32) -> SysResult<POINT> {
		let mut pt = POINT::default();
		bool_to_sysresult(
			unsafe {
				gdi::ffi::SetWindowOrgEx(
					self.as_ptr(), x, y, &mut pt as *mut _ as _,
				)
			},
		).map(|_| pt)
	}

	/// [`StretchBlt`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-stretchblt)
	/// method.
	fn StretchBlt(&self,
		pos_dest: POINT, sz_dest: SIZE,
		hdc_src: &HDC,
		pt_src: POINT, sz_src: SIZE,
		rop: co::ROP) -> SysResult<()>
	{
		bool_to_sysresult(
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

	/// [`StrokeAndFillPath`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-strokeandfillpath)
	/// method.
	fn StrokeAndFillPath(&self) -> SysResult<()> {
		bool_to_sysresult(unsafe { gdi::ffi::StrokeAndFillPath(self.as_ptr()) })
	}

	/// [`StrokePath`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-strokepath)
	/// method.
	fn StrokePath(&self) -> SysResult<()> {
		bool_to_sysresult(unsafe { gdi::ffi::StrokePath(self.as_ptr()) })
	}

	/// [`TextOut`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-textoutw)
	/// method.
	fn TextOut(&self, x: i32, y: i32, text: &str) -> SysResult<()> {
		bool_to_sysresult(
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

	/// [`UpdateColors`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-updatecolors)
	/// method.
	fn UpdateColors(&self) -> SysResult<()> {
		bool_to_sysresult(unsafe { gdi::ffi::UpdateColors(self.as_ptr()) })
	}

	/// [`WidenPath`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-widenpath)
	/// method.
	fn WidenPath(&self) -> SysResult<()>  {
		bool_to_sysresult(unsafe { gdi::ffi::WidenPath(self.as_ptr()) })
	}
}
