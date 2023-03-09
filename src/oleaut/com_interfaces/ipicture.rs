#![allow(non_camel_case_types, non_snake_case)]

use crate::kernel::decl::WString;
use crate::ole::decl::{ComPtr, HrResult, IPicture};
use crate::ole::privs::ok_to_hrresult;
use crate::oleaut;
use crate::prelude::{ole_IPicture, ole_IStream};
use crate::user::decl::COLORREF;

impl oleaut_IPicture for IPicture {}

/// This trait is enabled with the `oleaut` feature, and provides methods for
/// [`IDispatch`](crate::IDispatch).
///
/// Prefer importing this trait through the prelude:
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// ```
pub trait oleaut_IPicture: ole_IPicture {
	/// [`OleLoadPicture`](https://learn.microsoft.com/en-us/windows/win32/api/olectl/nf-olectl-oleloadpicture)
	/// static method.
	///
	/// # Examples
	///
	/// Parsing an image from raw data:
	///
	/// ```rust,no_run
	/// use winsafe::prelude::*;
	/// use winsafe::{IPicture, IStream};
	///
	/// let stream: IStream; // initialized somewhere
	/// # let stream = IStream::from(unsafe { winsafe::ComPtr::null() });
	///
	/// let picture = IPicture::OleLoadPicture(&stream, None, true)?;
	/// # Ok::<_, winsafe::co::HRESULT>(())
	/// ```
	#[must_use]
	fn OleLoadPicture(
		stream: &impl ole_IStream,
		size: Option<u32>,
		keep_original_format: bool,
	) -> HrResult<IPicture>
	{
		unsafe {
			let mut ppv_queried = ComPtr::null();
			ok_to_hrresult(
				oleaut::ffi::OleLoadPicture(
					stream.ptr().0 as _,
					size.unwrap_or(0) as _,
					!keep_original_format as _, // note: reversed
					&Self::IID as *const _ as _,
					&mut ppv_queried as *mut _ as _,
				),
			).map(|_| IPicture::from(ppv_queried))
		}
	}

	/// [`OleLoadPicturePath`](https://learn.microsoft.com/en-us/windows/win32/api/olectl/nf-olectl-oleloadpicturepath)
	/// static method.
	///
	/// The picture must be in BMP (bitmap), JPEG, WMF (metafile), ICO (icon),
	/// or GIF format.
	#[must_use]
	fn OleLoadPicturePath(
		path: &str,
		transparent_color: Option<COLORREF>,
	) -> HrResult<IPicture>
	{
		unsafe {
			let mut ppv_queried = ComPtr::null();
			ok_to_hrresult(
				oleaut::ffi::OleLoadPicturePath(
					WString::from_str(path).as_ptr(),
					std::ptr::null_mut(),
					0,
					transparent_color.map_or(0, |c| c.0),
					&Self::IID as *const _ as _,
					&mut ppv_queried as *mut _ as _,
				)
			).map(|_| IPicture::from(ppv_queried))
		}
	}
}
