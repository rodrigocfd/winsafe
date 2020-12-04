#![allow(non_snake_case)]

use crate::{CLSID, GUID, IID};
use crate::co;
use crate::ffi::{ole32, Void};

/// [`CoCreateInstance`](https://docs.microsoft.com/en-us/windows/win32/api/combaseapi/nf-combaseapi-cocreateinstance)
/// function.
///
/// Returns a pointer to a pointer to an
/// [`IUnknown`](https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nn-unknwn-iunknown)
/// COM virtual table.
pub fn CoCreateInstance<T>(rclsid: &CLSID, pUnkOuter: *mut Void,
	dwClsContext: co::CLSCTX, riid: &IID) -> *mut *mut T
{
	let mut ppv: *mut *mut T = std::ptr::null_mut();
	unsafe {
		ole32::CoCreateInstance(rclsid.as_ref() as *const GUID as *mut Void, pUnkOuter,
			dwClsContext.into(), riid.as_ref() as *const GUID as *mut Void,
			&mut ppv
				as *mut *mut *mut T
				as *mut *mut *mut Void,
		);
	}
	ppv
}

/// [`CoInitializeEx`](https://docs.microsoft.com/en-us/windows/win32/api/combaseapi/nf-combaseapi-coinitializeex)
/// function.
///
/// Must be paired with a
/// [`CoUninitialize`](https://docs.microsoft.com/en-us/windows/win32/api/combaseapi/nf-combaseapi-couninitialize)
/// call.
pub fn CoInitializeEx(dwCoInit: co::COINIT) -> Result<co::ERROR, co::ERROR> {
	let err = co::ERROR::from(
		unsafe { ole32::CoInitializeEx(std::ptr::null_mut(), dwCoInit.into()) }
	);
	match err {
		co::ERROR::S_OK
			| co::ERROR::S_FALSE
			| co::ERROR::RPC_E_CHANGED_MODE => Ok(err),
		err => Err(err),
	}
}

/// [`CoUninitialize`](https://docs.microsoft.com/en-us/windows/win32/api/combaseapi/nf-combaseapi-couninitialize)
/// function.
pub fn CoUninitialize() {
	unsafe { ole32::CoUninitialize() }
}