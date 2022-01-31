#![allow(non_snake_case)]

use crate::{co, ole};
use crate::ole::decl::{CLSID, ComPtr, HrResult, IUnknown};
use crate::ole::privs::ok_to_hrresult;
use crate::prelude::ComInterface;

/// [`CoCreateInstance`](https://docs.microsoft.com/en-us/windows/win32/api/combaseapi/nf-combaseapi-cocreateinstance)
/// function.
///
/// Returns an [`IUnknown`](crate::IUnknown)-derived COM object.
///
/// # Examples
///
/// Instantiating an [`ITaskbarList`](crate::ITaskbarList) object:
///
/// ```rust,ignore
/// use winsafe::prelude::*;
/// use winsafe::{CLSID, co, CoCreateInstance, ITaskbarList};
///
/// let obj = CoCreateInstance::<ITaskbarList>(
///     &CLSID::TaskbarList,
///     None,
///     co::CLSCTX::INPROC_SERVER,
/// )?;
/// # Ok::<_, co::HRESULT>(())
/// ```
#[cfg_attr(docsrs, doc(cfg(feature = "ole")))]
pub fn CoCreateInstance<T>(
	clsid: &CLSID,
	iunk_outer: Option<&mut IUnknown>,
	cls_context: co::CLSCTX) -> HrResult<T>
	where T: ComInterface,
{
	let mut ppv = ComPtr::null();
	let mut ppv_outer = ComPtr::null();

	ok_to_hrresult(
		unsafe {
			ole::ffi::CoCreateInstance(
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
/// ```rust,no_run
/// use winsafe::prelude::*;
/// use winsafe::{co, CoInitializeEx, CoUninitialize};
///
/// CoInitializeEx(co::COINIT::MULTITHREADED)?;
///
/// // program runs...
///
/// CoUninitialize();
/// # Ok::<_, co::HRESULT>(())
/// ```
#[cfg_attr(docsrs, doc(cfg(feature = "ole")))]
pub fn CoInitializeEx(coinit: co::COINIT) -> HrResult<co::HRESULT> {
	let code = co::HRESULT(
		unsafe { ole::ffi::CoInitializeEx(std::ptr::null_mut(), coinit.0) },
	);
	match code {
		co::HRESULT::S_OK
		| co::HRESULT::S_FALSE
		| co::HRESULT::RPC_E_CHANGED_MODE => Ok(code),
		hr => Err(hr),
	}
}

/// [`CoTaskMemFree`](https://docs.microsoft.com/en-us/windows/win32/api/combaseapi/nf-combaseapi-cotaskmemfree)
/// function.
#[cfg_attr(docsrs, doc(cfg(feature = "ole")))]
pub fn CoTaskMemFree<T>(pv: *mut T) {
	unsafe { ole::ffi::CoTaskMemFree(pv as _) }
}

/// [`CoUninitialize`](https://docs.microsoft.com/en-us/windows/win32/api/combaseapi/nf-combaseapi-couninitialize)
/// function.
///
/// **Note:** Must be called **after** all COM interfaces have been released,
/// otherwise you'll get a segmentation fault error with
/// `STATUS_ACCESS_VIOLATION` code.
#[cfg_attr(docsrs, doc(cfg(feature = "ole")))]
pub fn CoUninitialize() {
	unsafe { ole::ffi::CoUninitialize() }
}
