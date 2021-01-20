//! Win32 COM free functions.

#![allow(non_snake_case)]

use std::ffi::c_void;

use crate::aliases::WinResult;
use crate::co;
use crate::com::{PPVtbl, Vtbl};
use crate::ffi::ole32;
use crate::privs::const_void;
use crate::structs::{CLSID, GUID};

/// [`CoCreateInstance`](https://docs.microsoft.com/en-us/windows/win32/api/combaseapi/nf-combaseapi-cocreateinstance)
/// function.
///
/// Returns an [`IUnknown`](crate::IUnknown)-derived COM object.
///
/// # Examples
///
/// Instantiating an [`ITaskbarList`](crate::shell::ITaskbarList) object:
/// ```rust,ignore
/// use winsafe::{co::CLSCTX, CoCreateInstance, shell};
///
/// let obj: shell::ITaskbarList = CoCreateInstance(
///   &shell::clsid::TaskbarList,
///   None,
///   CLSCTX::INPROC_SERVER,
/// ).unwrap();
/// ```
pub fn CoCreateInstance<VT: Vtbl, RetInterf: From<PPVtbl<VT>>>(
	rclsid: &CLSID,
	pUnkOuter: Option<*mut c_void>,
	dwClsContext: co::CLSCTX) -> WinResult<RetInterf>
{
	let mut ppv: PPVtbl<VT> = std::ptr::null_mut();

	match co::ERROR::from(
		unsafe {
			ole32::CoCreateInstance(
				const_void(rclsid),
				pUnkOuter.unwrap_or(std::ptr::null_mut()),
				dwClsContext.into(),
				VT::IID().as_ref() as *const GUID as *const _,
				&mut ppv
					as *mut PPVtbl<VT>
					as *mut *mut _,
			)
		}
	) {
		co::ERROR::S_OK => Ok(RetInterf::from(ppv)),
		err => Err(err),
	}
}

/// [`CoInitializeEx`](https://docs.microsoft.com/en-us/windows/win32/api/combaseapi/nf-combaseapi-coinitializeex)
/// function. Returns some error codes as success status.
///
/// **Note:** Must be paired with a [`CoUninitialize`](crate::CoUninitialize) call.
///
/// # Examples
///
/// ```rust,ignore
/// use winsafe::{co::COINIT, CoInitializeEx};
///
/// CoInitializeEx(COINIT::MULTITHREADED).unwrap();
/// ```
pub fn CoInitializeEx(dwCoInit: co::COINIT) -> WinResult<co::ERROR> {
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
///
/// Must be called **after** all COM interfaces have been released, otherwise you'll
/// get a segmentation fault error with `STATUS_ACCESS_VIOLATION` code.
pub fn CoUninitialize() {
	unsafe { ole32::CoUninitialize() }
}
