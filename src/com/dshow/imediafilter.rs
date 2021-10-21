#![allow(non_snake_case)]

use crate::aliases::WinResult;
use crate::com::iunknown::ComPtr;
use crate::com::idl::ipersist::{IPersistT, IPersistVT};
use crate::ffi::{HRESULT, PVOID};
use crate::privs::hr_to_winresult_bool;

/// [`IMediaFilter`](crate::dshow::IMediaFilter) virtual table.
pub struct IMediaFilterVT {
	pub IPersistVT: IPersistVT,
	pub Stop: fn(ComPtr) -> HRESULT,
	pub Pause: fn(ComPtr) -> HRESULT,
   pub Run: fn(ComPtr, i64) -> HRESULT,
	pub GetState: fn(ComPtr, i64, PVOID, *mut u32) -> HRESULT,
	pub SetSyncSource: fn(ComPtr, ComPtr) -> HRESULT,
	pub GetSyncSource: fn(ComPtr, *mut ComPtr) -> HRESULT,
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
	fn Pause(&self) -> WinResult<bool> {
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IMediaFilterVT);
			hr_to_winresult_bool((vt.Pause)(self.ptr()))
		}
	}

	/// [`IMediaFilter::Run`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-imediafilter-run)
	/// method.
	fn Run(&self, start: i64) -> WinResult<bool> {
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IMediaFilterVT);
			hr_to_winresult_bool((vt.Run)(self.ptr(), start))
		}
	}

	/// [`IMediaFilter::Stop`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-imediafilter-stop)
	/// method.
	fn Stop(&self) -> WinResult<bool> {
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IMediaFilterVT);
			hr_to_winresult_bool((vt.Stop)(self.ptr()))
		}
	}
}
