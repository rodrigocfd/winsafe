#![allow(non_camel_case_types, non_snake_case)]

use crate::co;
use crate::decl::*;
use crate::kernel::privs::*;
use crate::mf::vts::*;
use crate::ole::privs::*;
use crate::prelude::*;

com_interface! { IMFMediaSource: "279a808d-aec7-40c8-9c6b-a6b492c78a66";
	/// [`IMFMediaSource`](https://learn.microsoft.com/en-us/windows/win32/api/mfidl/nn-mfidl-imfmediasource)
	/// COM interface.
	///
	/// Automatically calls
	/// [`Release`](https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
}

impl mf_IMFMediaEventGenerator for IMFMediaSource {}
impl mf_IMFMediaSource for IMFMediaSource {}

/// This trait is enabled with the `mf` feature, and provides methods for
/// [`IMFMediaSource`](crate::IMFMediaSource).
///
/// Prefer importing this trait through the prelude:
///
/// ```no_run
/// use winsafe::prelude::*;
/// ```
pub trait mf_IMFMediaSource: mf_IMFMediaEventGenerator {
	/// [`IMFMediaSource::GetCharacteristics`](https://learn.microsoft.com/en-us/windows/win32/api/mfidl/nf-mfidl-imfmediasource-getcharacteristics)
	/// method.
	#[must_use]
	fn GetCharacteristics(&self) -> HrResult<co::MFMEDIASOURCE> {
		let mut characteristics = co::MFMEDIASOURCE::default();
		HrRet(unsafe {
			(vt::<IMFMediaSourceVT>(self).GetCharacteristics)(self.ptr(), characteristics.as_mut())
		}).to_hrresult()
		.map(|_| characteristics)
	}

	fn_com_interface_get! { CreatePresentationDescriptor: IMFMediaSourceVT => IMFPresentationDescriptor;
		/// [`IMFMediaSource::CreatePresentationDescriptor`](https://learn.microsoft.com/en-us/windows/win32/api/mfidl/nf-mfidl-imfmediasource-createpresentationdescriptor)
		/// method.
	}

	fn_com_noparm! { Pause: IMFMediaSourceVT;
		/// [`IMFMediaSource::Pause`](https://learn.microsoft.com/en-us/windows/win32/api/mfidl/nf-mfidl-imfmediasource-pause)
		/// method.
	}

	fn_com_noparm! { Shutdown: IMFMediaSourceVT;
		/// [`IMFMediaSource::Shutdown`](https://learn.microsoft.com/en-us/windows/win32/api/mfidl/nf-mfidl-imfmediasource-shutdown)
		/// method.
	}

	/// [`IMFMediaSource::Start`](https://learn.microsoft.com/en-us/windows/win32/api/mfidl/nf-mfidl-imfmediasource-start)
	/// method.
	fn Start(
		&self,
		presentation_descriptor: IMFPresentationDescriptor,
		time_format: Option<&GUID>,
		start_position: &PropVariant,
	) -> HrResult<()> {
		HrRet(unsafe {
			(vt::<IMFMediaSourceVT>(self).Start)(
				self.ptr(),
				presentation_descriptor.ptr(),
				pcvoid(time_format.unwrap_or(&GUID::default())),
				pcvoid(&start_position.to_raw()?),
			)
		}).to_hrresult()
	}

	fn_com_noparm! { Stop: IMFMediaSourceVT;
		/// [`IMFMediaSource::Stop`](https://learn.microsoft.com/en-us/windows/win32/api/mfidl/nf-mfidl-imfmediasource-stop)
		/// method.
	}
}
