#![allow(non_camel_case_types, non_snake_case)]

use std::any::TypeId;

use crate::{co, gdi};
use crate::gdi::decl::{BITMAPINFO, HPALETTE, TEXTMETRIC};
use crate::gdi::guard::{DeleteDCGuard, DeleteObjectGuard, SelectObjectGuard};
use crate::gdi::privs::{CLR_INVALID, GDI_ERROR, LF_FACESIZE};
use crate::kernel::decl::{GetLastError, SysResult, WString};
use crate::kernel::privs::{
	bool_to_sysresult, ptr_to_sysresult, ptr_to_sysresult_handle,
};
use crate::prelude::{GdiObjectSelect, Handle};
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
pub trait gdi_Hdc: Handle {
	/// [`AborthPath`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-abortpath)
	/// method.
	fn AbortPath(&self) -> SysResult<()> {
		bool_to_sysresult(unsafe { gdi::ffi::AbortPath(self.ptr()) })
	}

	/// [`AngleArc`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-anglearc)
	/// method.
	fn AngleArc(&self,
		center: POINT,
		radius: u32,
		start_angle: f32,
		sweep_angle: f32,
	) -> SysResult<()>
	{
		bool_to_sysresult(
			unsafe {
				gdi::ffi::AngleArc(
					self.ptr(),
					center.x, center.y,
					radius, start_angle, sweep_angle,
				)
			},
		)
	}

	/// [`Arc`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-arc)
	/// method.
	fn Arc(&self,
		bound: RECT, radial_start: POINT, radial_end: POINT) -> SysResult<()>
	{
		bool_to_sysresult(
			unsafe {
				gdi::ffi::Arc(
					self.ptr(),
					bound.left, bound.top,
					bound.right, bound.bottom,
					radial_start.x, radial_start.y,
					radial_end.x, radial_end.y,
				)
			},
		)
	}

	/// [`ArcTo`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-arcto)
	/// method.
	fn ArcTo(&self,
		bound: RECT, radial_start: POINT, radial_end: POINT) -> SysResult<()>
	{
		bool_to_sysresult(
			unsafe {
				gdi::ffi::ArcTo(
					self.ptr(),
					bound.left, bound.top,
					bound.right, bound.bottom,
					radial_start.x, radial_start.y,
					radial_end.x, radial_end.y,
				)
			},
		)
	}

	/// [`BeginPath`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-beginpath)
	/// method.
	fn BeginPath(&self) -> SysResult<()> {
		bool_to_sysresult(unsafe { gdi::ffi::BeginPath(self.ptr()) })
	}

	/// [`BitBlt`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-bitblt)
	/// method.
	fn BitBlt(&self,
		dest_pos: POINT,
		sz: SIZE,
		hdc_src: &HDC,
		src_src: POINT,
		rop: co::ROP,
	) -> SysResult<()>
	{
		bool_to_sysresult(
			unsafe {
				gdi::ffi::BitBlt(
					self.ptr(),
					dest_pos.x, dest_pos.y,
					sz.cx, sz.cy,
					hdc_src.ptr(),
					src_src.x, src_src.y,
					rop.raw(),
				)
			},
		)
	}

	/// [`CancelDC`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-canceldc)
	/// method.
	fn CancelDC(&self) -> SysResult<()> {
		bool_to_sysresult(unsafe { gdi::ffi::CancelDC(self.ptr()) })
	}

	/// [`Chord`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-chord)
	/// method.
	fn Chord(&self,
		bounds: RECT, start_radial: POINT, end_radial: POINT) -> SysResult<()>
	{
		bool_to_sysresult(
			unsafe {
				gdi::ffi::Chord(
					self.ptr(),
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
		bool_to_sysresult(unsafe { gdi::ffi::CloseFigure(self.ptr()) })
	}

	/// [`CreateCompatibleBitmap`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-createcompatiblebitmap)
	/// method.
	#[must_use]
	fn CreateCompatibleBitmap(&self,
		cx: i32, cy: i32) -> SysResult<DeleteObjectGuard<HBITMAP>>
	{
		unsafe {
			ptr_to_sysresult_handle(
				gdi::ffi::CreateCompatibleBitmap(self.ptr(), cx, cy),
			).map(|h| DeleteObjectGuard::new(h))
		}
	}

	/// [`CreateCompatibleDC`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-createcompatibledc)
	/// method.
	#[must_use]
	fn CreateCompatibleDC(&self) -> SysResult<DeleteDCGuard> {
		unsafe {
			ptr_to_sysresult_handle(gdi::ffi::CreateCompatibleDC(self.ptr()))
				.map(|h| DeleteDCGuard::new(h))
		}
	}

	/// [`CreateHalftonePalette`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-createhalftonepalette)
	/// method.
	#[must_use]
	fn CreateHalftonePalette(&self) -> SysResult<DeleteObjectGuard<HPALETTE>> {
		unsafe {
			ptr_to_sysresult_handle(gdi::ffi::CreateHalftonePalette(self.ptr()))
				.map(|h| DeleteObjectGuard::new(h))
		}
	}

	/// [`Ellipse`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-ellipse)
	/// method.
	fn Ellipse(&self, bound: RECT) -> SysResult<()> {
		bool_to_sysresult(
			unsafe {
				gdi::ffi::Ellipse(
					self.ptr(),
					bound.left, bound.top,
					bound.right, bound.bottom,
				)
			},
		)
	}

	/// [`EndPath`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-endpath)
	/// method.
	fn EndPath(&self) -> SysResult<()> {
		bool_to_sysresult(unsafe { gdi::ffi::EndPath(self.ptr()) })
	}

	/// [`FillPath`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-fillpath)
	/// method.
	fn FillPath(&self) -> SysResult<()> {
		bool_to_sysresult(unsafe { gdi::ffi::FillPath(self.ptr()) })
	}

	/// [`FillRect`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-fillrect)
	/// method.
	fn FillRect(&self, rc: RECT, hbr: &HBRUSH) -> SysResult<()> {
		bool_to_sysresult(
			unsafe {
				gdi::ffi::FillRect(
					self.ptr(),
					&rc as *const _ as _,
					hbr.ptr(),
				)
			},
		)
	}

	/// [`FillRgn`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-fillrgn)
	/// method.
	fn FillRgn(&self, rgn: &HRGN, brush: &HBRUSH) -> SysResult<()> {
		bool_to_sysresult(
			unsafe {
				gdi::ffi::FillRgn(self.ptr(), rgn.ptr(), brush.ptr())
			},
		)
	}

	/// [`FlattenPath`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-flattenpath)
	/// method.
	fn FlattenPath(&self) -> SysResult<()> {
		bool_to_sysresult(unsafe { gdi::ffi::FlattenPath(self.ptr()) })
	}

	/// [`FrameRgn`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-framergn)
	/// method.
	fn FrameRgn(&self,
		rgn: &HRGN, brush: &HBRUSH, w: i32, h: i32) -> SysResult<()>
	{
		bool_to_sysresult(
			unsafe {
				gdi::ffi::FrameRgn(
					self.ptr(),
					rgn.ptr(),
					brush.ptr(),
					w, h,
				)
			},
		)
	}

	/// [`GetBkColor`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-getbkcolor)
	/// method.
	#[must_use]
	fn GetBkColor(&self) -> SysResult<COLORREF> {
		match unsafe { gdi::ffi::GetBkColor(self.ptr()) } {
			CLR_INVALID => Err(GetLastError()),
			c => Ok(unsafe { COLORREF::from_raw(c) }),
		}
	}

	/// [`GetBkMode`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-getbkmode)
	/// method.
	#[must_use]
	fn GetBkMode(&self) -> SysResult<co::BKMODE> {
		match unsafe { gdi::ffi::GetBkMode(self.ptr()) } {
			0 => Err(GetLastError()),
			v => Ok(unsafe { co::BKMODE::from_raw(v) }),
		}
	}

	/// [`GetDCBrushColor`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-getdcbrushcolor)
	/// method.
	#[must_use]
	fn GetDCBrushColor(&self) -> SysResult<COLORREF> {
		match unsafe { gdi::ffi::GetDCBrushColor(self.ptr()) } {
			CLR_INVALID => Err(GetLastError()),
			color => Ok(unsafe { COLORREF::from_raw(color) }),
		}
	}

	/// [`GetDCPenColor`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-getdcpencolor)
	/// method.
	#[must_use]
	fn GetDCPenColor(&self) -> SysResult<COLORREF> {
		match unsafe { gdi::ffi::GetDCPenColor(self.ptr()) } {
			CLR_INVALID => Err(GetLastError()),
			color => Ok(unsafe { COLORREF::from_raw(color) }),
		}
	}

	/// [`GetDIBits`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-getdibits)
	/// method.
	///
	/// # Safety
	///
	/// If `bmpDataBuf` is smaller than needed, you'll have a buffer overflow.
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
	/// let _hbmp_guard = hdc_mem.SelectObject(&*hbmp)?;
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
	/// # Ok::<_, co::ERROR>(())
	/// ```
	unsafe fn GetDIBits(&self,
		hbm: &HBITMAP,
		first_scan_line: u32,
		num_scan_lines: u32,
		bmp_data_buf: Option<&mut [u8]>,
		bmi: &mut BITMAPINFO,
		usage: co::DIB,
	) -> SysResult<i32>
	{
		let ret = gdi::ffi::GetDIBits(
			self.ptr(),
			hbm.ptr(),
			first_scan_line,
			num_scan_lines,
			bmp_data_buf.map_or(std::ptr::null_mut(), |buf| buf.as_mut_ptr() as _),
			bmi as *const _ as _,
			usage.raw(),
		);

		if unsafe { co::ERROR::from_raw(ret as _) } == co::ERROR::INVALID_PARAMETER {
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
		unsafe { gdi::ffi::GetDeviceCaps(self.ptr(), index.raw()) }
	}

	/// [`GetStretchBltMode`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-getstretchbltmode)
	/// method.
	#[must_use]
	fn GetStretchBltMode(&self) -> SysResult<co::STRETCH_MODE> {
		match unsafe { gdi::ffi::GetStretchBltMode(self.ptr()) } {
			0 => Err(GetLastError()),
			sm => Ok(unsafe { co::STRETCH_MODE::from_raw(sm) }),
		}
	}

	/// [`GetTextColor`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-gettextcolor)
	/// method.
	#[must_use]
	fn GetTextColor(&self) -> SysResult<COLORREF> {
		match unsafe { gdi::ffi::GetTextColor(self.ptr()) } {
			CLR_INVALID => Err(GetLastError()),
			color => Ok(unsafe { COLORREF::from_raw(color) }),
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
					self.ptr(),
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
				self.ptr(),
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
			unsafe { gdi::ffi::GetTextMetricsW(self.ptr(), tm as *mut _ as _) },
		)
	}

	/// [`GetViewportExtEx`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-getviewportextex)
	/// method.
	#[must_use]
	fn GetViewportExtEx(&self) -> SysResult<SIZE> {
		let mut sz = SIZE::default();
		bool_to_sysresult(
			unsafe {
				gdi::ffi::GetViewportExtEx(self.ptr(), &mut sz as *mut _ as _)
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
				gdi::ffi::GetViewportOrgEx(self.ptr(), &mut pt as *mut _ as _)
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
				gdi::ffi::GetWindowExtEx(self.ptr(), &mut sz as *mut _ as _)
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
				gdi::ffi::GetWindowOrgEx(self.ptr(), &mut pt as *mut _ as _)
			},
		).map(|_| pt)
	}

	/// [`LineTo`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-lineto)
	/// method.
	fn LineTo(&self, x: i32, y: i32) -> SysResult<()> {
		bool_to_sysresult(unsafe { gdi::ffi::LineTo(self.ptr(), x, y) })
	}

	/// [`MoveToEx`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-movetoex)
	/// method.
	fn MoveToEx(&self, x: i32, y: i32, pt: Option<&mut POINT>) -> SysResult<()> {
		bool_to_sysresult(
			unsafe {
				gdi::ffi::MoveToEx(
					self.ptr(),
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
					self.ptr(), top_left.x, top_left.y, sz.cx, sz.cy, rop.raw(),
				)
			},
		)
	}

	/// [`PathToRegion`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-pathtoregion)
	/// method.
	#[must_use]
	fn PathToRegion(&self) -> SysResult<DeleteObjectGuard<HRGN>> {
		unsafe {
			ptr_to_sysresult_handle(gdi::ffi::PathToRegion(self.ptr()))
				.map(|h| DeleteObjectGuard::new(h))
		}
	}

	/// [`Pie`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-pie)
	/// method.
	fn Pie(&self,
		bounds: RECT, radial_1: POINT, radial_2: POINT) -> SysResult<()>
	{
		bool_to_sysresult(
			unsafe {
				gdi::ffi::Pie(
					self.ptr(),
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
					self.ptr(), pts.as_ptr() as _, pts.len() as _,
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
					self.ptr(), pts.as_ptr() as _, pts.len() as _,
				)
			},
		)
	}

	/// [`Polyline`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-polyline)
	/// method.
	fn Polyline(&self, pts: &[POINT]) -> SysResult<()> {
		bool_to_sysresult(
			unsafe {
				gdi::ffi::Polyline(self.ptr(), pts.as_ptr() as _, pts.len() as _)
			},
		)
	}

	/// [`PolylineTo`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-polylineto)
	/// method.
	fn PolylineTo(&self, pts: &[POINT]) -> SysResult<()> {
		bool_to_sysresult(
			unsafe {
				gdi::ffi::PolylineTo(
					self.ptr(), pts.as_ptr() as _, pts.len() as _,
				)
			},
		)
	}

	/// [`PtVisible`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-ptvisible)
	/// method.
	#[must_use]
	fn PtVisible(&self, x: i32, y: i32) -> SysResult<bool> {
		match unsafe { gdi::ffi::PtVisible(self.ptr(), x, y) } {
			-1 => Err(GetLastError()),
			0 => Ok(false),
			_ => Ok(true),
		}
	}

	/// [`RealizePalette`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-realizepalette)
	/// method.
	fn RealizePalette(&self) -> SysResult<u32> {
		match unsafe { gdi::ffi::RealizePalette(self.ptr()) } {
			GDI_ERROR => Err(GetLastError()),
			num => Ok(num),
		}
	}

	/// [`Rectangle`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-rectangle)
	/// method.
	fn Rectangle(&self, bounds: RECT) -> SysResult<()> {
		bool_to_sysresult(
			unsafe {
				gdi::ffi::Rectangle(self.ptr(),
					bounds.left, bounds.top, bounds.right, bounds.bottom)
			},
		)
	}

	/// [`RestoreDC`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-restoredc)
	/// method.
	fn RestoreDC(&self, saved_dc: i32) -> SysResult<()> {
		bool_to_sysresult(unsafe { gdi::ffi::RestoreDC(self.ptr(), saved_dc) })
	}

	/// [`RoundRect`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-roundrect)
	/// method.
	fn RoundRect(&self, bounds: RECT, sz: SIZE) -> SysResult<()> {
		bool_to_sysresult(
			unsafe {
				gdi::ffi::RoundRect(
					self.ptr(),
					bounds.left, bounds.top, bounds.right, bounds.bottom,
					sz.cx, sz.cy,
				)
			},
		)
	}

	/// [`SaveDC`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-savedc)
	/// method.
	fn SaveDC(&self) -> SysResult<i32> {
		match unsafe { gdi::ffi::SaveDC(self.ptr()) } {
			0 => Err(GetLastError()),
			v => Ok(v),
		}
	}

	/// [`SelectClipPath`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-selectclippath)
	/// method.
	fn SelectClipPath(&self, mode: co::RGN) -> SysResult<()> {
		bool_to_sysresult(
			unsafe { gdi::ffi::SelectClipPath(self.ptr(), mode.raw()) },
		)
	}

	/// [`SelectClipRgn`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-selectcliprgn)
	/// method.
	fn SelectClipRgn(&self, rgn: &HRGN) -> SysResult<co::REGION> {
		match unsafe { gdi::ffi::SelectClipRgn(self.ptr(), rgn.ptr()) } {
			0 => Err(GetLastError()),
			v => Ok(unsafe { co::REGION::from_raw(v) }),
		}
	}

	/// [`SelectObject`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-selectobject)
	/// method.
	///
	/// In the original C implementation, `SelectObject` returns a handle to the
	/// object being replaced. You must perform a cleanup operation, calling
	/// `SelectObject` again, passing the handle to the replaced object.
	///
	/// Here, the cleanup is performed automatically, because `SelectObject`
	/// returns a [`SelectObjectGuard`](crate::guard::SelectObjectGuard), which
	/// stores the replaced handle and calls `SelectObject` automatically when
	/// the guard goes out of scope. You must, however, keep the guard alive,
	/// otherwise the cleanup will be performed right away.
	///
	/// # Examples
	///
	/// ```rust,no_run
	/// use winsafe::prelude::*;
	/// use winsafe::{co, COLORREF, HDC, HPEN};
	///
	/// let hdc: HDC; // initialized somewhere
	/// # let hdc = HDC::NULL;
	///
	/// let hpen = HPEN::CreatePen(
	///     co::PS::SOLID,
	///     1,
	///     COLORREF::new(0xff, 0x00, 0x88),
	/// )?;
	///
	/// let _pen_guard = hdc.SelectObject(&*hpen); // keep guard alive
	/// # Ok::<_, co::ERROR>(())
	/// ```
	#[must_use]
	fn SelectObject<G>(&self,
		hgdiobj: &G,
	) -> SysResult<SelectObjectGuard<'_, Self, G>>
		where G: GdiObjectSelect,
	{
		unsafe {
			ptr_to_sysresult(
				gdi::ffi::SelectObject(self.ptr(), hgdiobj.ptr()),
			).map(|ptr| {
				if hgdiobj.type_id() == TypeId::of::<HRGN>() {
					SelectObjectGuard::new(
						self,
						G::NULL, // regions don't need cleanup
						Some(co::REGION::from_raw(ptr as *mut _ as _)),
					)
				} else {
					SelectObjectGuard::new(
						self,
						G::from_ptr(ptr), // GDI object to cleanup
						None,
					)
				}
			})
		}
	}

	/// [`SelectPalette`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-selectpalette)
	/// method.
	fn SelectPalette(&self,
		hpal: &HPALETTE, force_bkgd: bool) -> SysResult<Option<HPALETTE>>
	{
		let ptr = unsafe {
			gdi::ffi::SelectPalette(
				self.ptr(),
				hpal.ptr(),
				force_bkgd as _,
			)
		};

		if ptr.is_null() {
			match GetLastError() {
				co::ERROR::SUCCESS => Ok(None),
				err => Err(err),
			}
		} else {
			Ok(Some(unsafe { HPALETTE::from_ptr(ptr) }))
		}
	}

	/// [`SetArcDirection`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-setarcdirection)
	/// method.
	fn SetArcDirection(&self, dir: co::AD) -> SysResult<co::AD> {
		match unsafe { gdi::ffi::SetArcDirection(self.ptr(), dir.raw()) } {
			0 => Err(GetLastError()),
			v => Ok(unsafe { co::AD::from_raw(v) }),
		}
	}

	/// [`SetBkColor`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-setbkcolor)
	/// method.
	fn SetBkColor(&self, color: COLORREF) -> SysResult<COLORREF> {
		match unsafe { gdi::ffi::SetBkColor(self.ptr(), color.into()) } {
			CLR_INVALID => Err(GetLastError()),
			old => Ok(unsafe { COLORREF::from_raw(old) }),
		}
	}

	/// [`SetBkMode`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-setbkmode)
	/// method.
	fn SetBkMode(&self, mode: co::BKMODE) -> SysResult<co::BKMODE> {
		match unsafe { gdi::ffi::SetBkMode(self.ptr(), mode.raw()) } {
			0 => Err(GetLastError()),
			v => Ok(unsafe { co::BKMODE::from_raw(v) }),
		}
	}

	/// [`SetBrushOrgEx`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-setbrushorgex)
	/// method.
	fn SetBrushOrgEx(&self, new_origin: POINT) -> SysResult<POINT> {
		let mut old_origin = POINT::default();
		bool_to_sysresult(
			unsafe {
				gdi::ffi::SetBrushOrgEx(
					self.ptr(),
					new_origin.x, new_origin.y,
					&mut old_origin as *mut _ as _,
				)
			},
		).map(|_| old_origin)
	}

	/// [`SetDCBrushColor`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-setdcbrushcolor)
	/// method.
	fn SetDCBrushColor(&self, color: COLORREF) -> SysResult<COLORREF> {
		match unsafe { gdi::ffi::SetDCBrushColor(self.ptr(), color.into()) } {
			CLR_INVALID => Err(GetLastError()),
			old => Ok(unsafe { COLORREF::from_raw(old) }),
		}
	}

	/// [`SetDCPenColor`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-setdcpencolor)
	/// method.
	fn SetDCPenColor(&self, color: COLORREF) -> SysResult<COLORREF> {
		match unsafe { gdi::ffi::SetDCPenColor(self.ptr(), color.into()) } {
			CLR_INVALID => Err(GetLastError()),
			old => Ok(unsafe { COLORREF::from_raw(old) }),
		}
	}

	/// [`SetDIBits`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-setdibits)
	/// method.
	fn SetDIBits(&self,
		hbm: &HBITMAP,
		first_scan_line: u32,
		num_scan_lines: u32,
		dib_color_data: &[u8],
		bmi: &BITMAPINFO,
		color_use: co::DIB,
	) -> SysResult<i32>
	{
		match unsafe {
			gdi::ffi::SetDIBits(
				self.ptr(),
				hbm.ptr(),
				first_scan_line,
				num_scan_lines,
				dib_color_data.as_ptr() as _,
				bmi as *const _ as _,
				color_use.raw(),
			)
		} {
			0 => match GetLastError() {
				co::ERROR::SUCCESS => Ok(0),
				err => Err(err),
			},
			n => Ok(n),
		}
	}

	/// [`SetGraphicsMode`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-setgraphicsmode)
	/// method.
	fn SetGraphicsMode(&self, mode: co::GM) -> SysResult<co::GM> {
		match unsafe { gdi::ffi::SetGraphicsMode(self.ptr(), mode.raw()) } {
			0 => Err(GetLastError()),
			v => Ok(unsafe { co::GM::from_raw(v) })
		}
	}

	/// [`SetStretchBltMode`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-setstretchbltmode)
	/// method.
	fn SetStretchBltMode(&self,
		mode: co::STRETCH_MODE) -> SysResult<co::STRETCH_MODE>
	{
		match unsafe {
			co::ERROR::from_raw(
				gdi::ffi::SetStretchBltMode(self.ptr(), mode.raw()) as _,
			)
		} {
			co::ERROR::INVALID_PARAMETER => Err(co::ERROR::INVALID_PARAMETER),
			err_val => Ok(unsafe { co::STRETCH_MODE::from_raw(err_val.raw() as _) }),
		}
	}

	/// [`SetTextAlign`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-settextalign)
	/// method.
	fn SetTextAlign(&self, align: co::TA) -> SysResult<co::TA> {
		match unsafe { gdi::ffi::SetTextAlign(self.ptr(), align.raw()) } {
			GDI_ERROR => Err(GetLastError()),
			ta => Ok(unsafe { co::TA::from_raw(ta) }),
		}
	}

	/// [`SetTextColor`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-settextcolor)
	/// method.
	fn SetTextColor(&self, color: COLORREF) -> SysResult<COLORREF> {
		match unsafe { gdi::ffi::SetTextColor(self.ptr(), color.into()) } {
			CLR_INVALID => Err(GetLastError()),
			old => Ok(unsafe { COLORREF::from_raw(old) }),
		}
	}

	/// [`SetTextJustification`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-settextjustification)
	/// method.
	fn SetTextJustification(&self, extra: i32, count: i32) -> SysResult<()> {
		bool_to_sysresult(
			unsafe { gdi::ffi::SetTextJustification(self.ptr(), extra, count) },
		)
	}

	/// [`SetViewportExtEx`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-setviewportextex)
	/// method.
	fn SetViewportExtEx(&self, x: i32, y: i32) -> SysResult<SIZE> {
		let mut sz = SIZE::default();
		bool_to_sysresult(
			unsafe {
				gdi::ffi::SetViewportExtEx(
					self.ptr(), x, y, &mut sz as *mut _ as _,
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
					self.ptr(), x, y, &mut pt as *mut _ as _,
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
					self.ptr(), x, y, &mut sz as *mut _ as _,
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
					self.ptr(), x, y, &mut pt as *mut _ as _,
				)
			},
		).map(|_| pt)
	}

	/// [`StretchBlt`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-stretchblt)
	/// method.
	fn StretchBlt(&self,
		pos_dest: POINT,
		sz_dest: SIZE,
		hdc_src: &HDC,
		pt_src: POINT,
		sz_src: SIZE,
		rop: co::ROP,
	) -> SysResult<()>
	{
		bool_to_sysresult(
			unsafe {
				gdi::ffi::StretchBlt(
					self.ptr(),
					pos_dest.x, pos_dest.y,
					sz_dest.cx, sz_dest.cy,
					hdc_src.ptr(),
					pt_src.x, pt_src.y,
					sz_src.cx, sz_src.cy,
					rop.raw(),
				)
			},
		)
	}

	/// [`StrokeAndFillPath`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-strokeandfillpath)
	/// method.
	fn StrokeAndFillPath(&self) -> SysResult<()> {
		bool_to_sysresult(unsafe { gdi::ffi::StrokeAndFillPath(self.ptr()) })
	}

	/// [`StrokePath`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-strokepath)
	/// method.
	fn StrokePath(&self) -> SysResult<()> {
		bool_to_sysresult(unsafe { gdi::ffi::StrokePath(self.ptr()) })
	}

	/// [`TextOut`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-textoutw)
	/// method.
	fn TextOut(&self, x: i32, y: i32, text: &str) -> SysResult<()> {
		bool_to_sysresult(
			unsafe {
				gdi::ffi::TextOutW(
					self.ptr(),
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
		bool_to_sysresult(unsafe { gdi::ffi::UpdateColors(self.ptr()) })
	}

	/// [`WidenPath`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-widenpath)
	/// method.
	fn WidenPath(&self) -> SysResult<()>  {
		bool_to_sysresult(unsafe { gdi::ffi::WidenPath(self.ptr()) })
	}
}
