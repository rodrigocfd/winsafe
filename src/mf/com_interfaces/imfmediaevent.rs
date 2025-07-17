#![allow(non_camel_case_types, non_snake_case)]

use crate::co;
use crate::decl::*;
use crate::kernel::privs::*;
use crate::mf::vts::*;
use crate::ole::privs::*;
use crate::prelude::*;

com_interface! { IMFMediaEvent: "df598932-f10c-4e39-bba2-c308f101daa3";
	/// [`IMFMediaEvent`](https://learn.microsoft.com/en-us/windows/win32/api/mfobjects/nn-mfobjects-imfasyncresult)
	/// COM interface.
	///
	/// Automatically calls
	/// [`Release`](https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
}

impl mf_IMFMediaEvent for IMFMediaEvent {}
impl mf_IMFAttributes for IMFMediaEvent {}

/// This trait is enabled with the `mf` feature, and provides methods for
/// [`IMFMediaEvent`](crate::IMFMediaEvent).
///
/// Prefer importing this trait through the prelude:
///
/// ```no_run
/// use winsafe::prelude::*;
/// ```
pub trait mf_IMFMediaEvent: mf_IMFAttributes {
	/// [`IMFMediaEvent::GetExtendedType`](https://learn.microsoft.com/en-us/windows/win32/api/mfobjects/nf-mfobjects-imfmediaevent-getextendedtype)
	/// method.
	#[must_use]
	fn GetExtendedType(&self) -> HrResult<GUID> {
		let mut ex_ty = GUID::default();
		HrRet(unsafe {
			(vt::<IMFMediaEventVT>(self).GetExtendedType)(self.ptr(), pvoid(&mut ex_ty))
		}).to_hrresult()
		.map(|_| ex_ty)
	}

	/// [`IMFMediaEvent::GetStatus`](https://learn.microsoft.com/en-us/windows/win32/api/mfobjects/nf-mfobjects-imfmediaevent-getstatus)
	/// method.
	#[must_use]
	fn GetStatus(&self) -> HrResult<co::HRESULT> {
		let mut status = co::HRESULT::default();
		HrRet(unsafe {
			(vt::<IMFMediaEventVT>(self).GetStatus)(self.ptr(), status.as_mut())
		}).to_hrresult()
		.map(|_| status)
	}

	/// [`IMFMediaEvent::GetType`](https://learn.microsoft.com/en-us/windows/win32/api/mfobjects/nf-mfobjects-imfmediaevent-gettype)
	/// method.
	#[must_use]
	fn GetType(&self) -> HrResult<co::ME> {
		let mut met = co::ME::default();
		HrRet(unsafe { (vt::<IMFMediaEventVT>(self).GetType)(self.ptr(), met.as_mut()) })
		.to_hrresult()
			.map(|_| met)
	}

	/// [`IMFMediaEvent::GetValue`](https://learn.microsoft.com/en-us/windows/win32/api/mfobjects/nf-mfobjects-imfmediaevent-getvalue)
	/// method.
	#[must_use]
	fn GetValue(&self) -> HrResult<PropVariant> {
		let mut value = PROPVARIANT::default();
		HrRet(unsafe {
			(vt::<IMFMediaEventVT>(self).GetValue)(self.ptr(), pvoid(&mut value))
		}).to_hrresult()?;
		PropVariant::from_raw(&value)
	}
}
