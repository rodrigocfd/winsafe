#![allow(non_snake_case)]

use crate::aliases::WinResult;
use crate::co;
use crate::com::iunknown::{ComInterface, ComPtr, IUnknown};
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
/// use winsafe::prelude::*;
/// use winsafe::{co, CoCreateInstance, shell};
///
/// let obj = CoCreateInstance::<shell::ITaskbarList>(
///     &shell::clsid::TaskbarList,
///     None,
///     co::CLSCTX::INPROC_SERVER,
/// )?;
/// ```
pub fn CoCreateInstance<T: ComInterface>(
	clsid: &CLSID,
	iunk_outer: Option<&mut IUnknown>,
	cls_context: co::CLSCTX) -> WinResult<T>
{
	let mut ppv = ComPtr::null();
	let mut ppv_outer = ComPtr::null();

	hr_to_winresult(
		unsafe {
			ole32::CoCreateInstance(
				clsid as *const _ as _,
				iunk_outer.as_ref()
					.map_or(std::ptr::null_mut(), |_| &mut ppv_outer as *mut _ as _),
				cls_context.0,
				&T::IID as *const _ as _,
				&mut ppv as *mut _ as _,
			)
		},
	).map(|_| {
		if let Some(iunk_outer) = iunk_outer {
			*iunk_outer = IUnknown::from(ppv_outer); // create outer Unknown if due
		}
		T::from(ppv) // return new Unknown-derived object
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
/// use winsafe::prelude::*;
/// use winsafe::{co, CoInitializeEx, CoUninitialize};
///
/// CoInitializeEx(co::COINIT::MULTITHREADED)?;
///
/// // program runs...
///
/// CoUninitialize();
/// ```
pub fn CoInitializeEx(coinit: co::COINIT) -> WinResult<co::ERROR> {
	let code = co::ERROR(
		unsafe { ole32::CoInitializeEx(std::ptr::null_mut(), coinit.0) } as _
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
