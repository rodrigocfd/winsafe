#![allow(non_camel_case_types, non_snake_case)]

use crate::kernel::ffi_types::{COMPTR, HRES, PCVOID};
use crate::prelude::ole_IUnknown;
use crate::vt::IUnknownVT;

/// [`IMFMediaEventGenerator`](crate::IMFMediaEventGenerator) virtual table.
#[repr(C)]
pub struct IMFMediaEventGeneratorVT {
	pub IUnknownVT: IUnknownVT,
	pub GetEvent: fn(COMPTR, u32, *mut COMPTR) -> HRES,
	pub BeginGetEvent: fn(COMPTR, COMPTR, COMPTR) -> HRES,
	pub EndGetEvent: fn(COMPTR, COMPTR, *mut COMPTR) -> HRES,
	pub QueueEvent: fn(COMPTR, u32, PCVOID, HRES, PCVOID) -> HRES,
}

com_interface! { IMFMediaEventGenerator: "2cd0bd52-bcd5-4b89-b62c-eadc0c031e7d";
	/// [`IMFMediaEventGenerator`](https://learn.microsoft.com/en-us/windows/win32/api/mfobjects/nn-mfobjects-imfmediaeventgenerator)
	/// COM interface over
	/// [`IMFMediaEventGeneratorVT`](crate::vt::IMFMediaEventGeneratorVT).
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
/// ```rust,no_run
/// use winsafe::prelude::*;
/// ```
pub trait mf_IMFMediaEventGenerator: ole_IUnknown {

}
