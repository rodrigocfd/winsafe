#![allow(non_snake_case)]

use crate::aliases::HrResult;
use crate::com::iunknown::ComPtr;
use crate::com::idl::ipersist::{IPersistT, IPersistVT};
use crate::ffi::{HRES, PVOID};
use crate::privs::okfalse_to_hrresult;

/// [`IMediaFilter`](crate::dshow::IMediaFilter) virtual table.
#[repr(C)]
pub struct IMediaFilterVT {
	pub IPersistVT: IPersistVT,
	pub Stop: fn(ComPtr) -> HRES,
	pub Pause: fn(ComPtr) -> HRES,
   pub Run: fn(ComPtr, i64) -> HRES,
	pub GetState: fn(ComPtr, i64, PVOID, *mut u32) -> HRES,
	pub SetSyncSource: fn(ComPtr, ComPtr) -> HRES,
	pub GetSyncSource: fn(ComPtr, *mut ComPtr) -> HRES,
}

/// [`IMediaFilter`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nn-strmif-imediafilter)
/// COM interface over [`IMediaFilterVT`](crate::dshow::vt::IMediaFilterVT).
///
/// Automatically calls
/// [`IUnknown::Release`](https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
/// when the object goes out of scope.
pub struct IMediaFilter(ComPtr);

impl_iunknown!(IMediaFilter, 0x56a86899, 0x0ad4, 0x11ce, 0xb03a, 0x0020af0ba770);
impl IPersistT for IMediaFilter {}
impl IMediaFilterT for IMediaFilter {}

/// Exposes the [`IMediaFilter`](crate::dshow::IMediaFilter) methods.
pub trait IMediaFilterT: IPersistT {
	/// [`IMediaFilter::Pause`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-imediafilter-pause)
	/// method.
	fn Pause(&self) -> HrResult<bool> {
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IMediaFilterVT);
			okfalse_to_hrresult((vt.Pause)(self.ptr()))
		}
	}

	/// [`IMediaFilter::Run`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-imediafilter-run)
	/// method.
	fn Run(&self, start: i64) -> HrResult<bool> {
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IMediaFilterVT);
			okfalse_to_hrresult((vt.Run)(self.ptr(), start))
		}
	}

	/// [`IMediaFilter::Stop`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-imediafilter-stop)
	/// method.
	fn Stop(&self) -> HrResult<bool> {
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IMediaFilterVT);
			okfalse_to_hrresult((vt.Stop)(self.ptr()))
		}
	}
}
