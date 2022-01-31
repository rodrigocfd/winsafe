#![allow(non_snake_case)]

use crate::{co, oleaut};
use crate::ffi_types::{BOOL, HANDLE, HRES, PCVOID};
use crate::kernel::decl::WString;
use crate::ole::decl::{ComPtr, HrResult};
use crate::ole::privs::ok_to_hrresult;
use crate::prelude::{Handle, OleIUnknown, ShlwapiIStream};
use crate::shlwapi::decl::IStream;
use crate::user::decl::{COLORREF, HBITMAP, HDC, POINT, RECT, SIZE};
use crate::vt::IUnknownVT;

/// [`IPicture`](crate::IPicture) virtual table.
#[cfg_attr(docsrs, doc(cfg(feature = "oleaut")))]
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

/// [`IPicture`](https://docs.microsoft.com/en-us/windows/win32/api/ocidl/nn-ocidl-ipicture)
/// COM interface over [`IPictureVT`](crate::vt::IPictureVT).
///
/// Automatically calls
/// [`Release`](https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
/// when the object goes out of scope.
#[cfg_attr(docsrs, doc(cfg(feature = "oleaut")))]
pub struct IPicture(ComPtr);

impl_iunknown!(IPicture, 0x7bf80980, 0xbf32, 0x101a, 0x8bbb, 0x00aa00300cab);
impl OleautIPicture for IPicture {}

/// [`IPicture`](crate::IPicture) methods from `oleaut` feature.
#[cfg_attr(docsrs, doc(cfg(feature = "oleaut")))]
pub trait OleautIPicture: OleIUnknown {
	/// Calls
	/// [`OleLoadPicturePath`](https://docs.microsoft.com/en-us/windows/win32/api/olectl/nf-olectl-oleloadpicturepath)
	/// to load a picture from a file.
	///
	/// The picture must be in BMP (bitmap), JPEG, WMF (metafile), ICO (icon),
	/// or GIF format.
	fn from_file(
		path: &str,
		transparent_color: Option<COLORREF>) -> HrResult<IPicture>
	{
		let mut ppv_queried = ComPtr::null();
		ok_to_hrresult(
			unsafe {
				oleaut::ffi::OleLoadPicturePath(
					WString::from_str(path).as_ptr(),
					std::ptr::null_mut(),
					0,
					transparent_color.map_or(0, |c| c.0),
					&Self::IID as *const _ as _,
					&mut ppv_queried as *mut _ as _,
				)
			},
		).map(|_| IPicture::from(ppv_queried))
	}

	/// Calls
	/// [`IStream::from_slice`](crate::prelude::ShlwapiIStream::from_slice) and
	/// [`IPicture::from_stream`](crate::prelude::OleautIPicture::from_stream)
	/// to load a picture straight from a slice.
	fn from_slice(
		src: &[u8],
		keep_original_format: bool) -> HrResult<IPicture>
	{
		Self::from_stream(&IStream::from_slice(src)?, None, keep_original_format)
	}

	/// Calls
	/// [`OleLoadPicture`](https://docs.microsoft.com/en-us/windows/win32/api/olectl/nf-olectl-oleloadpicture)
	/// to load a picture from a stream.
	fn from_stream(
		stream: &IStream,
		size: Option<u32>,
		keep_original_format: bool) -> HrResult<IPicture>
	{
		let mut ppv_queried = ComPtr::null();
		ok_to_hrresult(
			unsafe {
				oleaut::ffi::OleLoadPicture(
					stream.ptr().0 as _,
					size.unwrap_or(0) as _,
					!keep_original_format as _, // note: reversed
					&Self::IID as *const _ as _,
					&mut ppv_queried as *mut _ as _,
				)
			},
		).map(|_| IPicture::from(ppv_queried))
	}

	/// [`IPicture::get_CurDC`](https://docs.microsoft.com/en-us/windows/win32/api/ocidl/nf-ocidl-ipicture-get_curdc)
	/// method.
	fn get_CurDC(&self) -> HrResult<HDC> {
		let mut hdc = HDC::NULL;
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IPictureVT);
			ok_to_hrresult((vt.get_CurDC)(self.ptr(), &mut hdc.0))
				.map(|_| hdc)
		}
	}

	/// [`IPicture::get_Height`](https://docs.microsoft.com/en-us/windows/win32/api/ocidl/nf-ocidl-ipicture-get_height)
	/// method.
	///
	/// **Note:** Returns a value in HIMETRIC units. To convert it to pixels,
	/// prefer using the simpler
	/// [`IPicture::size_px`](crate::prelude::GdiOleautIPicture::size_px), or
	/// use
	/// [`HDC::HiMetricToPixel`](crate::prelude::GdiOleautHdc::HiMetricToPixel)
	/// to perform the conversion manually.
	///
	/// # Examples
	///
	/// ```rust,ignore
	/// use winsafe::prelude::*;
	/// use winsafe::{HWND, IPicture};
	///
	/// let pic: IPicture; // initialized somewhere
	/// # use winsafe::{CLSID, co::CLSCTX, CoCreateInstance};
	/// # let pic = CoCreateInstance::<IPicture>(&CLSID::new(0,0,0,0,0), None, CLSCTX::INPROC_SERVER)?;
	///
	/// let hdc = HWND::NULL.GetDC()?;
	///
	/// let (_, height) = hdc.HiMetricToPixel(0, pic.get_Height()?);
	/// println!("Height: {} px", height);
	///
	/// HWND::NULL.ReleaseDC(hdc)?;
	/// # Ok::<_, Box<dyn std::error::Error>>(())
	/// ```
	fn get_Height(&self) -> HrResult<i32> {
		let mut h = i32::default();
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IPictureVT);
			ok_to_hrresult((vt.get_Height)(self.ptr(), &mut h))
				.map(|_| h)
		}
	}

	/// [`IPicture::get_Type`](https://docs.microsoft.com/en-us/windows/win32/api/ocidl/nf-ocidl-ipicture-get_type)
	/// method.
	fn get_Type(&self) -> HrResult<co::PICTYPE> {
		let mut ty = i16::default();
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IPictureVT);
			ok_to_hrresult((vt.get_Type)(self.ptr(), &mut ty))
				.map(|_| co::PICTYPE(ty))
		}
	}

	/// [`IPicture::get_Width`](https://docs.microsoft.com/en-us/windows/win32/api/ocidl/nf-ocidl-ipicture-get_width)
	/// method.
	///
	/// **Note:** Returns a value in HIMETRIC units. To convert it to pixels,
	/// prefer using the simpler
	/// [`IPicture::size_px`](crate::prelude::GdiOleautIPicture::size_px), or
	/// use
	/// [`HDC::HiMetricToPixel`](crate::prelude::GdiOleautHdc::HiMetricToPixel)
	/// to perform the conversion manually.
	///
	/// # Examples
	///
	/// ```rust,ignore
	/// use winsafe::prelude::*;
	/// use winsafe::{HWND, IPicture};
	///
	/// let pic: IPicture; // initialized somewhere
	/// # use winsafe::{CLSID, co::CLSCTX, CoCreateInstance};
	/// # let pic = CoCreateInstance::<IPicture>(&CLSID::new(0,0,0,0,0), None, CLSCTX::INPROC_SERVER)?;
	///
	/// let hdc = HWND::NULL.GetDC()?;
	///
	/// let (width, _) = hdc.HiMetricToPixel(pic.get_Width()?, 0);
	/// println!("Width: {} px", width);
	///
	/// HWND::NULL.ReleaseDC(hdc)?;
	/// # Ok::<_, Box<dyn std::error::Error>>(())
	/// ```
	fn get_Width(&self) -> HrResult<i32> {
		let mut w = i32::default();
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IPictureVT);
			ok_to_hrresult((vt.get_Width)(self.ptr(), &mut w))
				.map(|_| w)
		}
	}

	/// [`IPicture::PictureChanged`](https://docs.microsoft.com/en-us/windows/win32/api/ocidl/nf-ocidl-ipicture-picturechanged)
	/// method.
	fn PictureChanged(&self) -> HrResult<()> {
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IPictureVT);
			ok_to_hrresult((vt.PictureChanged)(self.ptr()))
		}
	}

	/// [`IPicture::put_KeepOriginalFormat`](https://docs.microsoft.com/en-us/windows/win32/api/ocidl/nf-ocidl-ipicture-put_keeporiginalformat)
	/// method.
	fn put_KeepOriginalFormat(&self, keep: bool) -> HrResult<()> {
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IPictureVT);
			ok_to_hrresult((vt.put_KeepOriginalFormat)(self.ptr(), keep as _))
		}
	}

	/// [`IPicture::Render`](https://docs.microsoft.com/en-us/windows/win32/api/ocidl/nf-ocidl-ipicture-render)
	/// method.
	fn Render(&self,
		hdc: HDC, dest_pt: POINT, dest_sz: SIZE,
		src_offset: Option<POINT>, src_extent: SIZE,
		metafile_bounds: Option<&RECT>) -> HrResult<()>
	{
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IPictureVT);
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

	/// [`IPicture::SelectPicture`](https://docs.microsoft.com/en-us/windows/win32/api/ocidl/nf-ocidl-ipicture-selectpicture)
	/// method.
	fn SelectPicture(&self, hdc: HDC) -> HrResult<(HDC, HBITMAP)> {
		let mut hdc_out = HDC::NULL;
		let mut hbmp = HBITMAP::NULL;

		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IPictureVT);
			ok_to_hrresult(
				(vt.SelectPicture)(self.ptr(), hdc.0, &mut hdc_out.0, &mut hbmp.0),
			)
		}.map(|_| (hdc_out, hbmp))
	}
}
