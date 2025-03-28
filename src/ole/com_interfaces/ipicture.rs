#![allow(non_camel_case_types, non_snake_case)]

use crate::co;
use crate::decl::*;
use crate::ole::{privs::*, vts::*};
use crate::prelude::*;

com_interface! { IPicture: "7bf80980-bf32-101a-8bbb-00aa00300cab";
	/// [`IPicture`](https://learn.microsoft.com/en-us/windows/win32/api/ocidl/nn-ocidl-ipicture)
	/// COM interface.
	///
	/// Automatically calls
	/// [`Release`](https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
	///
	/// # Examples
	///
	/// Loading an image from bytes:
	///
	/// ```rust,ignore
	/// use winsafe::{self as w, prelude::*};
	///
	/// let image_bytes: Vec<u8>; // initialized somewhere
	/// # let image_bytes = Vec::<u8>::new();
	///
	/// let stream = w::SHCreateMemStream(&image_bytes)?;
	/// let ipic = w::OleLoadPicture(&stream, None, true)?;
	/// # w::HrResult::Ok(())
	/// ```
}

impl ole_IPicture for IPicture {}

/// This trait is enabled with the `ole` feature, and provides methods for
/// [`IPicture`](crate::IPicture).
///
/// Prefer importing this trait through the prelude:
///
/// ```no_run
/// use winsafe::prelude::*;
/// ```
pub trait ole_IPicture: ole_IUnknown {
	/// [`IPicture::get_Attributes`](https://learn.microsoft.com/en-us/windows/win32/api/ocidl/nf-ocidl-ipicture-get_attributes)
	/// method.
	#[must_use]
	fn get_Attributes(&self) -> HrResult<co::PICTURE> {
		let mut attr = co::PICTURE::default();
		ok_to_hrresult(unsafe {
			(vt::<IPictureVT>(self).get_Attributes)(self.ptr(), attr.as_mut())
		})
		.map(|_| attr)
	}

	/// [`IPicture::get_CurDC`](https://learn.microsoft.com/en-us/windows/win32/api/ocidl/nf-ocidl-ipicture-get_curdc)
	/// method.
	#[must_use]
	fn get_CurDC(&self) -> HrResult<HDC> {
		let mut hdc = HDC::NULL;
		ok_to_hrresult(unsafe { (vt::<IPictureVT>(self).get_CurDC)(self.ptr(), hdc.as_mut()) })
			.map(|_| hdc)
	}

	/// [`IPicture::get_Height`](https://learn.microsoft.com/en-us/windows/win32/api/ocidl/nf-ocidl-ipicture-get_height)
	/// method.
	///
	/// Returns the value in HIMETRIC units. If you need pixels, prefer using
	/// [`size_px`](crate::prelude::ole_IPicture::size_px), which performs the
	/// conversion automatically.
	#[must_use]
	fn get_Height(&self) -> HrResult<i32> {
		let mut h = i32::default();
		ok_to_hrresult(unsafe { (vt::<IPictureVT>(self).get_Height)(self.ptr(), &mut h) })
			.map(|_| h)
	}

	/// [`IPicture::get_Type`](https://learn.microsoft.com/en-us/windows/win32/api/ocidl/nf-ocidl-ipicture-get_type)
	/// method.
	#[must_use]
	fn get_Type(&self) -> HrResult<co::PICTYPE> {
		let mut ty = co::PICTYPE::default();
		ok_to_hrresult(unsafe { (vt::<IPictureVT>(self).get_Type)(self.ptr(), ty.as_mut()) })
			.map(|_| ty)
	}

	/// [`IPicture::get_Width`](https://learn.microsoft.com/en-us/windows/win32/api/ocidl/nf-ocidl-ipicture-get_width)
	/// method.
	///
	/// Returns the value in HIMETRIC units. If you need pixels, prefer using
	/// [`size_px`](crate::prelude::ole_IPicture::size_px), which performs the
	/// conversion automatically.
	#[must_use]
	fn get_Width(&self) -> HrResult<i32> {
		let mut w = i32::default();
		ok_to_hrresult(unsafe { (vt::<IPictureVT>(self).get_Width)(self.ptr(), &mut w) }).map(|_| w)
	}

	fn_com_noparm! { PictureChanged: IPictureVT;
		/// [`IPicture::PictureChanged`](https://learn.microsoft.com/en-us/windows/win32/api/ocidl/nf-ocidl-ipicture-picturechanged)
		/// method.
	}

	/// [`IPicture::put_KeepOriginalFormat`](https://learn.microsoft.com/en-us/windows/win32/api/ocidl/nf-ocidl-ipicture-put_keeporiginalformat)
	/// method.
	fn put_KeepOriginalFormat(&self, keep: bool) -> HrResult<()> {
		ok_to_hrresult(unsafe {
			(vt::<IPictureVT>(self).put_KeepOriginalFormat)(self.ptr(), keep as _)
		})
	}

	/// [`IPicture::Render`](https://learn.microsoft.com/en-us/windows/win32/api/ocidl/nf-ocidl-ipicture-render)
	/// method.
	///
	/// This method will automatically perform the inverse height calculations –
	/// convert `src_extent_himetric.cy` to a negative value and compensate
	/// `src_offset_himetric.y`. This is necessary because HIMETRIC height is
	/// rendered in reverse order (bottom to top) when compared to HDC height.
	///
	/// Default values:
	///
	/// | Parameter | Default value |
	/// | -- | :--: |
	/// | `dest_pt` | `0`, `0` |
	/// | `dest_sz` | [`HDC`](crate::HDC) client rect |
	/// | `src_offset_himetric` | `0`, `0` |
	/// | `src_extent_himetric` | image size from [`get_Width`](crate::prelude::ole_IPicture::get_Width), [`get_Height`](crate::prelude::ole_IPicture::get_Height) |
	fn Render(
		&self,
		hdc: &HDC,
		dest_pt: Option<POINT>,
		dest_sz: Option<SIZE>,
		src_offset_himetric: Option<POINT>,
		src_extent_himetric: Option<SIZE>,
		metafile_bounds: Option<&RECT>,
	) -> AnyResult<()> {
		let dest_sz_hdc = match dest_sz {
			Some(sz) => sz,
			None => {
				let rc = hdc.WindowFromDC().unwrap().GetClientRect()?;
				SIZE::new(rc.right - rc.left, rc.bottom - rc.top)
			},
		};
		let src_extent_compensated = match src_extent_himetric {
			Some(sz) => SIZE::new(sz.cx, -sz.cy),
			None => SIZE::new(self.get_Width()?, -self.get_Height()?),
		};

		ok_to_hrresult(unsafe {
			(vt::<IPictureVT>(self).Render)(
				self.ptr(),
				hdc.ptr(),
				dest_pt.map_or(0, |pt| pt.x),
				dest_pt.map_or(0, |pt| pt.y),
				dest_sz_hdc.cx,
				dest_sz_hdc.cy,
				src_offset_himetric.map_or(0, |off| off.x),
				src_offset_himetric.map_or(0, |off| off.y) - src_extent_compensated.cy,
				src_extent_compensated.cx,
				src_extent_compensated.cy,
				metafile_bounds.map_or(std::ptr::null_mut(), |rc| rc as *const _ as _),
			)
		})
		.map_err(|e| e.into())
	}

	/// [`IPicture::SelectPicture`](https://learn.microsoft.com/en-us/windows/win32/api/ocidl/nf-ocidl-ipicture-selectpicture)
	/// method.
	fn SelectPicture(&self, hdc: &HDC) -> HrResult<(HDC, HBITMAP)> {
		let mut hdc_out = HDC::NULL;
		let mut hbmp = HBITMAP::NULL;

		ok_to_hrresult(unsafe {
			(vt::<IPictureVT>(self).SelectPicture)(
				self.ptr(),
				hdc.ptr(),
				hdc_out.as_mut(),
				hbmp.as_mut(),
			)
		})
		.map(|_| (hdc_out, hbmp))
	}

	/// Retrieves the image size in pixels by calling
	/// [`get_Width`](crate::prelude::ole_IPicture::get_Width) and
	/// [`get_Height`](crate::prelude::ole_IPicture::get_Height), passing the
	/// values to
	/// [`HDC::HiMetricToPixel`](crate::prelude::gdi_Hdc::HiMetricToPixel), using
	/// the `HDC` for the entire screen retrieved with
	/// [`HWND::GetDC`](crate::prelude::user_Hwnd::GetDC).
	fn size_px(&self) -> AnyResult<(i32, i32)> {
		let width = self.get_Width()?;
		let height = self.get_Height()?;
		let hdc_screen = HWND::NULL.GetDC()?;
		Ok(hdc_screen.HiMetricToPixel(width, height))
	}
}
