#![allow(non_snake_case)]

use crate::{co, ole};
use crate::ole::decl::{ComPtr, HrResult, IUnknown};
use crate::ole::privs::ok_to_hrresult;
use crate::prelude::ole_IUnknown;

/// [`CoCreateInstance`](https://learn.microsoft.com/en-us/windows/win32/api/combaseapi/nf-combaseapi-cocreateinstance)
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
/// use winsafe::{co, CoCreateInstance, ITaskbarList};
///
/// let obj = CoCreateInstance::<ITaskbarList>(
///     &co::CLSID::TaskbarList,
///     None,
///     co::CLSCTX::INPROC_SERVER,
/// )?;
/// # Ok::<_, co::HRESULT>(())
/// ```
#[cfg_attr(docsrs, doc(cfg(feature = "ole")))]
#[must_use]
pub fn CoCreateInstance<T>(
	clsid: &co::CLSID,
	iunk_outer: Option<&mut IUnknown>,
	cls_context: co::CLSCTX) -> HrResult<T>
	where T: ole_IUnknown,
{
	unsafe {
		let mut ppv = ComPtr::null();
		let mut ppv_outer = ComPtr::null();
		ok_to_hrresult(
			ole::ffi::CoCreateInstance(
				clsid as *const _ as _,
				iunk_outer.as_ref()
					.map_or(std::ptr::null_mut(), |_| &mut ppv_outer as *mut _ as _),
				cls_context.0,
				&T::IID as *const _ as _,
				&mut ppv as *mut _ as _,
			),
		).map(|_| {
			if let Some(iunk_outer) = iunk_outer {
				*iunk_outer = IUnknown::from(ppv_outer); // create outer Unknown if due
			}
			T::from(ppv) // return new Unknown-derived object
		})
	}
}

/// [`CoInitializeEx`](https://learn.microsoft.com/en-us/windows/win32/api/combaseapi/nf-combaseapi-coinitializeex)
/// function, which
/// [initializes](https://learn.microsoft.com/en-us/windows/win32/learnwin32/initializing-the-com-library)
/// the COM library. Returns some error codes as success status.
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
/// CoInitializeEx(
///     co::COINIT::APARTMENTTHREADED
///     | co::COINIT::DISABLE_OLE1DDE)?;
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

/// [`CoLockObjectExternal`](https://learn.microsoft.com/en-us/windows/win32/api/combaseapi/nf-combaseapi-colockobjectexternal)
/// function.
///
/// **Note:** If you lock a COM pointer, `CoLockObjectExternal` must be called
/// again to unlock it, or you'll have a resource leak.
#[cfg_attr(docsrs, doc(cfg(feature = "ole")))]
pub fn CoLockObjectExternal(
	obj: &impl ole_IUnknown,
	lock: bool,
	last_unlock_releases: bool) -> HrResult<()>
{
	ok_to_hrresult(
		unsafe {
			ole::ffi::CoLockObjectExternal(
				obj.ptr().0 as _,
				lock as _,
				last_unlock_releases as _,
			)
		},
	)
}

/// [`CoTaskMemAlloc`](https://learn.microsoft.com/en-us/windows/win32/api/combaseapi/nf-combaseapi-cotaskmemalloc)
/// function.
///
/// # Safety
///
/// This function manually allocates a memory block, which must be freed with
/// [`CoTaskMemFree`](crate::CoTaskMemFree).
#[cfg_attr(docsrs, doc(cfg(feature = "ole")))]
pub unsafe fn CoTaskMemAlloc(cb: usize) -> HrResult<*mut u8> {
	let p = unsafe { ole::ffi::CoTaskMemAlloc(cb) };
	if p.is_null() {
		Err(co::HRESULT::E_OUTOFMEMORY)
	} else {
		Ok(p as _)
	}
}

/// [`CoTaskMemFree`](https://learn.microsoft.com/en-us/windows/win32/api/combaseapi/nf-combaseapi-cotaskmemfree)
/// function.
#[cfg_attr(docsrs, doc(cfg(feature = "ole")))]
pub fn CoTaskMemFree(pv: *mut u8) {
	unsafe { ole::ffi::CoTaskMemFree(pv as _) }
}

/// [`CoTaskMemRealloc`](https://learn.microsoft.com/en-us/windows/win32/api/combaseapi/nf-combaseapi-cotaskmemfree)
/// function.
///
/// # Safety
///
/// This function manually allocates a memory block, which must be freed with
/// [`CoTaskMemFree`](crate::CoTaskMemFree).
#[cfg_attr(docsrs, doc(cfg(feature = "ole")))]
pub unsafe fn CoTaskMemRealloc(pv: *mut u8, cb: usize) -> HrResult<*mut u8> {
	let p = unsafe { ole::ffi::CoTaskMemRealloc(pv as _, cb) };
	if p.is_null() {
		Err(co::HRESULT::E_OUTOFMEMORY)
	} else {
		Ok(p as _)
	}
}

/// [`CoUninitialize`](https://learn.microsoft.com/en-us/windows/win32/api/combaseapi/nf-combaseapi-couninitialize)
/// function.
///
/// **Note:** Must be called **after** all COM interfaces have been released,
/// otherwise you'll get a segmentation fault error with
/// `STATUS_ACCESS_VIOLATION` code.
#[cfg_attr(docsrs, doc(cfg(feature = "ole")))]
pub fn CoUninitialize() {
	unsafe { ole::ffi::CoUninitialize() }
}
