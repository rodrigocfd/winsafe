#![allow(non_camel_case_types, non_snake_case)]

use crate::co;
use crate::kernel::ffi_types::{COMPTR, HRES, PCVOID};
use crate::mf::decl::IMFTopology;
use crate::ole::decl::HrResult;
use crate::ole::privs::{ok_to_hrresult, vt};
use crate::prelude::{mf_IMFMediaEventGenerator, mf_IMFTopology, ole_IUnknown};
use crate::vt::IMFMediaEventGeneratorVT;

/// [`IMFMediaSession`](crate::IMFMediaSession) virtual table.
#[repr(C)]
pub struct IMFMediaSessionVT {
	pub IMFMediaEventGeneratorVT: IMFMediaEventGeneratorVT,
	pub SetTopology: fn(COMPTR, u32, COMPTR) -> HRES,
	pub ClearTopologies: fn(COMPTR) -> HRES,
	pub Start: fn(COMPTR, PCVOID, PCVOID) -> HRES,
	pub Pause: fn(COMPTR) -> HRES,
	pub Stop: fn(COMPTR) -> HRES,
	pub Close: fn(COMPTR) -> HRES,
	pub Shutdown: fn(COMPTR) -> HRES,
	pub GetClock: fn(COMPTR, *mut COMPTR) -> HRES,
	pub GetSessionCapabilities: fn(COMPTR, *mut u32) -> HRES,
	pub GetFullTopology: fn(COMPTR, u32, u64, *mut COMPTR) -> HRES,
}

com_interface! { IMFMediaSession: "90377834-21d0-4dee-8214-ba2e3e6c1127";
	/// [`IMFMediaSession`](https://learn.microsoft.com/en-us/windows/win32/api/mfidl/nn-mfidl-imfmediasession)
	/// COM interface over [`IMFMediaSessionVT`](crate::vt::IMFMediaSessionVT).
	///
	/// Automatically calls
	/// [`Release`](https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
	///
	/// # Examples
	///
	/// ```rust,no_run
	/// use winsafe::prelude::*;
	/// use winsafe::{IMFAttributes, MFCreateMediaSession};
	///
	/// let media_session = MFCreateMediaSession(None::<&IMFAttributes>)?;
	/// # Ok::<_, winsafe::co::HRESULT>(())
	/// ```
}

impl mf_IMFMediaEventGenerator for IMFMediaSession {}
impl mf_IMFMediaSession for IMFMediaSession {}

/// This trait is enabled with the `mf` feature, and provides methods for
/// [`IMFMediaSession`](crate::IMFMediaSession).
///
/// Prefer importing this trait through the prelude:
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// ```
pub trait mf_IMFMediaSession: mf_IMFMediaEventGenerator {
	fn_com_noparm! { ClearTopologies: IMFMediaSessionVT;
		/// [`IMFMediaSession::ClearTopologies`](https://learn.microsoft.com/en-us/windows/win32/api/mfidl/nf-mfidl-imfmediasession-cleartopologies)
		/// method.
	}

	fn_com_noparm! { Close: IMFMediaSessionVT;
		/// [`IMFMediaSession::Close`](https://learn.microsoft.com/en-us/windows/win32/api/mfidl/nf-mfidl-imfmediasession-close)
		/// method.
	}

	/// [`IMFMediaSession::GetFullTopology`](https://learn.microsoft.com/en-us/windows/win32/api/mfidl/nf-mfidl-imfmediasession-getfulltopology)
	/// method.
	#[must_use]
	fn GetFullTopology(&self,
		flags: co::MFSESSION_GETFULLTOPOLOGY,
		topo_id: u64,
	) -> HrResult<IMFTopology>
	{
		let mut queried = unsafe { IMFTopology::null() };
		ok_to_hrresult(
			unsafe {
				(vt::<IMFMediaSessionVT>(self).GetFullTopology)(
					self.ptr(),
					flags.raw(),
					topo_id,
					queried.as_mut(),
				)
			},
		).map(|_| queried)
	}

	fn_com_noparm! { Pause: IMFMediaSessionVT;
		/// [`IMFMediaSession::Pause`](https://learn.microsoft.com/en-us/windows/win32/api/mfidl/nf-mfidl-imfmediasession-pause)
		/// method.
	}

	/// [`IMFMediaSession::SetTopology`](https://learn.microsoft.com/en-us/windows/win32/api/mfidl/nf-mfidl-imfmediasession-settopology)
	/// method.
	fn SetTopology(&self,
		flags: co::MFSESSION_SETTOPOLOGY,
		topology: &impl mf_IMFTopology,
	) -> HrResult<()>
	{
		ok_to_hrresult(
			unsafe {
				(vt::<IMFMediaSessionVT>(self).SetTopology)(
					self.ptr(),
					flags.raw(),
					topology.ptr(),
				)
			},
		)
	}

	fn_com_noparm! { Shutdown: IMFMediaSessionVT;
		/// [`IMFMediaSession::Shutdown`](https://learn.microsoft.com/en-us/windows/win32/api/mfidl/nf-mfidl-imfmediasession-shutdown)
		/// method.
	}

	fn_com_noparm! { Stop: IMFMediaSessionVT;
		/// [`IMFMediaSession::Stop`](https://learn.microsoft.com/en-us/windows/win32/api/mfidl/nf-mfidl-imfmediasession-stop)
		/// method.
	}
}