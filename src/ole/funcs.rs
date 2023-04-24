#![allow(non_snake_case)]

use crate::{co, ole};
use crate::kernel::decl::WString;
use crate::ole::decl::{
	ComPtr, COSERVERINFO, HrResult, IMoniker, IUnknown, MULTI_QI,
};
use crate::ole::guard::CoUninitializeGuard;
use crate::ole::privs::ok_to_hrresult;
use crate::prelude::ole_IUnknown;

/// [`CLSIDFromProgID`](https://learn.microsoft.com/en-us/windows/win32/api/combaseapi/nf-combaseapi-clsidfromprogid)
/// function.
#[must_use]
pub fn CLSIDFromProgID(prog_id: &str) -> HrResult<co::CLSID> {
	let mut clsid = co::CLSID::default();
	ok_to_hrresult(
		unsafe {
			ole::ffi::CLSIDFromProgID(
				WString::from_str(prog_id).as_ptr(),
				&mut clsid as *mut _ as _,
			)
		},
	).map(|_| clsid)
}

/// [`CLSIDFromProgIDEx`](https://learn.microsoft.com/en-us/windows/win32/api/combaseapi/nf-combaseapi-clsidfromprogidex)
/// function.
#[must_use]
pub fn CLSIDFromProgIDEx(prog_id: &str) -> HrResult<co::CLSID> {
	let mut clsid = co::CLSID::default();
	ok_to_hrresult(
		unsafe {
			ole::ffi::CLSIDFromProgIDEx(
				WString::from_str(prog_id).as_ptr(),
				&mut clsid as *mut _ as _,
			)
		},
	).map(|_| clsid)
}

/// [`CLSIDFromString`](https://learn.microsoft.com/en-us/windows/win32/api/combaseapi/nf-combaseapi-clsidfromstring)
/// function.
#[must_use]
pub fn CLSIDFromString(prog_id: &str) -> HrResult<co::CLSID> {
	let mut clsid = co::CLSID::default();
	ok_to_hrresult(
		unsafe {
			ole::ffi::CLSIDFromString(
				WString::from_str(prog_id).as_ptr(),
				&mut clsid as *mut _ as _,
			)
		},
	).map(|_| clsid)
}

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
#[must_use]
pub fn CoCreateInstance<T>(
	clsid: &co::CLSID,
	iunk_outer: Option<&mut IUnknown>,
	cls_context: co::CLSCTX,
) -> HrResult<T>
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

/// [`CoCreateInstanceEx`](https://learn.microsoft.com/en-us/windows/win32/api/combaseapi/nf-combaseapi-cocreateinstanceex)
/// function.
pub fn CoCreateInstanceEx(
	clsid: &co::CLSID,
	iunk_outer: Option<&mut IUnknown>,
	cls_context: co::CLSCTX,
	server_info: Option<&COSERVERINFO>,
	results: &mut [MULTI_QI],
) -> HrResult<()>
{
	unsafe {
		let mut ppv_outer = ComPtr::null();
		ok_to_hrresult(
			ole::ffi::CoCreateInstanceEx(
				clsid as *const _ as _,
				iunk_outer.as_ref()
					.map_or(std::ptr::null_mut(), |_| &mut ppv_outer as *mut _ as _),
				cls_context.0,
				server_info.map_or(std::ptr::null(), |si| si as *const _ as _),
				results.len() as _,
				results.as_mut_ptr() as _,
			),
		).map(|_| {
			if let Some(iunk_outer) = iunk_outer {
				*iunk_outer = IUnknown::from(ppv_outer); // create outer Unknown if due
			}
		})
	}
}

/// [`CoInitializeEx`](https://learn.microsoft.com/en-us/windows/win32/api/combaseapi/nf-combaseapi-coinitializeex)
/// function, which
/// [initializes](https://learn.microsoft.com/en-us/windows/win32/learnwin32/initializing-the-com-library)
/// the COM library. When succeeding, returns an informational error code.
///
/// In the original C implementation, you must call
/// [`CoUninitialize`](https://learn.microsoft.com/en-us/windows/win32/api/combaseapi/nf-combaseapi-couninitialize)
/// as a cleanup operation.
///
/// Here, the cleanup is performed automatically, because `CoInitializeEx`
/// returns a [`CoUninitializeGuard`](crate::guard::CoUninitializeGuard), which
/// automatically calls `CoUninitialize` when the guard goes out of scope. You
/// must, however, keep the guard alive, otherwise the cleanup will be performed
/// right away.
///
/// # Examples
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// use winsafe::{co, CoInitializeEx};
///
/// let _com_lib = CoInitializeEx( // keep guard alive
///     co::COINIT::APARTMENTTHREADED
///     | co::COINIT::DISABLE_OLE1DDE,
/// )?;
///
/// // program runs...
/// # Ok::<_, co::HRESULT>(())
/// ```
#[must_use]
pub fn CoInitializeEx(coinit: co::COINIT) -> HrResult<CoUninitializeGuard> {
	unsafe {
		let hr = co::HRESULT(
			ole::ffi::CoInitializeEx(std::ptr::null_mut(), coinit.0),
		);
		match hr {
			co::HRESULT::S_OK
			| co::HRESULT::S_FALSE
			| co::HRESULT::RPC_E_CHANGED_MODE => Ok(CoUninitializeGuard::new(hr)),
			hr => Err(hr),
		}
	}
}

/// [`CoLockObjectExternal`](https://learn.microsoft.com/en-us/windows/win32/api/combaseapi/nf-combaseapi-colockobjectexternal)
/// function.
///
/// **Note:** If you lock a COM pointer, `CoLockObjectExternal` must be called
/// again to unlock it, or you'll have a resource leak.
pub fn CoLockObjectExternal(
	obj: &impl ole_IUnknown,
	lock: bool,
	last_unlock_releases: bool,
) -> HrResult<()>
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
#[must_use]
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
#[must_use]
pub unsafe fn CoTaskMemRealloc(pv: *mut u8, cb: usize) -> HrResult<*mut u8> {
	let p = unsafe { ole::ffi::CoTaskMemRealloc(pv as _, cb) };
	if p.is_null() {
		Err(co::HRESULT::E_OUTOFMEMORY)
	} else {
		Ok(p as _)
	}
}

/// [`CreateClassMoniker`](https://learn.microsoft.com/en-us/windows/win32/api/objbase/nf-objbase-createclassmoniker)
/// function.
#[must_use]
pub fn CreateClassMoniker(clsid: &co::CLSID) -> HrResult<IMoniker> {
	unsafe {
		let mut ppv = ComPtr::null();
		ok_to_hrresult(
			ole::ffi::CreateClassMoniker(
				clsid as *const _ as _,
				&mut ppv as *mut _ as _,
			),
		).map(|_| IMoniker::from(ppv))
	}
}

/// [`CreateFileMoniker`](https://learn.microsoft.com/en-us/windows/win32/api/objbase/nf-objbase-createfilemoniker)
/// function.
#[must_use]
pub fn CreateFileMoniker(path_name: &str) -> HrResult<IMoniker> {
	unsafe {
		let mut ppv = ComPtr::null();
		ok_to_hrresult(
			ole::ffi::CreateFileMoniker(
				WString::from_str(path_name).as_ptr(),
				&mut ppv as *mut _ as _,
			),
		).map(|_| IMoniker::from(ppv))
	}
}

/// [`CreateItemMoniker`](https://learn.microsoft.com/en-us/windows/win32/api/objbase/nf-objbase-createitemmoniker)
/// function.
#[must_use]
pub fn CreateItemMoniker(delim: &str, item: &str) -> HrResult<IMoniker> {
	unsafe {
		let mut ppv = ComPtr::null();
		ok_to_hrresult(
			ole::ffi::CreateItemMoniker(
				WString::from_str(delim).as_ptr(),
				WString::from_str(item).as_ptr(),
				&mut ppv as *mut _ as _,
			),
		).map(|_| IMoniker::from(ppv))
	}
}

/// [`CreateObjrefMoniker`](https://learn.microsoft.com/en-us/windows/win32/api/objbase/nf-objbase-createobjrefmoniker)
/// function.
#[must_use]
pub fn CreateObjrefMoniker(unk: &impl ole_IUnknown) -> HrResult<IMoniker> {
	unsafe {
		let mut ppv = ComPtr::null();
		ok_to_hrresult(
			ole::ffi::CreateObjrefMoniker(
				unk.ptr().0 as _,
				&mut ppv as *mut _ as _,
			),
		).map(|_| IMoniker::from(ppv))
	}
}

/// [`CreatePointerMoniker`](https://learn.microsoft.com/en-us/windows/win32/api/objbase/nf-objbase-createpointermoniker)
/// function.
#[must_use]
pub fn CreatePointerMoniker(unk: &impl ole_IUnknown) -> HrResult<IMoniker> {
	unsafe {
		let mut ppv = ComPtr::null();
		ok_to_hrresult(
			ole::ffi::CreatePointerMoniker(
				unk.ptr().0 as _,
				&mut ppv as *mut _ as _,
			),
		).map(|_| IMoniker::from(ppv))
	}
}

/// [`StringFromCLSID`](https://learn.microsoft.com/en-us/windows/win32/api/combaseapi/nf-combaseapi-stringfromclsid)
/// function.
#[must_use]
pub fn StringFromCLSID(clsid: &co::CLSID) -> HrResult<String> {
	let mut pstr = std::ptr::null_mut::<u16>();
	ok_to_hrresult(
		unsafe { ole::ffi::StringFromCLSID(clsid as *const _ as _, &mut pstr) },
	).map(|_| {
		let name = WString::from_wchars_nullt(pstr);
		CoTaskMemFree(pstr as _);
		name.to_string()
	})
}
