#![allow(non_camel_case_types, non_snake_case)]

use crate::decl::*;
use crate::dxgi::vts::*;
use crate::kernel::privs::*;
use crate::ole::privs::*;
use crate::prelude::*;

com_interface! { IDXGIKeyedMutex: "9d8e1289-d7b3-465f-8126-250e349af85d";
	/// [`IDXGIKeyedMutex`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi/nn-dxgi-idxgikeyedmutex)
	/// COM interface.
	///
	/// Automatically calls
	/// [`Release`](https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
}

impl dxgi_IDXGIObject for IDXGIKeyedMutex {}
impl dxgi_IDXGIDeviceSubObject for IDXGIKeyedMutex {}
impl dxgi_IDXGIKeyedMutex for IDXGIKeyedMutex {}

/// This trait is enabled with the `dxgi` feature, and provides methods for
/// [`IDXGIKeyedMutex`](crate::IDXGIKeyedMutex).
///
/// Prefer importing this trait through the prelude:
///
/// ```no_run
/// use winsafe::prelude::*;
/// ```
pub trait dxgi_IDXGIKeyedMutex: dxgi_IDXGIDeviceSubObject {
	/// [`IDXGIKeyedMutex::AcquireSync`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi/nf-dxgi-idxgikeyedmutex-acquiresync)
	/// method.
	fn AcquireSync(&self, key: u64, dw_milliseconds: Option<u32>) -> HrResult<()> {
		ok_to_hrresult(unsafe {
			(vt::<IDXGIKeyedMutexVT>(self).AcquireSync)(
				self.ptr(),
				key,
				dw_milliseconds.unwrap_or(INFINITE),
			)
		})
	}

	/// [`IDXGIKeyedMutex::ReleaseSync`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi/nf-dxgi-idxgikeyedmutex-releasesync)
	/// method.
	fn ReleaseSync(&self, key: u64) -> HrResult<()> {
		ok_to_hrresult(unsafe { (vt::<IDXGIKeyedMutexVT>(self).ReleaseSync)(self.ptr(), key) })
	}
}
