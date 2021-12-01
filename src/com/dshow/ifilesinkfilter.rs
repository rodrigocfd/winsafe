#![allow(non_snake_case)]

use crate::aliases::HrResult;
use crate::com::dshow::any_structs::AM_MEDIA_TYPE;
use crate::com::funcs::CoTaskMemFree;
use crate::com::iunknown::{ComPtr, IUnknownT, IUnknownVT};
use crate::ffi::{HRES, PCSTR, PCVOID, PSTR, PVOID};
use crate::privs::ok_to_hrresult;
use crate::various::WString;

/// [`IFileSinkFilter`](crate::dshow::IFileSinkFilter) virtual table.
#[repr(C)]
pub struct IFileSinkFilterVT {
	pub IUnknownVT: IUnknownVT,
	pub SetFileName: fn(ComPtr, PCSTR, PCVOID) -> HRES,
	pub GetCurFile: fn(ComPtr, *mut PSTR, PVOID) -> HRES,
}

/// [`IFileSinkFilter`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nn-strmif-ifilesinkfilter)
/// COM interface over
/// [`IFileSinkFilterVT`](crate::dshow::vt::IFileSinkFilterVT)..
///
/// Automatically calls
/// [`IUnknown::Release`](https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
/// when the object goes out of scope.
pub struct IFileSinkFilter(ComPtr);

impl_iunknown!(IFileSinkFilter, 0xa2104830, 0x7c70, 0x11cf, 0x8bce, 0x00aa00a3f1a6);
impl IFileSinkFilterT for IFileSinkFilter {}

/// Exposes the [`IFileSinkFilter`](crate::dshow::IFileSinkFilter) methods.
pub trait IFileSinkFilterT: IUnknownT {
	/// [`IFileSinkFilter::GetCurFile`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-ifilesinkfilter-getcurfile)
	/// method.
	///
	/// If you pass an [`AM_MEDIA_TYPE`](crate::dshow::AM_MEDIA_TYPE) reference
	/// to `pmt`, its `pbFormat` field may return a valid reference to a format
	/// block. If so, you must free it with
	/// [`CoTaskMemFree`](crate::CoTaskMemFree), or you'll have a memory leak.
	///
	/// # Examples
	///
	/// ```rust,ignore
	/// use winsafe::prelude::*;
	/// use winsafe::{CoMemTaskFree, dshow};
	///
	/// let isink: dshow::IFileSinkFilter; // initialized somewhere
	///
	/// let mut ammt = dshow::AM_MEDIA_TYPE::default();
	/// unsafe {
	///     isink.GetCurFile(Some(&mut ammt))?;
	///     if let Some(pb_format) = ammt.pbFormat::<dshow::DVINFO>() {
	///         CoTaskMemFree(pb_format);
	///     }
	/// }
	/// ```
	unsafe fn GetCurFile(&self,
		mt: Option<&mut AM_MEDIA_TYPE>) -> HrResult<String>
	{
		let mut pstr: *mut u16 = std::ptr::null_mut();
		let vt = &**(self.ptr().0 as *mut *mut IFileSinkFilterVT);
		ok_to_hrresult(
			(vt.GetCurFile)(
				self.ptr(),
				&mut pstr,
				mt.map_or(std::ptr::null_mut(), |p| p as *mut _ as _),
			),
		).map(|_| {
			let name = WString::from_wchars_nullt(pstr);
			CoTaskMemFree(pstr);
			name.to_string()
		})
	}

	/// [`IFileSinkFilter::SetFileName`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-ifilesinkfilter-setfilename)
	/// method.
	fn SetFileName(&self,
		file_name: &str, mt: Option<&AM_MEDIA_TYPE>) -> HrResult<()>
	{
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IFileSinkFilterVT);
			ok_to_hrresult(
				(vt.SetFileName)(
					self.ptr(),
					WString::from_str(file_name).as_ptr(),
					mt.map_or(std::ptr::null(), |p| p as *const _ as _),
				),
			)
		}
	}
}
