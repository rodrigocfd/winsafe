#![allow(non_snake_case)]

use crate::ffi_types::{HRES, PVOID};
use crate::ole::decl::{ComPtr, HrResult};
use crate::ole::privs::okfalse_to_hrresult;
use crate::prelude::ShlwapiIPersist;
use crate::vt::IPersistVT;

/// [`IMediaFilter`](crate::IMediaFilter) virtual table.
#[cfg_attr(docsrs, doc(cfg(feature = "dshow")))]
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
/// COM interface over [`IMediaFilterVT`](crate::vt::IMediaFilterVT).
///
/// Automatically calls
/// [`IUnknown::Release`](https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
/// when the object goes out of scope.
#[cfg_attr(docsrs, doc(cfg(feature = "dshow")))]
pub struct IMediaFilter(ComPtr);

impl_iunknown!(IMediaFilter, "56a86899-0ad4-11ce-b03a-0020af0ba770");
impl ShlwapiIPersist for IMediaFilter {}
impl DshowIMediaFilter for IMediaFilter {}

/// [`IMediaFilter`](crate::IMediaFilter) methods methods from `dshow` feature.
#[cfg_attr(docsrs, doc(cfg(feature = "dshow")))]
pub trait DshowIMediaFilter: ShlwapiIPersist {
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
