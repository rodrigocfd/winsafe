#![allow(non_snake_case)]

use crate::{CLSID, GUID, IID};
use crate::co;
use crate::ffi::{ole32, Void};

/// [`CoCreateInstance`](https://docs.microsoft.com/en-us/windows/win32/api/combaseapi/nf-combaseapi-cocreateinstance)
/// function.
///
/// Returns a pointer to a pointer to an [`IUnknown`](crate::com::IUnknown) COM
/// virtual table.
pub fn CoCreateInstance<T>(rclsid: &CLSID, pUnkOuter: Option<*const Void>,
	dwClsContext: co::CLSCTX, riid: &IID) -> *const *const T
{
	let mut ppv: *const *const T = std::ptr::null();
	unsafe {
		ole32::CoCreateInstance(rclsid.as_ref() as *const GUID as *const Void,
			pUnkOuter.unwrap_or(std::ptr::null()),
			dwClsContext.into(), riid.as_ref() as *const GUID as *const Void,
			&mut ppv
				as *const *const *const T
				as *const *const *const Void,
		);
	}
	ppv
}

/// [`CoInitializeEx`](https://docs.microsoft.com/en-us/windows/win32/api/combaseapi/nf-combaseapi-coinitializeex)
/// function.
///
/// Must be paired with a [`CoUninitialize`](crate::CoUninitialize) call.
pub fn CoInitializeEx(dwCoInit: co::COINIT) -> Result<co::ERROR, co::ERROR> {
	let err = co::ERROR::from(
		unsafe { ole32::CoInitializeEx(std::ptr::null(), dwCoInit.into()) }
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