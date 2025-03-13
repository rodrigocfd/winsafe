#![allow(non_camel_case_types, non_snake_case)]

use crate::decl::*;
use crate::mf::vts::*;
use crate::ole::privs::*;
use crate::prelude::*;

com_interface! { IMFStreamDescriptor: "56c03d9c-9dbb-45f5-ab4b-d80f47c05938";
	/// [`IMFStreamDescriptor`](https://learn.microsoft.com/en-us/windows/win32/api/mfidl/nn-mfidl-imfstreamdescriptor)
	/// COM interface.
	///
	/// Automatically calls
	/// [`Release`](https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
}

impl mf_IMFAttributes for IMFStreamDescriptor {}
impl mf_IMFStreamDescriptor for IMFStreamDescriptor {}

/// This trait is enabled with the `mf` feature, and provides methods for
/// [`IMFStreamDescriptor`](crate::IMFStreamDescriptor).
///
/// Prefer importing this trait through the prelude:
///
/// ```no_run
/// use winsafe::prelude::*;
/// ```
pub trait mf_IMFStreamDescriptor: mf_IMFAttributes {
	fn_com_interface_get! { GetMediaTypeHandler: IMFStreamDescriptorVT, IMFMediaTypeHandler;
		/// [`IMFStreamDescriptor::GetMediaTypeHandler`](https://learn.microsoft.com/en-us/windows/win32/api/mfidl/nf-mfidl-imfstreamdescriptor-getmediatypehandler)
		/// method.
	}

	/// [`IMFStreamDescriptor::GetStreamIdentifier`](https://learn.microsoft.com/en-us/windows/win32/api/mfidl/nf-mfidl-imfstreamdescriptor-getstreamidentifier)
	/// method.
	#[must_use]
	fn GetStreamIdentifier(&self) -> HrResult<u32> {
		let mut id = u32::default();
		ok_to_hrresult(unsafe {
			(vt::<IMFStreamDescriptorVT>(self).GetStreamIdentifier)(self.ptr(), &mut id)
		})
		.map(|_| id)
	}
}
