#![allow(non_camel_case_types, non_snake_case)]

use crate::decl::*;
use crate::mf::vts::*;
use crate::ole::privs::*;
use crate::prelude::*;

com_interface! { IMFPresentationDescriptor: "03cb2711-24d7-4db6-a17f-f3a7a479a536";
	/// [`IMFPresentationDescriptor`](https://learn.microsoft.com/en-us/windows/win32/api/mfidl/nn-mfidl-imfpresentationdescriptor)
	/// COM interface.
	///
	/// Automatically calls
	/// [`Release`](https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
}

impl mf_IMFAttributes for IMFPresentationDescriptor {}
impl mf_IMFPresentationDescriptor for IMFPresentationDescriptor {}

/// This trait is enabled with the `mf` feature, and provides methods for
/// [`IMFPresentationDescriptor`](crate::IMFPresentationDescriptor).
///
/// Prefer importing this trait through the prelude:
///
/// ```no_run
/// use winsafe::prelude::*;
/// ```
pub trait mf_IMFPresentationDescriptor: mf_IMFAttributes {
	fn_com_interface_get! { Clone: IMFPresentationDescriptorVT, IMFPresentationDescriptor;
		/// [`IMFPresentationDescriptor::Clone`](https://learn.microsoft.com/en-us/windows/win32/api/mfidl/nf-mfidl-imfpresentationdescriptor-clone)
		/// method.
	}

	/// [`IMFPresentationDescriptor::DeselectStream`](https://learn.microsoft.com/en-us/windows/win32/api/mfidl/nf-mfidl-imfpresentationdescriptor-deselectstream)
	/// method.
	fn DeselectStream(&self, descriptor_index: u32) -> HrResult<()> {
		ok_to_hrresult(unsafe {
			(vt::<IMFPresentationDescriptorVT>(self).DeselectStream)(self.ptr(), descriptor_index)
		})
	}

	/// [`IMFPresentationDescriptor::GetStreamDescriptorByIndex`](https://learn.microsoft.com/en-us/windows/win32/api/mfidl/nf-mfidl-imfpresentationdescriptor-getstreamdescriptorbyindex)
	/// method.
	#[must_use]
	fn GetStreamDescriptorByIndex(&self, index: u32) -> HrResult<(bool, IMFStreamDescriptor)> {
		let mut selected = 0;
		let mut queried = unsafe { IMFStreamDescriptor::null() };

		ok_to_hrresult(unsafe {
			(vt::<IMFPresentationDescriptorVT>(self).GetStreamDescriptorByIndex)(
				self.ptr(),
				index,
				&mut selected,
				queried.as_mut(),
			)
		})
		.map(|_| (selected != 0, queried))
	}

	/// [`IMFPresentationDescriptor::GetStreamDescriptorCount`](https://learn.microsoft.com/en-us/windows/win32/api/mfidl/nf-mfidl-imfpresentationdescriptor-getstreamdescriptorcount)
	/// method.
	#[must_use]
	fn GetStreamDescriptorCount(&self) -> HrResult<u32> {
		let mut descriptor_count = 0u32;
		ok_to_hrresult(unsafe {
			(vt::<IMFPresentationDescriptorVT>(self).GetStreamDescriptorCount)(
				self.ptr(),
				&mut descriptor_count,
			)
		})
		.map(|_| descriptor_count)
	}

	/// [`IMFPresentationDescriptor::SelectStream`](https://learn.microsoft.com/en-us/windows/win32/api/mfidl/nf-mfidl-imfpresentationdescriptor-selectstream)
	/// method.
	fn SelectStream(&self, descriptor_index: u32) -> HrResult<()> {
		ok_to_hrresult(unsafe {
			(vt::<IMFPresentationDescriptorVT>(self).SelectStream)(self.ptr(), descriptor_index)
		})
	}
}
