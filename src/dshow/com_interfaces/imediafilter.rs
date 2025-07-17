#![allow(non_camel_case_types, non_snake_case)]

use crate::co;
use crate::decl::*;
use crate::dshow::vts::*;
use crate::kernel::privs::*;
use crate::ole::privs::*;
use crate::prelude::*;

com_interface! { IMediaFilter: "56a86899-0ad4-11ce-b03a-0020af0ba770";
	/// [`IMediaFilter`](https://learn.microsoft.com/en-us/windows/win32/api/strmif/nn-strmif-imediafilter)
	/// COM interface.
	///
	/// Automatically calls
	/// [`IUnknown::Release`](https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
}

impl ole_IPersist for IMediaFilter {}
impl dshow_IMediaFilter for IMediaFilter {}

/// This trait is enabled with the `dshow` feature, and provides methods for
/// [`IMediaFilter`](crate::IMediaFilter).
///
/// Prefer importing this trait through the prelude:
///
/// ```no_run
/// use winsafe::prelude::*;
/// ```
pub trait dshow_IMediaFilter: ole_IPersist {
	/// [`IMediaFilter::GetState`](https://learn.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-imediafilter-getstate)
	/// method.
	#[must_use]
	fn GetState(&self, ms_timeout: Option<u32>) -> HrResult<co::FILTER_STATE> {
		let mut fs = co::FILTER_STATE::default();
		HrRet(unsafe {
			(vt::<IMediaFilterVT>(self).GetState)(
				self.ptr(),
				ms_timeout.unwrap_or(INFINITE),
				fs.as_mut(),
			)
		})
		.to_hrresult()
		.map(|_| fs)
	}

	/// [`IMediaFilter::Pause`](https://learn.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-imediafilter-pause)
	/// method.
	fn Pause(&self) -> HrResult<bool> {
		HrRet(unsafe { (vt::<IMediaFilterVT>(self).Pause)(self.ptr()) }).to_bool_hrresult()
	}

	/// [`IMediaFilter::Run`](https://learn.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-imediafilter-run)
	/// method.
	fn Run(&self, start: i64) -> HrResult<bool> {
		HrRet(unsafe { (vt::<IMediaFilterVT>(self).Run)(self.ptr(), start) }).to_bool_hrresult()
	}

	/// [`IMediaFilter::Stop`](https://learn.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-imediafilter-stop)
	/// method.
	fn Stop(&self) -> HrResult<bool> {
		HrRet(unsafe { (vt::<IMediaFilterVT>(self).Stop)(self.ptr()) }).to_bool_hrresult()
	}
}
