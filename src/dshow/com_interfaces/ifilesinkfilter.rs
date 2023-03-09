#![allow(non_camel_case_types, non_snake_case)]

use crate::dshow::decl::AM_MEDIA_TYPE;
use crate::kernel::decl::WString;
use crate::kernel::ffi_types::{HRES, PCSTR, PCVOID, PSTR, PVOID};
use crate::ole::decl::{ComPtr, CoTaskMemFree, HrResult};
use crate::ole::privs::ok_to_hrresult;
use crate::prelude::ole_IUnknown;
use crate::vt::IUnknownVT;

/// [`IFileSinkFilter`](crate::IFileSinkFilter) virtual table.
#[repr(C)]
pub struct IFileSinkFilterVT {
	pub IUnknownVT: IUnknownVT,
	pub SetFileName: fn(ComPtr, PCSTR, PCVOID) -> HRES,
	pub GetCurFile: fn(ComPtr, *mut PSTR, PVOID) -> HRES,
}

com_interface! { IFileSinkFilter: "a2104830-7c70-11cf-8bce-00aa00a3f1a6";
	/// [`IFileSinkFilter`](https://learn.microsoft.com/en-us/windows/win32/api/strmif/nn-strmif-ifilesinkfilter)
	/// COM interface over [`IFileSinkFilterVT`](crate::vt::IFileSinkFilterVT).
	///
	/// Automatically calls
	/// [`IUnknown::Release`](https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
}

impl dshow_IFileSinkFilter for IFileSinkFilter {}

/// This trait is enabled with the `dshow` feature, and provides methods for
/// [`IFileSinkFilter`](crate::IFileSinkFilter).
///
/// Prefer importing this trait through the prelude:
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// ```
pub trait dshow_IFileSinkFilter: ole_IUnknown {
	/// [`IFileSinkFilter::GetCurFile`](https://learn.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-ifilesinkfilter-getcurfile)
	/// method.
	///
	/// # Safety
	///
	/// If you pass an [`AM_MEDIA_TYPE`](crate::AM_MEDIA_TYPE) reference to
	/// `pmt`, its `pbFormat` field may return a valid reference to a format
	/// block. If so, you must free it with
	/// [`CoTaskMemFree`](crate::CoTaskMemFree), or you'll have a memory leak.
	///
	/// # Examples
	///
	/// ```rust,no_run
	/// use winsafe::prelude::*;
	/// use winsafe::{AM_MEDIA_TYPE, CoTaskMemFree, DVINFO, IFileSinkFilter};
	///
	/// let sinkf: IFileSinkFilter; // initialized somewhere
	/// # let sinkf = IFileSinkFilter::from(unsafe { winsafe::ComPtr::null() });
	///
	/// let mut ammt = AM_MEDIA_TYPE::default();
	/// unsafe {
	///     sinkf.GetCurFile(Some(&mut ammt))?;
	///     if let Some(pb_format) = ammt.pbFormat::<DVINFO>() { // valid reference?
	///         CoTaskMemFree(pb_format as *mut _ as _);
	///     }
	/// }
	/// # Ok::<_, winsafe::co::HRESULT>(())
	/// ```
	#[must_use]
	unsafe fn GetCurFile(&self,
		mt: Option<&mut AM_MEDIA_TYPE>) -> HrResult<String>
	{
		let mut pstr: *mut u16 = std::ptr::null_mut();
		let vt = self.vt_ref::<IFileSinkFilterVT>();
		ok_to_hrresult(
			(vt.GetCurFile)(
				self.ptr(),
				&mut pstr,
				mt.map_or(std::ptr::null_mut(), |amt| amt as *mut _ as _),
			),
		).map(|_| {
			let name = WString::from_wchars_nullt(pstr);
			CoTaskMemFree(pstr as _);
			name.to_string()
		})
	}

	/// [`IFileSinkFilter::SetFileName`](https://learn.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-ifilesinkfilter-setfilename)
	/// method.
	fn SetFileName(&self,
		file_name: &str, mt: Option<&AM_MEDIA_TYPE>) -> HrResult<()>
	{
		unsafe {
			let vt = self.vt_ref::<IFileSinkFilterVT>();
			ok_to_hrresult(
				(vt.SetFileName)(
					self.ptr(),
					WString::from_str(file_name).as_ptr(),
					mt.map_or(std::ptr::null(), |amt| amt as *const _ as _),
				),
			)
		}
	}
}
