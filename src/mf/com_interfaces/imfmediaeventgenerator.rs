#![allow(non_camel_case_types, non_snake_case)]

use crate::co;
use crate::decl::*;
use crate::kernel::privs::*;
use crate::mf::vts::*;
use crate::ole::privs::*;
use crate::prelude::*;

com_interface! { IMFMediaEventGenerator: "2cd0bd52-bcd5-4b89-b62c-eadc0c031e7d";
	/// [`IMFMediaEventGenerator`](https://learn.microsoft.com/en-us/windows/win32/api/mfobjects/nn-mfobjects-imfmediaeventgenerator)
	/// COM interface.
	///
	/// Automatically calls
	/// [`Release`](https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
}

impl mf_IMFMediaEventGenerator for IMFMediaEventGenerator {}

/// This trait is enabled with the `mf` feature, and provides methods for
/// [`IMFMediaEventGenerator`](crate::IMFMediaEventGenerator).
///
/// Prefer importing this trait through the prelude:
///
/// ```no_run
/// use winsafe::prelude::*;
/// ```
pub trait mf_IMFMediaEventGenerator: ole_IUnknown {
	/// [`IMFMediaEventGenerator::BeginGetEvent`](https://learn.microsoft.com/en-us/windows/win32/api/mfobjects/nf-mfobjects-imfmediaeventgenerator-begingetevent)
	/// method.
	fn BeginGetEvent(
		&self,
		callback: &IMFAsyncCallback,
		state: Option<&impl ole_IUnknown>,
	) -> HrResult<()> {
		HrRet(unsafe {
			(vt::<IMFMediaEventGeneratorVT>(self).BeginGetEvent)(
				self.ptr(),
				callback.ptr(),
				state.map_or(std::ptr::null_mut(), |s| s.ptr()),
			)
		}).to_hrresult()
	}

	/// [`IMFMediaEventGenerator::EndGetEvent`](https://learn.microsoft.com/en-us/windows/win32/api/mfobjects/nf-mfobjects-imfmediaeventgenerator-endgetevent)
	/// method.
	#[must_use]
	fn EndGetEvent(&self, result: &IMFAsyncResult) -> HrResult<IMFMediaEvent> {
		let mut queried = unsafe { IMFMediaEvent::null() };
		HrRet(unsafe {
			(vt::<IMFMediaEventGeneratorVT>(self).EndGetEvent)(
				self.ptr(),
				result.ptr(),
				queried.as_mut(),
			)
		}).to_hrresult()
		.map(|_| queried)
	}

	/// [`IMFMediaEventGenerator::GetEvent`](https://learn.microsoft.com/en-us/windows/win32/api/mfobjects/nf-mfobjects-imfmediaeventgenerator-getevent)
	/// method.
	#[must_use]
	fn GetEvent(&self, flags: Option<co::MF_EVENT_FLAG>) -> HrResult<IMFMediaEvent> {
		let mut queried = unsafe { IMFMediaEvent::null() };
		HrRet(unsafe {
			(vt::<IMFMediaEventGeneratorVT>(self).GetEvent)(
				self.ptr(),
				flags.unwrap_or_default().raw(),
				queried.as_mut(),
			)
		}).to_hrresult()
		.map(|_| queried)
	}

	/// [`IMFMediaEventGenerator::QueueEvent`](https://learn.microsoft.com/en-us/windows/win32/api/mfobjects/nf-mfobjects-imfmediaeventgenerator-queueevent)
	/// method.
	fn QueueEvent(
		&self,
		met: co::ME,
		extended_type: &GUID,
		status: co::HRESULT,
		value: Option<&PropVariant>,
	) -> HrResult<()> {
		HrRet(unsafe {
			(vt::<IMFMediaEventGeneratorVT>(self).QueueEvent)(
				self.ptr(),
				met.raw(),
				pcvoid(extended_type),
				status.raw(),
				match value {
					None => std::ptr::null(),
					Some(v) => pcvoid(&v.to_raw()?),
				},
			)
		}).to_hrresult()
	}
}
