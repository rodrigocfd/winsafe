//! Win32 COM free functions.

#![allow(non_snake_case)]

use std::ffi::c_void;

use crate::aliases::WinResult;
use crate::co;
use crate::com::{ComVT, PPComVT};
use crate::ffi::ole32;
use crate::structs::{CLSID, GUID};

/// Converts a native `HRESULT` to `WinResult`, `S_OK` yielding `Ok` and
/// anything else yielding `Err`.
pub(crate) fn hr_to_winresult(hresult: i32) -> WinResult<()> {
	match co::ERROR(hresult as u32) {
		co::ERROR::S_OK => Ok(()),
		err => Err(err),
	}
}

/// Converts a native `HRESULT` to `WinResult`, `S_OK` yielding `Ok(true)`,
/// `S_FALSE` yielding `Ok(false)` and anything else yielding `Err`.
pub(crate) fn hr_to_winresult_bool(hresult: i32) -> WinResult<bool> {
	match co::ERROR(hresult as u32) {
		co::ERROR::S_OK => Ok(true),
		co::ERROR::S_FALSE => Ok(false),
		err => Err(err),
	}
}

/// [`CoCreateInstance`](https://docs.microsoft.com/en-us/windows/win32/api/combaseapi/nf-combaseapi-cocreateinstance)
/// function.
///
/// Returns an [`IUnknown`](crate::IUnknown)-derived COM object.
///
/// # Examples
///
/// Instantiating an [`ITaskbarList`](crate::shell::ITaskbarList) object:
///
/// ```rust,ignore
/// use winsafe::{co, CoCreateInstance, shell};
///
/// let obj: shell::ITaskbarList = CoCreateInstance(
///     &shell::clsid::TaskbarList,
///     None,
///     co::CLSCTX::INPROC_SERVER,
/// ).unwrap();
/// ```
pub fn CoCreateInstance<VT: ComVT, RetInterf: From<PPComVT<VT>>>(
	rclsid: &CLSID,
	pUnkOuter: Option<*mut c_void>,
	dwClsContext: co::CLSCTX) -> WinResult<RetInterf>
{
	let mut ppv: PPComVT<VT> = std::ptr::null_mut();

	match co::ERROR(
		unsafe {
			ole32::CoCreateInstance(
				rclsid as *const _ as *const _,
				pUnkOuter.unwrap_or(std::ptr::null_mut()),
				dwClsContext.0,
				VT::IID().as_ref() as *const GUID as *const _,
				&mut ppv as *mut _ as *mut _,
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
/// **Note:** Must be paired with a [`CoUninitialize`](crate::CoUninitialize)
/// call.
///
/// # Examples
///
/// ```rust,ignore
/// use winsafe::{co, CoInitializeEx, CoUninitialize};
///
/// CoInitializeEx(co::COINIT::MULTITHREADED).unwrap();
///
/// // program runs...
///
/// CoUninitialize().
/// ```
pub fn CoInitializeEx(dwCoInit: co::COINIT) -> WinResult<co::ERROR> {
	let err = co::ERROR(
		unsafe { ole32::CoInitializeEx(std::ptr::null_mut(), dwCoInit.0) }
	);
	match err {
		co::ERROR::S_OK
			| co::ERROR::S_FALSE
			| co::ERROR::RPC_E_CHANGED_MODE => Ok(err),
		err => Err(err),
	}
}

/// [`CoTaskMemFree`](https://docs.microsoft.com/en-us/windows/win32/api/combaseapi/nf-combaseapi-cotaskmemfree)
/// function.
pub fn CoTaskMemFree<T>(pv: *mut T) {
	unsafe { ole32::CoTaskMemFree(pv as *mut _) }
}

/// [`CoUninitialize`](https://docs.microsoft.com/en-us/windows/win32/api/combaseapi/nf-combaseapi-couninitialize)
/// function.
///
/// **Note:** Must be called **after** all COM interfaces have been released,
/// otherwise you'll get a segmentation fault error with
/// `STATUS_ACCESS_VIOLATION` code.
pub fn CoUninitialize() {
	unsafe { ole32::CoUninitialize() }
}
