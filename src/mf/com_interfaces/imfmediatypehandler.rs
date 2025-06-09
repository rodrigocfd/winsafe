#![allow(non_camel_case_types, non_snake_case)]

use crate::decl::*;
use crate::kernel::privs::*;
use crate::mf::vts::*;
use crate::ole::privs::*;
use crate::prelude::*;

com_interface! { IMFMediaTypeHandler: "e93dcf6c-4b07-4e1e-8123-aa16ed6eadf5";
	/// [`IMFMediaTypeHandler`](https://learn.microsoft.com/en-us/windows/win32/api/mfidl/nn-mfidl-imfmediatypehandler)
	/// COM interface.
	///
	/// Automatically calls
	/// [`Release`](https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
}

/// This trait is enabled with the `mf` feature, and provides methods for
/// [`IMFMediaTypeHandler`](crate::IMFMediaTypeHandler).
///
/// Prefer importing this trait through the prelude:
///
/// ```no_run
/// use winsafe::prelude::*;
/// ```
pub trait mf_IMFMediaTypeHandler: ole_IUnknown {
	/// [`IMFMediaTypeHandler::GetMajorType`](https://learn.microsoft.com/en-us/windows/win32/api/mfidl/nf-mfidl-imfmediatypehandler-getmajortype)
	/// method.
	#[must_use]
	fn GetMajorType(&self) -> HrResult<GUID> {
		let mut mt = GUID::default();
		ok_to_hrresult(unsafe {
			(vt::<IMFMediaTypeHandlerVT>(self).GetMajorType)(self.ptr(), pvoid(&mut mt))
		})
		.map(|_| mt)
	}

	/// [`IMFMediaTypeHandler::GetMediaTypeCount`](https://learn.microsoft.com/en-us/windows/win32/api/mfidl/nf-mfidl-imfmediatypehandler-getmediatypecount)
	/// method.
	#[must_use]
	fn GetMediaTypeCount(&self) -> HrResult<u32> {
		let mut count = 0u32;
		ok_to_hrresult(unsafe {
			(vt::<IMFMediaTypeHandlerVT>(self).GetMediaTypeCount)(self.ptr(), &mut count)
		})
		.map(|_| count)
	}
}
