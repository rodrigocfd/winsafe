#![allow(non_snake_case)]

use crate::{CLSID, ComVtbl, GUID};
use crate::co;
use crate::ffi::{ole32, Void};

/// [`CoCreateInstance`](https://docs.microsoft.com/en-us/windows/win32/api/combaseapi/nf-combaseapi-cocreateinstance)
/// function.
///
/// Returns an [`IUnknown`](crate::IUnknown)-derived COM interface object.
pub fn CoCreateInstance<V: ComVtbl, I: From<*const *const V>>(
	rclsid: &CLSID,
	pUnkOuter: Option<*const Void>,
	dwClsContext: co::CLSCTX,
) -> I {
	let mut ppv: *const *const V = std::ptr::null();
	unsafe {
		ole32::CoCreateInstance(
			rclsid.as_ref() as *const GUID as *const Void,
			pUnkOuter.unwrap_or(std::ptr::null()),
			dwClsContext.into(),
			V::IID().as_ref() as *const GUID as *const Void,
			&mut ppv
				as *const *const *const V
				as *const *const *const Void,
		);
	}
	I::from(ppv)
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
///
/// Must be called after all COM interfaces have been released.
pub fn CoUninitialize() {
	unsafe { ole32::CoUninitialize() }
}