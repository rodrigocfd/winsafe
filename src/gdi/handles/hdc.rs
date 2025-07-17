#![allow(non_camel_case_types, non_snake_case)]

use std::any::TypeId;

use crate::co;
use crate::decl::*;
use crate::gdi::{ffi, privs::*};
use crate::guard::*;
use crate::kernel::privs::*;
use crate::prelude::*;

impl HDC {
	/// [`AbortDoc`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-abortdoc)
	/// function.
	pub fn AbortDoc(&self) -> SysResult<()> {
		BoolRet(unsafe { ffi::AbortDoc(self.ptr()) }).to_invalidparm()
	}

	/// [`AborthPath`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-abortpath)
	/// function.
	pub fn AbortPath(&self) -> SysResult<()> {
		BoolRet(unsafe { ffi::AbortPath(self.ptr()) }).to_invalidparm()
	}

	/// [`AlphaBlend`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-alphablend)
	/// function.
	pub fn AlphaBlend(
		&self,
		origin_dest: RECT,
		hdc_src: &HDC,
		origin_src: RECT,
		ftn: &BLENDFUNCTION,
	) -> SysResult<()> {
		BoolRet(unsafe {
			ffi::AlphaBlend(
				self.ptr(),
				origin_dest.left,
				origin_dest.top,
				origin_dest.right,
				origin_dest.bottom,
				hdc_src.ptr(),
				origin_src.left,
				origin_src.top,
				origin_src.right,
				origin_src.bottom,
				pcvoid(ftn),
			)
		})
		.to_invalidparm()
	}

	/// [`AngleArc`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-anglearc)
	/// function.
	pub fn AngleArc(
		&self,
		center: POINT,
		radius: u32,
		start_angle: f32,
		sweep_angle: f32,
	) -> SysResult<()> {
		BoolRet(unsafe {
			ffi::AngleArc(self.ptr(), center.x, center.y, radius, start_angle, sweep_angle)
		})
		.to_invalidparm()
	}

	/// [`Arc`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-arc)
	/// function.
	pub fn Arc(&self, bound: RECT, radial_start: POINT, radial_end: POINT) -> SysResult<()> {
		BoolRet(unsafe {
			ffi::Arc(
				self.ptr(),
				bound.left,
				bound.top,
				bound.right,
				bound.bottom,
				radial_start.x,
				radial_start.y,
				radial_end.x,
				radial_end.y,
			)
		})
		.to_invalidparm()
	}

	/// [`ArcTo`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-arcto)
	/// function.
	pub fn ArcTo(&self, bound: RECT, radial_start: POINT, radial_end: POINT) -> SysResult<()> {
		BoolRet(unsafe {
			ffi::ArcTo(
				self.ptr(),
				bound.left,
				bound.top,
				bound.right,
				bound.bottom,
				radial_start.x,
				radial_start.y,
				radial_end.x,
				radial_end.y,
			)
		})
		.to_invalidparm()
	}

	/// [`BeginPath`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-beginpath)
	/// function.
	pub fn BeginPath(&self) -> SysResult<()> {
		BoolRet(unsafe { ffi::BeginPath(self.ptr()) }).to_invalidparm()
	}

	/// [`BitBlt`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-bitblt)
	/// function.
	pub fn BitBlt(
		&self,
		dest_pos: POINT,
		sz: SIZE,
		hdc_src: &HDC,
		src_src: POINT,
		rop: co::ROP,
	) -> SysResult<()> {
		BoolRet(unsafe {
			ffi::BitBlt(
				self.ptr(),
				dest_pos.x,
				dest_pos.y,
				sz.cx,
				sz.cy,
				hdc_src.ptr(),
				src_src.x,
				src_src.y,
				rop.raw(),
			)
		})
		.to_sysresult()
	}

	/// [`CancelDC`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-canceldc)
	/// function.
	pub fn CancelDC(&self) -> SysResult<()> {
		BoolRet(unsafe { ffi::CancelDC(self.ptr()) }).to_invalidparm()
	}

	/// [`ChoosePixelFormat`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-choosepixelformat)
	/// function.
	pub fn ChoosePixelFormat(&self, pfd: &PIXELFORMATDESCRIPTOR) -> SysResult<i32> {
		match unsafe { ffi::ChoosePixelFormat(self.ptr(), pcvoid(pfd)) } {
			0 => Err(GetLastError()),
			n => Ok(n),
		}
	}

	/// [`Chord`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-chord)
	/// function.
	pub fn Chord(&self, bounds: RECT, start_radial: POINT, end_radial: POINT) -> SysResult<()> {
		BoolRet(unsafe {
			ffi::Chord(
				self.ptr(),
				bounds.left,
				bounds.top,
				bounds.right,
				bounds.bottom,
				start_radial.x,
				start_radial.y,
				end_radial.x,
				end_radial.y,
			)
		})
		.to_invalidparm()
	}

	/// [`CloseFigure`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-closefigure)
	/// function.
	pub fn CloseFigure(&self) -> SysResult<()> {
		BoolRet(unsafe { ffi::CloseFigure(self.ptr()) }).to_invalidparm()
	}

	/// [`CreateCompatibleBitmap`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-createcompatiblebitmap)
	/// function.
	#[must_use]
	pub fn CreateCompatibleBitmap(
		&self,
		cx: i32,
		cy: i32,
	) -> SysResult<DeleteObjectGuard<HBITMAP>> {
		unsafe {
			PtrRet(ffi::CreateCompatibleBitmap(self.ptr(), cx, cy))
				.to_invalidparm_handle()
				.map(|h| DeleteObjectGuard::new(h))
		}
	}

	/// [`CreateCompatibleDC`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-createcompatibledc)
	/// function.
	#[must_use]
	pub fn CreateCompatibleDC(&self) -> SysResult<DeleteDCGuard> {
		unsafe {
			PtrRet(ffi::CreateCompatibleDC(self.ptr()))
				.to_invalidparm_handle()
				.map(|h| DeleteDCGuard::new(h))
		}
	}

	/// [`CreateHalftonePalette`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-createhalftonepalette)
	/// function.
	#[must_use]
	pub fn CreateHalftonePalette(&self) -> SysResult<DeleteObjectPaletteGuard> {
		unsafe {
			PtrRet(ffi::CreateHalftonePalette(self.ptr()))
				.to_invalidparm_handle()
				.map(|h| DeleteObjectPaletteGuard::new(h))
		}
	}

	/// [`DescribePixelFormat`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-describepixelformat)
	/// function.
	#[must_use]
	pub fn DescribePixelFormat(&self, index: i32) -> SysResult<PIXELFORMATDESCRIPTOR> {
		let mut pfd = PIXELFORMATDESCRIPTOR::default();
		BoolRet(unsafe {
			ffi::DescribePixelFormat(
				self.ptr(),
				index,
				std::mem::size_of::<PIXELFORMATDESCRIPTOR>() as _,
				pvoid(&mut pfd),
			)
		})
		.to_invalidparm()
		.map(|_| pfd)
	}

	/// [`Ellipse`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-ellipse)
	/// function.
	pub fn Ellipse(&self, bound: RECT) -> SysResult<()> {
		BoolRet(unsafe {
			ffi::Ellipse(self.ptr(), bound.left, bound.top, bound.right, bound.bottom)
		})
		.to_invalidparm()
	}

	/// [`EndPath`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-endpath)
	/// function.
	pub fn EndPath(&self) -> SysResult<()> {
		BoolRet(unsafe { ffi::EndPath(self.ptr()) }).to_invalidparm()
	}

	/// [`ExcludeClipRect`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-excludecliprect)
	/// function.
	pub fn ExcludeClipRect(&self, rc: RECT) -> SysResult<co::REGION> {
		match unsafe { ffi::ExcludeClipRect(self.ptr(), rc.left, rc.top, rc.right, rc.bottom) } {
			0 => Err(co::ERROR::INVALID_PARAMETER),
			v => Ok(unsafe { co::REGION::from_raw(v) }),
		}
	}

	/// [`FillPath`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-fillpath)
	/// function.
	pub fn FillPath(&self) -> SysResult<()> {
		BoolRet(unsafe { ffi::FillPath(self.ptr()) }).to_invalidparm()
	}

	/// [`FillRect`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-fillrect)
	/// function.
	pub fn FillRect(&self, rc: RECT, hbr: &HBRUSH) -> SysResult<()> {
		BoolRet(unsafe { ffi::FillRect(self.ptr(), pcvoid(&rc), hbr.ptr()) }).to_invalidparm()
	}

	/// [`FillRgn`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-fillrgn)
	/// function.
	pub fn FillRgn(&self, rgn: &HRGN, brush: &HBRUSH) -> SysResult<()> {
		BoolRet(unsafe { ffi::FillRgn(self.ptr(), rgn.ptr(), brush.ptr()) }).to_invalidparm()
	}

	/// [`FlattenPath`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-flattenpath)
	/// function.
	pub fn FlattenPath(&self) -> SysResult<()> {
		BoolRet(unsafe { ffi::FlattenPath(self.ptr()) }).to_invalidparm()
	}

	/// [`FrameRgn`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-framergn)
	/// function.
	pub fn FrameRgn(&self, rgn: &HRGN, brush: &HBRUSH, w: i32, h: i32) -> SysResult<()> {
		BoolRet(unsafe { ffi::FrameRgn(self.ptr(), rgn.ptr(), brush.ptr(), w, h) }).to_invalidparm()
	}

	/// [`GetBkColor`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-getbkcolor)
	/// function.
	#[must_use]
	pub fn GetBkColor(&self) -> SysResult<COLORREF> {
		match unsafe { ffi::GetBkColor(self.ptr()) } {
			CLR_INVALID => Err(co::ERROR::INVALID_PARAMETER),
			c => Ok(unsafe { COLORREF::from_raw(c) }),
		}
	}

	/// [`GetBkMode`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-getbkmode)
	/// function.
	#[must_use]
	pub fn GetBkMode(&self) -> SysResult<co::BKMODE> {
		match unsafe { ffi::GetBkMode(self.ptr()) } {
			0 => Err(co::ERROR::INVALID_PARAMETER),
			v => Ok(unsafe { co::BKMODE::from_raw(v) }),
		}
	}

	/// [`GetCurrentObject`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-getcurrentobject)
	/// function.
	///
	/// # Examples
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*, co};
	///
	/// let hdc: w::HDC; // initialized somewhere
	/// # let hdc = w::HDC::NULL;
	///
	/// let obj = hdc.GetCurrentObject(co::CUR_OBJ::BRUSH)?;
	/// let w::CurObj::Brush(hbrush) = obj else { unreachable!() };
	///
	/// println!("HBRUSH: {}", hbrush);
	/// w::SysResult::Ok(())
	/// ```
	#[must_use]
	pub fn GetCurrentObject(&self, kind: co::CUR_OBJ) -> SysResult<CurObj> {
		unsafe {
			PtrRet(ffi::GetCurrentObject(self.ptr(), kind.raw()))
				.to_invalidparm()
				.map(|h| match kind {
					co::CUR_OBJ::BITMAP => CurObj::Bitmap(HBITMAP::from_ptr(h)),
					co::CUR_OBJ::BRUSH => CurObj::Brush(HBRUSH::from_ptr(h)),
					co::CUR_OBJ::FONT => CurObj::Font(HFONT::from_ptr(h)),
					co::CUR_OBJ::PAL => CurObj::Pal(HPALETTE::from_ptr(h)),
					co::CUR_OBJ::PEN => CurObj::Pen(HPEN::from_ptr(h)),
					_ => panic!("co::OBJ_CUR not implemented: {}", kind),
				})
		}
	}

	/// [`GetCurrentPositionEx`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-getcurrentpositionex)
	/// function.
	#[must_use]
	pub fn GetCurrentPositionEx(&self) -> SysResult<POINT> {
		let mut pt = POINT::default();
		BoolRet(unsafe { ffi::GetCurrentPositionEx(self.ptr(), pvoid(&mut pt)) })
			.to_invalidparm()
			.map(|_| pt)
	}

	/// [`GetDCBrushColor`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-getdcbrushcolor)
	/// function.
	#[must_use]
	pub fn GetDCBrushColor(&self) -> SysResult<COLORREF> {
		match unsafe { ffi::GetDCBrushColor(self.ptr()) } {
			CLR_INVALID => Err(co::ERROR::INVALID_PARAMETER),
			color => Ok(unsafe { COLORREF::from_raw(color) }),
		}
	}

	/// [`GetDCPenColor`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-getdcpencolor)
	/// function.
	#[must_use]
	pub fn GetDCPenColor(&self) -> SysResult<COLORREF> {
		match unsafe { ffi::GetDCPenColor(self.ptr()) } {
			CLR_INVALID => Err(co::ERROR::INVALID_PARAMETER),
			color => Ok(unsafe { COLORREF::from_raw(color) }),
		}
	}

	/// [`GetDeviceCaps`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-getdevicecaps)
	/// function.
	#[must_use]
	pub fn GetDeviceCaps(&self, index: co::GDC) -> i32 {
		unsafe { ffi::GetDeviceCaps(self.ptr(), index.raw()) }
	}

	/// [`GetDIBits`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-getdibits)
	/// function.
	///
	/// # Safety
	///
	/// If `bmpDataBuf` is smaller than needed, you'll have a buffer overflow.
	///
	/// # Examples
	///
	/// Taking a screenshot and saving to file:
	///
	/// ```no_run
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
	/// hdc_mem.BitBlt(
	///     w::POINT::new(),
	///     w::SIZE::with(cx_screen, cy_screen),
	///     &hdc_screen,
	///     w::POINT::new(),
	///     co::ROP::SRCCOPY,
	/// )?;
	///
	/// let bmp_obj = hbmp.GetObject()?;
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
	///     hdc_screen.GetDIBits(
	///         &hbmp,
	///         0,
	///         cy_screen as _,
	///         Some(&mut data_buf),
	///         &mut bi,
	///         co::DIB::RGB_COLORS,
	///     )?;
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
	/// # w::SysResult::Ok(())
	/// ```
	pub unsafe fn GetDIBits(
		&self,
		hbm: &HBITMAP,
		first_scan_line: u32,
		num_scan_lines: u32,
		bmp_data_buf: Option<&mut [u8]>,
		bmi: &mut BITMAPINFO,
		usage: co::DIB,
	) -> SysResult<i32> {
		let ret = unsafe {
			ffi::GetDIBits(
				self.ptr(),
				hbm.ptr(),
				first_scan_line,
				num_scan_lines,
				bmp_data_buf.map_or(std::ptr::null_mut(), |buf| buf.as_mut_ptr() as _),
				pvoid(bmi),
				usage.raw(),
			)
		};

		if ret == 0 { Err(co::ERROR::INVALID_PARAMETER) } else { Ok(ret) }
	}

	/// [`GetPixelFormat`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-getpixelformat)
	/// function.
	#[must_use]
	pub fn GetPixelFormat(&self) -> SysResult<i32> {
		match unsafe { ffi::GetPixelFormat(self.ptr()) } {
			0 => Err(GetLastError()),
			n => Ok(n),
		}
	}

	/// [`GetStretchBltMode`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-getstretchbltmode)
	/// function.
	#[must_use]
	pub fn GetStretchBltMode(&self) -> SysResult<co::STRETCH_MODE> {
		match unsafe { ffi::GetStretchBltMode(self.ptr()) } {
			0 => Err(co::ERROR::INVALID_PARAMETER),
			sm => Ok(unsafe { co::STRETCH_MODE::from_raw(sm) }),
		}
	}

	/// [`GetTextColor`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-gettextcolor)
	/// function.
	#[must_use]
	pub fn GetTextColor(&self) -> SysResult<COLORREF> {
		match unsafe { ffi::GetTextColor(self.ptr()) } {
			CLR_INVALID => Err(co::ERROR::INVALID_PARAMETER),
			color => Ok(unsafe { COLORREF::from_raw(color) }),
		}
	}

	/// [`GetTextExtentPoint32`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-gettextextentpoint32w)
	/// function.
	#[must_use]
	pub fn GetTextExtentPoint32(&self, text: &str) -> SysResult<SIZE> {
		let mut sz = SIZE::default();
		let wtext = WString::from_str(text);

		BoolRet(unsafe {
			ffi::GetTextExtentPoint32W(
				self.ptr(),
				wtext.as_ptr(),
				wtext.str_len() as _,
				pvoid(&mut sz),
			)
		})
		.to_invalidparm()
		.map(|_| sz)
	}

	/// [`GetTextFace`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-gettextfacew)
	/// function.
	#[must_use]
	pub fn GetTextFace(&self) -> SysResult<String> {
		let mut buf = WString::new_alloc_buf(LF_FACESIZE + 1);
		match unsafe { ffi::GetTextFaceW(self.ptr(), buf.buf_len() as _, buf.as_mut_ptr()) } {
			0 => Err(co::ERROR::INVALID_PARAMETER),
			v => Ok(v),
		}
		.map(|_| buf.to_string())
	}

	/// [`GetTextMetrics`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-gettextmetricsw)
	/// function.
	pub fn GetTextMetrics(&self) -> SysResult<TEXTMETRIC> {
		let mut tm = TEXTMETRIC::default();
		BoolRet(unsafe { ffi::GetTextMetricsW(self.ptr(), pvoid(&mut tm)) })
			.to_invalidparm()
			.map(|_| tm)
	}

	/// [`GetViewportExtEx`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-getviewportextex)
	/// function.
	#[must_use]
	pub fn GetViewportExtEx(&self) -> SysResult<SIZE> {
		let mut sz = SIZE::default();
		BoolRet(unsafe { ffi::GetViewportExtEx(self.ptr(), pvoid(&mut sz)) })
			.to_invalidparm()
			.map(|_| sz)
	}

	/// [`GetViewportOrgEx`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-getviewportorgex)
	/// function.
	#[must_use]
	pub fn GetViewportOrgEx(&self) -> SysResult<POINT> {
		let mut pt = POINT::default();
		BoolRet(unsafe { ffi::GetViewportOrgEx(self.ptr(), pvoid(&mut pt)) })
			.to_invalidparm()
			.map(|_| pt)
	}

	/// [`GetWindowExtEx`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-getwindowextex)
	/// function.
	#[must_use]
	pub fn GetWindowExtEx(&self) -> SysResult<SIZE> {
		let mut sz = SIZE::default();
		BoolRet(unsafe { ffi::GetWindowExtEx(self.ptr(), pvoid(&mut sz)) })
			.to_invalidparm()
			.map(|_| sz)
	}

	/// [`GetWindowOrgEx`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-getwindoworgex)
	/// function.
	#[must_use]
	pub fn GetWindowOrgEx(&self) -> SysResult<POINT> {
		let mut pt = POINT::default();
		BoolRet(unsafe { ffi::GetWindowOrgEx(self.ptr(), pvoid(&mut pt)) })
			.to_invalidparm()
			.map(|_| pt)
	}

	/// [`AtlHiMetricToPixel`](https://learn.microsoft.com/en-us/cpp/atl/reference/pixel-himetric-conversion-global-functions?view=msvc-170#atlhimetrictopixel)
	/// function.
	///
	/// Converts HIMETRIC units to pixels. The inverse operation is
	/// [`HDC::PixelToHiMetric`](crate::HDC::PixelToHiMetric).
	#[must_use]
	pub fn HiMetricToPixel(&self, x: i32, y: i32) -> (i32, i32) {
		// http://www.verycomputer.com/5_5f2f75dc2d090ee8_1.htm
		// https://forums.codeguru.com/showthread.php?109554-Unresizable-activeX-control
		(
			MulDiv(x, self.GetDeviceCaps(co::GDC::LOGPIXELSX), HIMETRIC_PER_INCH),
			MulDiv(y, self.GetDeviceCaps(co::GDC::LOGPIXELSY), HIMETRIC_PER_INCH),
		)
	}

	/// [`IntersectClipRect`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-intersectcliprect)
	/// function.
	pub fn IntersectClipRect(&self, rc: RECT) -> SysResult<()> {
		BoolRet(unsafe { ffi::IntersectClipRect(self.ptr(), rc.left, rc.top, rc.right, rc.bottom) })
			.to_invalidparm()
	}

	/// [`InvertRgn`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-invertrgn)
	/// function.
	pub fn InvertRgn(&self, hrgn: &HRGN) -> SysResult<()> {
		BoolRet(unsafe { ffi::InvertRgn(self.ptr(), hrgn.ptr()) }).to_invalidparm()
	}

	/// [`LineTo`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-lineto)
	/// function.
	pub fn LineTo(&self, x: i32, y: i32) -> SysResult<()> {
		BoolRet(unsafe { ffi::LineTo(self.ptr(), x, y) }).to_invalidparm()
	}

	/// [`MaskBlt`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-maskblt)
	/// function.
	pub fn MaskBlt(
		&self,
		dest_top_left: POINT,
		sz: SIZE,
		hdc_src: &HDC,
		src_top_left: POINT,
		hbm_mask: &HBITMAP,
		mask_offset: POINT,
		rop: co::ROP,
	) -> SysResult<()> {
		BoolRet(unsafe {
			ffi::MaskBlt(
				self.ptr(),
				dest_top_left.x,
				dest_top_left.y,
				sz.cx,
				sz.cy,
				hdc_src.ptr(),
				src_top_left.x,
				src_top_left.y,
				hbm_mask.ptr(),
				mask_offset.x,
				mask_offset.y,
				rop.raw(),
			)
		})
		.to_invalidparm()
	}

	/// [`MoveToEx`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-movetoex)
	/// function.
	pub fn MoveToEx(&self, x: i32, y: i32, pt: Option<&mut POINT>) -> SysResult<()> {
		BoolRet(unsafe { ffi::MoveToEx(self.ptr(), x, y, pvoid_or_null(pt)) }).to_invalidparm()
	}

	/// [`PaintRgn`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-paintrgn)
	/// function.
	pub fn PaintRgn(&self, hrgn: &HRGN) -> SysResult<()> {
		BoolRet(unsafe { ffi::PaintRgn(self.ptr(), hrgn.ptr()) }).to_invalidparm()
	}

	/// [`PatBlt`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-patblt)
	/// function.
	pub fn PatBlt(&self, top_left: POINT, sz: SIZE, rop: co::ROP) -> SysResult<()> {
		BoolRet(unsafe { ffi::PatBlt(self.ptr(), top_left.x, top_left.y, sz.cx, sz.cy, rop.raw()) })
			.to_invalidparm()
	}

	/// [`PathToRegion`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-pathtoregion)
	/// function.
	#[must_use]
	pub fn PathToRegion(&self) -> SysResult<DeleteObjectGuard<HRGN>> {
		unsafe {
			PtrRet(ffi::PathToRegion(self.ptr()))
				.to_invalidparm_handle()
				.map(|h| DeleteObjectGuard::new(h))
		}
	}

	/// [`Pie`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-pie)
	/// function.
	pub fn Pie(&self, bounds: RECT, radial_1: POINT, radial_2: POINT) -> SysResult<()> {
		BoolRet(unsafe {
			ffi::Pie(
				self.ptr(),
				bounds.left,
				bounds.top,
				bounds.right,
				bounds.bottom,
				radial_1.x,
				radial_1.y,
				radial_2.y,
				radial_2.y,
			)
		})
		.to_invalidparm()
	}

	/// [`AtlPixelToHiMetric`](https://learn.microsoft.com/en-us/cpp/atl/reference/pixel-himetric-conversion-global-functions?view=msvc-170#atlpixeltohimetric)
	/// function.
	///
	/// Converts pixels to HIMETRIC units. The inverse operation is
	/// [`HDC::HiMetricToPixel`](crate::HDC::HiMetricToPixel).
	#[must_use]
	pub fn PixelToHiMetric(&self, x: i32, y: i32) -> (i32, i32) {
		(
			MulDiv(x, HIMETRIC_PER_INCH, self.GetDeviceCaps(co::GDC::LOGPIXELSX)),
			MulDiv(y, HIMETRIC_PER_INCH, self.GetDeviceCaps(co::GDC::LOGPIXELSY)),
		)
	}

	/// [`PolyBezier`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-polybezier)
	/// function.
	pub fn PolyBezier(&self, pts: &[POINT]) -> SysResult<()> {
		BoolRet(unsafe { ffi::PolyBezier(self.ptr(), vec_ptr(pts) as _, pts.len() as _) })
			.to_invalidparm()
	}

	/// [`PolyBezierTo`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-polybezierto)
	/// function.
	pub fn PolyBezierTo(&self, pts: &[POINT]) -> SysResult<()> {
		BoolRet(unsafe { ffi::PolyBezierTo(self.ptr(), vec_ptr(pts) as _, pts.len() as _) })
			.to_invalidparm()
	}

	/// [`PolyDraw`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-polydraw)
	/// function.
	pub fn PolyDraw(&self, pts: &[(POINT, co::PT)]) -> SysResult<()> {
		let (pts, ajs): (Vec<_>, Vec<_>) = pts.iter().cloned().unzip();
		BoolRet(unsafe {
			ffi::PolyDraw(self.ptr(), pts.as_ptr() as _, ajs.as_ptr() as _, pts.len() as _)
		})
		.to_invalidparm()
	}

	/// [`Polygon`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-polygon)
	/// function.
	pub fn Polygon(&self, pts: &[POINT]) -> SysResult<()> {
		BoolRet(unsafe { ffi::Polygon(self.ptr(), pts.as_ptr() as _, pts.len() as _) })
			.to_invalidparm()
	}

	/// [`Polyline`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-polyline)
	/// function.
	pub fn Polyline(&self, pts: &[POINT]) -> SysResult<()> {
		BoolRet(unsafe { ffi::Polyline(self.ptr(), vec_ptr(pts) as _, pts.len() as _) })
			.to_invalidparm()
	}

	/// [`PolylineTo`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-polylineto)
	/// function.
	pub fn PolylineTo(&self, pts: &[POINT]) -> SysResult<()> {
		BoolRet(unsafe { ffi::PolylineTo(self.ptr(), vec_ptr(pts) as _, pts.len() as _) })
			.to_invalidparm()
	}

	/// [`PolyPolygon`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-polypolygon)
	/// function.
	pub fn PolyPolygon(&self, polygons: &[&[POINT]]) -> SysResult<()> {
		let all_pts_flat = polygons
			.iter()
			.flat_map(|pts| pts.iter())
			.map(|pt| *pt)
			.collect::<Vec<_>>();
		let pts_per_polygon = polygons
			.iter()
			.map(|pts| pts.len() as i32)
			.collect::<Vec<_>>();

		BoolRet(unsafe {
			ffi::PolyPolygon(
				self.ptr(),
				vec_ptr(&all_pts_flat) as _,
				vec_ptr(&pts_per_polygon) as _,
				polygons.len() as _,
			)
		})
		.to_invalidparm()
	}

	/// [`PolyPolyline`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-polypolyline)
	/// function.
	pub fn PolyPolyline(&self, polylines: &[&[POINT]]) -> SysResult<()> {
		let all_pts_flat = polylines
			.iter()
			.flat_map(|pts| pts.iter())
			.map(|pt| *pt)
			.collect::<Vec<_>>();
		let pts_per_polyline = polylines
			.iter()
			.map(|pts| pts.len() as u32)
			.collect::<Vec<_>>();

		BoolRet(unsafe {
			ffi::PolyPolyline(
				self.ptr(),
				vec_ptr(&all_pts_flat) as _,
				vec_ptr(&pts_per_polyline) as _,
				polylines.len() as _,
			)
		})
		.to_invalidparm()
	}

	/// [`PtVisible`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-ptvisible)
	/// function.
	#[must_use]
	pub fn PtVisible(&self, x: i32, y: i32) -> SysResult<bool> {
		match unsafe { ffi::PtVisible(self.ptr(), x, y) } {
			-1 => Err(co::ERROR::INVALID_PARAMETER),
			0 => Ok(false),
			_ => Ok(true),
		}
	}

	/// [`RealizePalette`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-realizepalette)
	/// function.
	pub fn RealizePalette(&self) -> SysResult<u32> {
		match unsafe { ffi::RealizePalette(self.ptr()) } {
			GDI_ERROR => Err(co::ERROR::INVALID_PARAMETER),
			num => Ok(num),
		}
	}

	/// [`Rectangle`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-rectangle)
	/// function.
	pub fn Rectangle(&self, bounds: RECT) -> SysResult<()> {
		BoolRet(unsafe {
			ffi::Rectangle(self.ptr(), bounds.left, bounds.top, bounds.right, bounds.bottom)
		})
		.to_invalidparm()
	}

	/// [`RestoreDC`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-restoredc)
	/// function.
	pub fn RestoreDC(&self, saved_dc: i32) -> SysResult<()> {
		BoolRet(unsafe { ffi::RestoreDC(self.ptr(), saved_dc) }).to_invalidparm()
	}

	/// [`RoundRect`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-roundrect)
	/// function.
	pub fn RoundRect(&self, bounds: RECT, sz: SIZE) -> SysResult<()> {
		BoolRet(unsafe {
			ffi::RoundRect(
				self.ptr(),
				bounds.left,
				bounds.top,
				bounds.right,
				bounds.bottom,
				sz.cx,
				sz.cy,
			)
		})
		.to_invalidparm()
	}

	/// [`SaveDC`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-savedc)
	/// function.
	pub fn SaveDC(&self) -> SysResult<i32> {
		match unsafe { ffi::SaveDC(self.ptr()) } {
			0 => Err(co::ERROR::INVALID_PARAMETER),
			v => Ok(v),
		}
	}

	/// [`SelectClipPath`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-selectclippath)
	/// function.
	pub fn SelectClipPath(&self, mode: co::RGN) -> SysResult<()> {
		BoolRet(unsafe { ffi::SelectClipPath(self.ptr(), mode.raw()) }).to_invalidparm()
	}

	/// [`SelectClipRgn`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-selectcliprgn)
	/// function.
	pub fn SelectClipRgn(&self, rgn: &HRGN) -> SysResult<co::REGION> {
		match unsafe { ffi::SelectClipRgn(self.ptr(), rgn.ptr()) } {
			0 => Err(co::ERROR::INVALID_PARAMETER),
			v => Ok(unsafe { co::REGION::from_raw(v) }),
		}
	}

	/// [`SelectObject`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-selectobject)
	/// function.
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
	/// ```no_run
	/// use winsafe::{self as w, prelude::*, co};
	///
	/// let hdc: w::HDC; // initialized somewhere
	/// # let hdc = w::HDC::NULL;
	///
	/// let hpen = w::HPEN::CreatePen(
	///     co::PS::SOLID,
	///     1,
	///     w::COLORREF::from_rgb(0xff, 0x00, 0x88),
	/// )?;
	///
	/// let _pen_guard = hdc.SelectObject(&*hpen); // keep guard alive
	/// # w::SysResult::Ok(())
	/// ```
	#[must_use]
	pub fn SelectObject<G: GdiObject>(&self, hgdiobj: &G) -> SysResult<SelectObjectGuard<'_, G>> {
		unsafe {
			PtrRet(ffi::SelectObject(self.ptr(), hgdiobj.ptr()))
				.to_invalidparm()
				.map(|ptr| {
					if hgdiobj.type_id() == TypeId::of::<HRGN>() {
						SelectObjectGuard::new(
							self,
							G::NULL, // regions don't need cleanup
							Some(co::REGION::from_raw(ptr as _)),
						)
					} else {
						SelectObjectGuard::new(
							self,
							G::from_ptr(ptr), // GDI object to be passed to SelectObject at the end of scope
							None,
						)
					}
				})
		}
	}

	/// [`SelectPalette`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-selectpalette)
	/// function.
	pub fn SelectPalette(&self, hpal: &HPALETTE, force_bkgd: bool) -> SysResult<HPALETTE> {
		PtrRet(unsafe { ffi::SelectPalette(self.ptr(), hpal.ptr(), force_bkgd as _) })
			.to_invalidparm_handle()
	}

	/// [`SetArcDirection`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-setarcdirection)
	/// function.
	pub fn SetArcDirection(&self, dir: co::AD) -> SysResult<co::AD> {
		match unsafe { ffi::SetArcDirection(self.ptr(), dir.raw()) } {
			0 => Err(co::ERROR::INVALID_PARAMETER),
			v => Ok(unsafe { co::AD::from_raw(v) }),
		}
	}

	/// [`SetBkColor`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-setbkcolor)
	/// function.
	pub fn SetBkColor(&self, color: COLORREF) -> SysResult<COLORREF> {
		match unsafe { ffi::SetBkColor(self.ptr(), color.into()) } {
			CLR_INVALID => Err(co::ERROR::INVALID_PARAMETER),
			old => Ok(unsafe { COLORREF::from_raw(old) }),
		}
	}

	/// [`SetBkMode`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-setbkmode)
	/// function.
	pub fn SetBkMode(&self, mode: co::BKMODE) -> SysResult<co::BKMODE> {
		match unsafe { ffi::SetBkMode(self.ptr(), mode.raw()) } {
			0 => Err(co::ERROR::INVALID_PARAMETER),
			v => Ok(unsafe { co::BKMODE::from_raw(v) }),
		}
	}

	/// [`SetBrushOrgEx`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-setbrushorgex)
	/// function.
	pub fn SetBrushOrgEx(&self, new_origin: POINT) -> SysResult<POINT> {
		let mut old_origin = POINT::default();
		BoolRet(unsafe {
			ffi::SetBrushOrgEx(self.ptr(), new_origin.x, new_origin.y, pvoid(&mut old_origin))
		})
		.to_invalidparm()
		.map(|_| old_origin)
	}

	/// [`SetDCBrushColor`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-setdcbrushcolor)
	/// function.
	pub fn SetDCBrushColor(&self, color: COLORREF) -> SysResult<COLORREF> {
		match unsafe { ffi::SetDCBrushColor(self.ptr(), color.into()) } {
			CLR_INVALID => Err(co::ERROR::INVALID_PARAMETER),
			old => Ok(unsafe { COLORREF::from_raw(old) }),
		}
	}

	/// [`SetDCPenColor`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-setdcpencolor)
	/// function.
	pub fn SetDCPenColor(&self, color: COLORREF) -> SysResult<COLORREF> {
		match unsafe { ffi::SetDCPenColor(self.ptr(), color.into()) } {
			CLR_INVALID => Err(co::ERROR::INVALID_PARAMETER),
			old => Ok(unsafe { COLORREF::from_raw(old) }),
		}
	}

	/// [`SetDIBits`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-setdibits)
	/// function.
	pub fn SetDIBits(
		&self,
		hbm: &HBITMAP,
		first_scan_line: u32,
		num_scan_lines: u32,
		dib_color_data: &[u8],
		bmi: &BITMAPINFO,
		color_use: co::DIB,
	) -> SysResult<i32> {
		match unsafe {
			ffi::SetDIBits(
				self.ptr(),
				hbm.ptr(),
				first_scan_line,
				num_scan_lines,
				vec_ptr(dib_color_data) as _,
				pcvoid(bmi),
				color_use.raw(),
			)
		} {
			0 => Err(co::ERROR::INVALID_PARAMETER),
			n => Ok(n),
		}
	}

	/// [`SetGraphicsMode`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-setgraphicsmode)
	/// function.
	pub fn SetGraphicsMode(&self, mode: co::GM) -> SysResult<co::GM> {
		match unsafe { ffi::SetGraphicsMode(self.ptr(), mode.raw()) } {
			0 => Err(co::ERROR::INVALID_PARAMETER),
			v => Ok(unsafe { co::GM::from_raw(v) }),
		}
	}

	/// [`SetStretchBltMode`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-setstretchbltmode)
	/// function.
	pub fn SetStretchBltMode(&self, mode: co::STRETCH_MODE) -> SysResult<co::STRETCH_MODE> {
		match unsafe { co::ERROR::from_raw(ffi::SetStretchBltMode(self.ptr(), mode.raw()) as _) } {
			co::ERROR::INVALID_PARAMETER => Err(co::ERROR::INVALID_PARAMETER),
			err_val => Ok(unsafe { co::STRETCH_MODE::from_raw(err_val.raw() as _) }),
		}
	}

	/// [`SetTextAlign`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-settextalign)
	/// function.
	pub fn SetTextAlign(&self, align: co::TA) -> SysResult<co::TA> {
		match unsafe { ffi::SetTextAlign(self.ptr(), align.raw()) } {
			GDI_ERROR => Err(co::ERROR::INVALID_PARAMETER),
			ta => Ok(unsafe { co::TA::from_raw(ta) }),
		}
	}

	/// [`SetTextColor`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-settextcolor)
	/// function.
	pub fn SetTextColor(&self, color: COLORREF) -> SysResult<COLORREF> {
		match unsafe { ffi::SetTextColor(self.ptr(), color.into()) } {
			CLR_INVALID => Err(co::ERROR::INVALID_PARAMETER),
			old => Ok(unsafe { COLORREF::from_raw(old) }),
		}
	}

	/// [`SetTextJustification`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-settextjustification)
	/// function.
	pub fn SetTextJustification(&self, extra: i32, count: i32) -> SysResult<()> {
		BoolRet(unsafe { ffi::SetTextJustification(self.ptr(), extra, count) }).to_invalidparm()
	}

	/// [`SetViewportExtEx`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-setviewportextex)
	/// function.
	pub fn SetViewportExtEx(&self, x: i32, y: i32) -> SysResult<SIZE> {
		let mut sz = SIZE::default();
		BoolRet(unsafe { ffi::SetViewportExtEx(self.ptr(), x, y, pvoid(&mut sz)) })
			.to_invalidparm()
			.map(|_| sz)
	}

	/// [`SetViewportOrgEx`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-setviewportorgex)
	/// function.
	pub fn SetViewportOrgEx(&self, x: i32, y: i32) -> SysResult<POINT> {
		let mut pt = POINT::default();
		BoolRet(unsafe { ffi::SetViewportOrgEx(self.ptr(), x, y, pvoid(&mut pt)) })
			.to_invalidparm()
			.map(|_| pt)
	}

	/// [`SetWindowExtEx`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-setwindowextex)
	/// function.
	pub fn SetWindowExtEx(&self, x: i32, y: i32) -> SysResult<SIZE> {
		let mut sz = SIZE::default();
		BoolRet(unsafe { ffi::SetWindowExtEx(self.ptr(), x, y, pvoid(&mut sz)) })
			.to_invalidparm()
			.map(|_| sz)
	}

	/// [`SetWindowOrgEx`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-setwindoworgex)
	/// function.
	pub fn SetWindowOrgEx(&self, x: i32, y: i32) -> SysResult<POINT> {
		let mut pt = POINT::default();
		BoolRet(unsafe { ffi::SetWindowOrgEx(self.ptr(), x, y, pvoid(&mut pt)) })
			.to_invalidparm()
			.map(|_| pt)
	}

	/// [`StretchBlt`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-stretchblt)
	/// function.
	pub fn StretchBlt(
		&self,
		pos_dest: POINT,
		sz_dest: SIZE,
		hdc_src: &HDC,
		pt_src: POINT,
		sz_src: SIZE,
		rop: co::ROP,
	) -> SysResult<()> {
		BoolRet(unsafe {
			ffi::StretchBlt(
				self.ptr(),
				pos_dest.x,
				pos_dest.y,
				sz_dest.cx,
				sz_dest.cy,
				hdc_src.ptr(),
				pt_src.x,
				pt_src.y,
				sz_src.cx,
				sz_src.cy,
				rop.raw(),
			)
		})
		.to_invalidparm()
	}

	/// [`StrokeAndFillPath`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-strokeandfillpath)
	/// function.
	pub fn StrokeAndFillPath(&self) -> SysResult<()> {
		BoolRet(unsafe { ffi::StrokeAndFillPath(self.ptr()) }).to_invalidparm()
	}

	/// [`StrokePath`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-strokepath)
	/// function.
	pub fn StrokePath(&self) -> SysResult<()> {
		BoolRet(unsafe { ffi::StrokePath(self.ptr()) }).to_invalidparm()
	}

	/// [`TextOut`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-textoutw)
	/// function.
	pub fn TextOut(&self, x: i32, y: i32, text: &str) -> SysResult<()> {
		let output = WString::from_str(text);
		BoolRet(unsafe { ffi::TextOutW(self.ptr(), x, y, output.as_ptr(), output.str_len() as _) })
			.to_invalidparm()
	}

	/// [`TransparentBlt`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-transparentblt)
	/// function.
	pub fn TransparentBlt(
		&self,
		dest_top_left: POINT,
		dest_sz: SIZE,
		hdc_src: HDC,
		src_top_left: POINT,
		src_sz: SIZE,
		color_transparent: COLORREF,
	) -> SysResult<()> {
		BoolRet(unsafe {
			ffi::TransparentBlt(
				self.ptr(),
				dest_top_left.x,
				dest_top_left.y,
				dest_sz.cx,
				dest_sz.cy,
				hdc_src.ptr(),
				src_top_left.x,
				src_top_left.y,
				src_sz.cx,
				src_sz.cy,
				color_transparent.into(),
			)
		})
		.to_invalidparm()
	}

	/// [`UpdateColors`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-updatecolors)
	/// function.
	pub fn UpdateColors(&self) -> SysResult<()> {
		BoolRet(unsafe { ffi::UpdateColors(self.ptr()) }).to_invalidparm()
	}

	/// [`WidenPath`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-widenpath)
	/// function.
	pub fn WidenPath(&self) -> SysResult<()> {
		BoolRet(unsafe { ffi::WidenPath(self.ptr()) }).to_invalidparm()
	}
}
