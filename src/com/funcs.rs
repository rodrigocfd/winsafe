//! Win32 COM free functions.

#![allow(non_snake_case)]

use crate::aliases::WinResult;
use crate::co;
use crate::com::{ComVT, IUnknown, IUnknownVT, PPComVT};
use crate::ffi::ole32;
use crate::privs::hr_to_winresult;
use crate::structs::CLSID;

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
	pUnkOuter: Option<&mut IUnknown>,
	dwClsContext: co::CLSCTX) -> WinResult<RetInterf>
{
	let mut ppv: PPComVT<VT> = std::ptr::null_mut();
	let mut ppvOuter: PPComVT<IUnknownVT> = std::ptr::null_mut();

	hr_to_winresult(
		unsafe {
			ole32::CoCreateInstance(
				rclsid as *const _ as _,
				pUnkOuter.as_ref()
					.map_or(std::ptr::null_mut(), |_| &mut ppvOuter as *mut _ as _),
				dwClsContext.0,
				&VT::IID as *const _ as _,
				&mut ppv as *mut _ as _,
			)
		},
	).map(|_| {
		if let Some(iunkOuter) = pUnkOuter {
			*iunkOuter = IUnknown::from(ppvOuter);
		}
		RetInterf::from(ppv)
	})
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
	let code = co::ERROR(
		unsafe { ole32::CoInitializeEx(std::ptr::null_mut(), dwCoInit.0) } as _
	);
	match code {
		co::ERROR::S_OK
			| co::ERROR::S_FALSE
			| co::ERROR::RPC_E_CHANGED_MODE => Ok(code),
		err => Err(err),
	}
}

/// [`CoTaskMemFree`](https://docs.microsoft.com/en-us/windows/win32/api/combaseapi/nf-combaseapi-cotaskmemfree)
/// function.
pub fn CoTaskMemFree<T>(pv: *mut T) {
	unsafe { ole32::CoTaskMemFree(pv as _) }
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
