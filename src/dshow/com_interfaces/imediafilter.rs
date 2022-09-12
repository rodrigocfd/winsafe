#![allow(non_camel_case_types, non_snake_case)]

use crate::kernel::ffi_types::{HRES, PVOID};
use crate::ole::decl::{ComPtr, HrResult};
use crate::ole::privs::okfalse_to_hrresult;
use crate::prelude::ole_IPersist;
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

com_interface! { IMediaFilter: "dshow";
	"56a86899-0ad4-11ce-b03a-0020af0ba770";
	/// [`IMediaFilter`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nn-strmif-imediafilter)
	/// COM interface over [`IMediaFilterVT`](crate::vt::IMediaFilterVT).
	///
	/// Automatically calls
	/// [`IUnknown::Release`](https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
}

impl ole_IPersist for IMediaFilter {}
impl dshow_IMediaFilter for IMediaFilter {}

/// This trait is enabled with the `dshow` feature, and provides methods for
/// [`IMediaFilter`](crate::IMediaFilter).
///
/// Prefer importing this trait through the prelude:
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// ```
#[cfg_attr(docsrs, doc(cfg(feature = "dshow")))]
pub trait dshow_IMediaFilter: ole_IPersist {
	/// [`IMediaFilter::Pause`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-imediafilter-pause)
	/// method.
	fn Pause(&self) -> HrResult<bool> {
		unsafe {
			let vt = self.vt_ref::<IMediaFilterVT>();
			okfalse_to_hrresult((vt.Pause)(self.ptr()))
		}
	}

	/// [`IMediaFilter::Run`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-imediafilter-run)
	/// method.
	fn Run(&self, start: i64) -> HrResult<bool> {
		unsafe {
			let vt = self.vt_ref::<IMediaFilterVT>();
			okfalse_to_hrresult((vt.Run)(self.ptr(), start))
		}
	}

	/// [`IMediaFilter::Stop`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-imediafilter-stop)
	/// method.
	fn Stop(&self) -> HrResult<bool> {
		unsafe {
			let vt = self.vt_ref::<IMediaFilterVT>();
			okfalse_to_hrresult((vt.Stop)(self.ptr()))
		}
	}
}
