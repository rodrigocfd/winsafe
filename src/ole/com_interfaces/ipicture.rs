#![allow(non_camel_case_types, non_snake_case)]

use crate::co;
use crate::kernel::ffi_types::{BOOL, HANDLE, HRES, PCVOID};
use crate::ole::decl::{ComPtr, HrResult};
use crate::ole::privs::ok_to_hrresult;
use crate::prelude::{Handle, ole_IUnknown};
use crate::user::decl::{HBITMAP, HDC, POINT, RECT, SIZE};
use crate::vt::IUnknownVT;

/// [`IPicture`](crate::IPicture) virtual table.
#[cfg_attr(docsrs, doc(cfg(feature = "ole")))]
#[repr(C)]
pub struct IPictureVT {
	pub IUnknownVT: IUnknownVT,
	pub get_Handle: fn(ComPtr, *mut u32) -> HRES,
	pub get_hPal: fn(ComPtr, *mut u32) -> HRES,
	pub get_Type: fn(ComPtr, *mut i16) -> HRES,
	pub get_Width: fn(ComPtr, *mut i32) -> HRES,
	pub get_Height: fn(ComPtr, *mut i32) -> HRES,
	pub Render: fn(ComPtr, HANDLE, i32, i32, i32, i32, i32, i32, i32, i32, PCVOID) -> HRES,
	pub set_hPal: fn(ComPtr, u32) -> HRES,
	pub get_CurDC: fn(ComPtr, *mut HANDLE) -> HRES,
	pub SelectPicture: fn(ComPtr, HANDLE, *mut HANDLE, *mut HANDLE) -> HRES,
	pub get_KeepOriginalFormat: fn(ComPtr, *mut BOOL) -> HRES,
	pub put_KeepOriginalFormat: fn(ComPtr, BOOL) -> HRES,
	pub PictureChanged: fn(ComPtr) -> HRES,
	pub SaveAsFile: fn(ComPtr, *mut ComPtr, BOOL, *mut i32) -> HRES,
	pub get_Attributes: fn(ComPtr, *mut u32) -> HRES,
}

com_interface! { IPicture: "ole";
	"7bf80980-bf32-101a-8bbb-00aa00300cab";
	/// [`IPicture`](https://learn.microsoft.com/en-us/windows/win32/api/ocidl/nn-ocidl-ipicture)
	/// COM interface over [`IPictureVT`](crate::vt::IPictureVT).
	///
	/// Automatically calls
	/// [`Release`](https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
}

impl ole_IPicture for IPicture {}

/// This trait is enabled with the `ole` feature, and provides methods for
/// [`IPicture`](crate::IPicture).
///
/// Prefer importing this trait through the prelude:
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// ```
#[cfg_attr(docsrs, doc(cfg(feature = "ole")))]
pub trait ole_IPicture: ole_IUnknown {
	/// [`IPicture::get_CurDC`](https://learn.microsoft.com/en-us/windows/win32/api/ocidl/nf-ocidl-ipicture-get_curdc)
	/// method.
	#[must_use]
	fn get_CurDC(&self) -> HrResult<HDC> {
		let mut hdc = HDC::NULL;
		unsafe {
			let vt = self.vt_ref::<IPictureVT>();
			ok_to_hrresult((vt.get_CurDC)(self.ptr(), &mut hdc.0))
				.map(|_| hdc)
		}
	}

	/// [`IPicture::get_Height`](https://learn.microsoft.com/en-us/windows/win32/api/ocidl/nf-ocidl-ipicture-get_height)
	/// method.
	///
	/// **Note:** Returns a value in HIMETRIC units. To convert it to pixels,
	/// prefer using the simpler
	/// [`IPicture::size_px`](crate::prelude::gdi_ole_IPicture::size_px), or use
	/// [`HDC::HiMetricToPixel`](crate::prelude::gdi_ole_Hdc::HiMetricToPixel)
	/// to perform the conversion manually.
	///
	/// # Examples
	///
	/// ```rust,ignore
	/// use winsafe::prelude::*;
	/// use winsafe::{HWND, IPicture};
	///
	/// let pic: IPicture; // initialized somewhere
	/// # let pic = IPicture::from(unsafe { winsafe::ComPtr::null() });
	///
	/// let hdc = HWND::NULL.GetDC()?;
	///
	/// let (_, height) = hdc.HiMetricToPixel(0, pic.get_Height()?);
	/// println!("Height: {} px", height);
	///
	/// HWND::NULL.ReleaseDC(hdc)?;
	/// # Ok::<_, Box<dyn std::error::Error>>(())
	/// ```
	#[must_use]
	fn get_Height(&self) -> HrResult<i32> {
		let mut h = i32::default();
		unsafe {
			let vt = self.vt_ref::<IPictureVT>();
			ok_to_hrresult((vt.get_Height)(self.ptr(), &mut h))
				.map(|_| h)
		}
	}

	/// [`IPicture::get_Type`](https://learn.microsoft.com/en-us/windows/win32/api/ocidl/nf-ocidl-ipicture-get_type)
	/// method.
	#[must_use]
	fn get_Type(&self) -> HrResult<co::PICTYPE> {
		let mut ty = i16::default();
		unsafe {
			let vt = self.vt_ref::<IPictureVT>();
			ok_to_hrresult((vt.get_Type)(self.ptr(), &mut ty))
				.map(|_| co::PICTYPE(ty))
		}
	}

	/// [`IPicture::get_Width`](https://learn.microsoft.com/en-us/windows/win32/api/ocidl/nf-ocidl-ipicture-get_width)
	/// method.
	///
	/// **Note:** Returns a value in HIMETRIC units. To convert it to pixels,
	/// prefer using the simpler
	/// [`IPicture::size_px`](crate::prelude::gdi_ole_IPicture::size_px), or use
	/// [`HDC::HiMetricToPixel`](crate::prelude::gdi_ole_Hdc::HiMetricToPixel)
	/// to perform the conversion manually.
	///
	/// # Examples
	///
	/// ```rust,ignore
	/// use winsafe::prelude::*;
	/// use winsafe::{HWND, IPicture};
	///
	/// let pic: IPicture; // initialized somewhere
	/// # let pic = IPicture::from(unsafe { winsafe::ComPtr::null() });
	///
	/// let hdc = HWND::NULL.GetDC()?;
	///
	/// let (width, _) = hdc.HiMetricToPixel(pic.get_Width()?, 0);
	/// println!("Width: {} px", width);
	///
	/// HWND::NULL.ReleaseDC(hdc)?;
	/// # Ok::<_, Box<dyn std::error::Error>>(())
	/// ```
	#[must_use]
	fn get_Width(&self) -> HrResult<i32> {
		let mut w = i32::default();
		unsafe {
			let vt = self.vt_ref::<IPictureVT>();
			ok_to_hrresult((vt.get_Width)(self.ptr(), &mut w))
				.map(|_| w)
		}
	}

	/// [`IPicture::PictureChanged`](https://learn.microsoft.com/en-us/windows/win32/api/ocidl/nf-ocidl-ipicture-picturechanged)
	/// method.
	fn PictureChanged(&self) -> HrResult<()> {
		unsafe {
			let vt = self.vt_ref::<IPictureVT>();
			ok_to_hrresult((vt.PictureChanged)(self.ptr()))
		}
	}

	/// [`IPicture::put_KeepOriginalFormat`](https://learn.microsoft.com/en-us/windows/win32/api/ocidl/nf-ocidl-ipicture-put_keeporiginalformat)
	/// method.
	fn put_KeepOriginalFormat(&self, keep: bool) -> HrResult<()> {
		unsafe {
			let vt = self.vt_ref::<IPictureVT>();
			ok_to_hrresult((vt.put_KeepOriginalFormat)(self.ptr(), keep as _))
		}
	}

	/// [`IPicture::Render`](https://learn.microsoft.com/en-us/windows/win32/api/ocidl/nf-ocidl-ipicture-render)
	/// method.
	fn Render(&self,
		hdc: &HDC,
		dest_pt: POINT,
		dest_sz: SIZE,
		src_offset: Option<POINT>,
		src_extent: SIZE,
		metafile_bounds: Option<&RECT>) -> HrResult<()>
	{
		unsafe {
			let vt = self.vt_ref::<IPictureVT>();
			ok_to_hrresult(
				(vt.Render)(
					self.ptr(),
					hdc.0,
					dest_pt.x, dest_pt.y,
					dest_sz.cx, dest_sz.cy,
					src_offset.map_or(0, |off| off.x),
					src_offset.map_or(0, |off| off.y),
					src_extent.cx, src_extent.cy,
					metafile_bounds.map_or(std::ptr::null_mut(), |rc| rc as *const _ as _),
				),
			)
		}
	}

	/// [`IPicture::SelectPicture`](https://learn.microsoft.com/en-us/windows/win32/api/ocidl/nf-ocidl-ipicture-selectpicture)
	/// method.
	fn SelectPicture(&self, hdc: &HDC) -> HrResult<(HDC, HBITMAP)> {
		let mut hdc_out = HDC::NULL;
		let mut hbmp = HBITMAP::NULL;

		unsafe {
			let vt = self.vt_ref::<IPictureVT>();
			ok_to_hrresult(
				(vt.SelectPicture)(self.ptr(), hdc.0, &mut hdc_out.0, &mut hbmp.0),
			)
		}.map(|_| (hdc_out, hbmp))
	}
}
