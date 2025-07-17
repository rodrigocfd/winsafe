#![allow(non_camel_case_types, non_snake_case)]

use crate::decl::*;
use crate::dshow::vts::*;
use crate::kernel::privs::*;
use crate::ole::privs::*;
use crate::prelude::*;

com_interface! { IFileSinkFilter: "a2104830-7c70-11cf-8bce-00aa00a3f1a6";
	/// [`IFileSinkFilter`](https://learn.microsoft.com/en-us/windows/win32/api/strmif/nn-strmif-ifilesinkfilter)
	/// COM interface.
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
/// ```no_run
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
	/// [`CoTaskMemFree`](crate::guard::CoTaskMemFreeGuard), or you'll have a
	/// memory leak.
	///
	/// # Examples
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*, guard};
	///
	/// let sinkf: w::IFileSinkFilter; // initialized somewhere
	/// # let sinkf = unsafe { w::IFileSinkFilter::null() };
	///
	/// let mut ammt = w::AM_MEDIA_TYPE::default();
	/// unsafe {
	///     sinkf.GetCurFile(Some(&mut ammt))?;
	///     if let Some(pb_format) = ammt.pbFormat::<w::DVINFO>() { // valid reference?
	///         let _ = guard::CoTaskMemFreeGuard::new(pb_format as *mut _  as _, 0);
	///     }
	/// }
	/// # w::HrResult::Ok(())
	/// ```
	#[must_use]
	unsafe fn GetCurFile(&self, mt: Option<&mut AM_MEDIA_TYPE>) -> HrResult<String> {
		let mut pstr = std::ptr::null_mut::<u16>();
		HrRet(unsafe {
			(vt::<IFileSinkFilterVT>(self).GetCurFile)(self.ptr(), &mut pstr, pvoid_or_null(mt))
		})
		.to_hrresult()
		.map(|_| htaskmem_ptr_to_str(pstr))
	}

	/// [`IFileSinkFilter::SetFileName`](https://learn.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-ifilesinkfilter-setfilename)
	/// method.
	fn SetFileName(&self, file_name: &str, mt: Option<&AM_MEDIA_TYPE>) -> HrResult<()> {
		HrRet(unsafe {
			(vt::<IFileSinkFilterVT>(self).SetFileName)(
				self.ptr(),
				WString::from_str(file_name).as_ptr(),
				pcvoid_or_null(mt),
			)
		})
		.to_hrresult()
	}
}
