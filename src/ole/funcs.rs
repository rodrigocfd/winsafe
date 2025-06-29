#![allow(non_snake_case)]

use crate::co;
use crate::decl::*;
use crate::guard::*;
use crate::kernel::privs::*;
use crate::ole::ffi;
use crate::ole::privs::*;
use crate::prelude::*;

/// [`CLSIDFromProgID`](https://learn.microsoft.com/en-us/windows/win32/api/combaseapi/nf-combaseapi-clsidfromprogid)
/// function.
///
/// # Examples
///
/// ```no_run
/// use winsafe::{self as w, prelude::*, co};
///
/// let _com_guard = w::CoInitializeEx(
///     co::COINIT::APARTMENTTHREADED | co::COINIT::DISABLE_OLE1DDE)?;
///
/// let cls_id = w::CLSIDFromProgID("Excel.Application")?;
///
/// let excel = w::CoCreateInstance::<w::IDispatch>(
///     &cls_id,
///     None::<&w::IUnknown>,
///     co::CLSCTX::LOCAL_SERVER,
/// )?;
///
/// let ids = excel.GetIDsOfNames(&["Workbooks"], w::LCID::USER_DEFAULT)?;
/// println!("{}", ids[0]);
/// # w::HrResult::Ok(())
/// ```
#[must_use]
pub fn CLSIDFromProgID(prog_id: &str) -> HrResult<co::CLSID> {
	let mut clsid = co::CLSID::default();
	ok_to_hrresult(unsafe {
		ffi::CLSIDFromProgID(WString::from_str(prog_id).as_ptr(), pvoid(&mut clsid))
	})
	.map(|_| clsid)
}

/// [`CLSIDFromProgIDEx`](https://learn.microsoft.com/en-us/windows/win32/api/combaseapi/nf-combaseapi-clsidfromprogidex)
/// function.
#[must_use]
pub fn CLSIDFromProgIDEx(prog_id: &str) -> HrResult<co::CLSID> {
	let mut clsid = co::CLSID::default();
	ok_to_hrresult(unsafe {
		ffi::CLSIDFromProgIDEx(WString::from_str(prog_id).as_ptr(), pvoid(&mut clsid))
	})
	.map(|_| clsid)
}

/// [`CLSIDFromString`](https://learn.microsoft.com/en-us/windows/win32/api/combaseapi/nf-combaseapi-clsidfromstring)
/// function.
#[must_use]
pub fn CLSIDFromString(prog_id: &str) -> HrResult<co::CLSID> {
	let mut clsid = co::CLSID::default();
	ok_to_hrresult(unsafe {
		ffi::CLSIDFromString(WString::from_str(prog_id).as_ptr(), pvoid(&mut clsid))
	})
	.map(|_| clsid)
}

/// [`CoCreateGuid`](https://learn.microsoft.com/en-us/windows/win32/api/combaseapi/nf-combaseapi-cocreateguid)
/// function.
///
/// Returns a globally unique 128-bit integer.
#[must_use]
pub fn CoCreateGuid() -> HrResult<GUID> {
	let mut guid = GUID::default();
	ok_to_hrresult(unsafe { ffi::CoCreateGuid(pvoid(&mut guid)) }).map(|_| guid)
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
/// use winsafe::{self as w, prelude::*, co};
///
/// let obj = w::CoCreateInstance::<w::ITaskbarList>(
///     &co::CLSID::TaskbarList,
///     None::<&w::IUnknown>,
///     co::CLSCTX::INPROC_SERVER,
/// )?;
/// # w::HrResult::Ok(())
/// ```
#[must_use]
pub fn CoCreateInstance<T>(
	clsid: &co::CLSID,
	iunk_outer: Option<&impl ole_IUnknown>,
	cls_context: co::CLSCTX,
) -> HrResult<T>
where
	T: ole_IUnknown,
{
	let mut queried = unsafe { T::null() };
	ok_to_hrresult(unsafe {
		ffi::CoCreateInstance(
			pcvoid(clsid),
			iunk_outer.map_or(std::ptr::null_mut(), |uo| uo.ptr()),
			cls_context.raw(),
			pcvoid(&T::IID),
			queried.as_mut(),
		)
	})
	.map(|_| queried)
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
/// ```no_run
/// use winsafe::{self as w, prelude::*, co};
///
/// let _com_guard = w::CoInitializeEx( // keep guard alive
///     co::COINIT::APARTMENTTHREADED
///     | co::COINIT::DISABLE_OLE1DDE,
/// )?;
///
/// // program runs...
///
/// // CoUninitialize() automatically called
/// # w::HrResult::Ok(())
/// ```
#[must_use]
pub fn CoInitializeEx(coinit: co::COINIT) -> HrResult<CoUninitializeGuard> {
	unsafe {
		let hr = co::HRESULT::from_raw(ffi::CoInitializeEx(std::ptr::null_mut(), coinit.raw()));
		match hr {
			co::HRESULT::S_OK | co::HRESULT::S_FALSE | co::HRESULT::RPC_E_CHANGED_MODE => {
				Ok(CoUninitializeGuard::new(hr))
			},
			hr => Err(hr),
		}
	}
}

/// [`CoLockObjectExternal`](https://learn.microsoft.com/en-us/windows/win32/api/combaseapi/nf-combaseapi-colockobjectexternal)
/// function.
///
/// Note that this function will lock the COM object, returning a
/// [`CoLockObjectExternalGuard`](crate::guard::CoLockObjectExternalGuard). The
/// unlocking is automatically performed by the guard when it goes out of scope.
pub fn CoLockObjectExternal<T>(obj: &T) -> HrResult<CoLockObjectExternalGuard<T>>
where
	T: ole_IUnknown,
{
	unsafe {
		ok_to_hrresult(ffi::CoLockObjectExternal(obj.ptr(), 1, 0))
			.map(|_| CoLockObjectExternalGuard::new(obj))
	}
}

/// [`CoTaskMemAlloc`](https://learn.microsoft.com/en-us/windows/win32/api/combaseapi/nf-combaseapi-cotaskmemalloc)
/// function.
///
/// # Examples
///
/// ```no_run
/// use winsafe::{self as w, prelude::*};
///
/// let pmem = w::CoTaskMemAlloc(100)?;
///
/// // use memory block...
///
/// // CoTaskMemFree() automatically called
/// # w::HrResult::Ok(())
/// ```
///
/// # Related functions
///
/// * [`CoTaskMemRealloc`](crate::CoTaskMemRealloc)
#[must_use]
pub fn CoTaskMemAlloc(cb: usize) -> HrResult<CoTaskMemFreeGuard> {
	let p = unsafe { ffi::CoTaskMemAlloc(cb) };
	if p.is_null() {
		Err(co::HRESULT::E_OUTOFMEMORY)
	} else {
		Ok(unsafe { CoTaskMemFreeGuard::new(p, cb) })
	}
}

/// [`CoTaskMemRealloc`](https://learn.microsoft.com/en-us/windows/win32/api/combaseapi/nf-combaseapi-cotaskmemfree)
/// function.
///
/// Originally this method returns a pointer to the reallocated memory block;
/// here the original pointer is automatically updated.
///
/// # Examples
///
/// ```no_run
/// use winsafe::{self as w, prelude::*};
///
/// let mut pmem = w::CoTaskMemAlloc(100)?;
/// w::CoTaskMemRealloc(&mut pmem, 120)?;
///
/// // use memory block...
///
/// // CoTaskMemFree() automatically called
/// # w::HrResult::Ok(())
/// ```
///
/// # Related functions
///
/// * [`CoTaskMemAlloc`](crate::CoTaskMemAlloc)
#[must_use]
pub fn CoTaskMemRealloc(pv: &mut CoTaskMemFreeGuard, cb: usize) -> HrResult<()> {
	let (old_pmem, _) = pv.leak();
	let p = unsafe { ffi::CoTaskMemRealloc(old_pmem, cb) };
	if p.is_null() {
		Err(co::HRESULT::E_OUTOFMEMORY)
	} else {
		*pv = unsafe { CoTaskMemFreeGuard::new(p, cb) };
		Ok(())
	}
}

/// [`CreateBindCtx`](https://learn.microsoft.com/en-us/windows/win32/api/objbase/nf-objbase-createbindctx)
/// function.
#[must_use]
pub fn CreateBindCtx() -> HrResult<IBindCtx> {
	let mut queried = unsafe { IBindCtx::null() };
	ok_to_hrresult(unsafe { ffi::CreateBindCtx(0, queried.as_mut()) }).map(|_| queried)
}

/// [`CreateClassMoniker`](https://learn.microsoft.com/en-us/windows/win32/api/objbase/nf-objbase-createclassmoniker)
/// function.
#[must_use]
pub fn CreateClassMoniker(clsid: &co::CLSID) -> HrResult<IMoniker> {
	let mut queried = unsafe { IMoniker::null() };
	ok_to_hrresult(unsafe { ffi::CreateClassMoniker(pcvoid(clsid), queried.as_mut()) })
		.map(|_| queried)
}

/// [`CreateFileMoniker`](https://learn.microsoft.com/en-us/windows/win32/api/objbase/nf-objbase-createfilemoniker)
/// function.
#[must_use]
pub fn CreateFileMoniker(path_name: &str) -> HrResult<IMoniker> {
	let mut queried = unsafe { IMoniker::null() };
	ok_to_hrresult(unsafe {
		ffi::CreateFileMoniker(WString::from_str(path_name).as_ptr(), queried.as_mut())
	})
	.map(|_| queried)
}

/// [`CreateItemMoniker`](https://learn.microsoft.com/en-us/windows/win32/api/objbase/nf-objbase-createitemmoniker)
/// function.
#[must_use]
pub fn CreateItemMoniker(delim: &str, item: &str) -> HrResult<IMoniker> {
	let mut queried = unsafe { IMoniker::null() };
	ok_to_hrresult(unsafe {
		ffi::CreateItemMoniker(
			WString::from_str(delim).as_ptr(),
			WString::from_str(item).as_ptr(),
			queried.as_mut(),
		)
	})
	.map(|_| queried)
}

/// [`CreateObjrefMoniker`](https://learn.microsoft.com/en-us/windows/win32/api/objbase/nf-objbase-createobjrefmoniker)
/// function.
#[must_use]
pub fn CreateObjrefMoniker(unk: &impl ole_IUnknown) -> HrResult<IMoniker> {
	let mut queried = unsafe { IMoniker::null() };
	ok_to_hrresult(unsafe { ffi::CreateObjrefMoniker(unk.ptr(), queried.as_mut()) })
		.map(|_| queried)
}

/// [`CreatePointerMoniker`](https://learn.microsoft.com/en-us/windows/win32/api/objbase/nf-objbase-createpointermoniker)
/// function.
#[must_use]
pub fn CreatePointerMoniker(unk: &impl ole_IUnknown) -> HrResult<IMoniker> {
	let mut queried = unsafe { IMoniker::null() };
	ok_to_hrresult(unsafe { ffi::CreatePointerMoniker(unk.ptr(), queried.as_mut()) })
		.map(|_| queried)
}

/// [`OleInitialize`](https://learn.microsoft.com/en-us/windows/win32/api/ole2/nf-ole2-oleinitialize)
/// function, which calls [`CoInitializeEx`](crate::CoInitializeEx) and enables
/// OLE operations.
///
/// In the original C implementation, you must call
/// [`OleUninitialize`](https://learn.microsoft.com/en-us/windows/win32/api/ole2/nf-ole2-oleuninitialize)
/// as a cleanup operation.
///
/// Here, the cleanup is performed automatically, because `OleInitialize`
/// returns an [`OleUninitializeGuard`](crate::guard::OleUninitializeGuard),
/// which automatically calls `OleUninitialize` when the guard goes out of
/// scope. You must, however, keep the guard alive, otherwise the cleanup will
/// be performed right away.
///
/// # Examples
///
/// ```no_run
/// use winsafe::{self as w, prelude::*, co};
///
/// let _ole_guard = w::OleInitialize()?;
///
/// // program runs...
///
/// // OleUninitialize() automatically called
/// # w::HrResult::Ok(())
/// ```
#[must_use]
pub fn OleInitialize() -> HrResult<OleUninitializeGuard> {
	unsafe {
		ok_to_hrresult(ffi::OleInitialize(std::ptr::null_mut()))
			.map(|_| OleUninitializeGuard::new())
	}
}

/// [`StringFromCLSID`](https://learn.microsoft.com/en-us/windows/win32/api/combaseapi/nf-combaseapi-stringfromclsid)
/// function.
#[must_use]
pub fn StringFromCLSID(clsid: &co::CLSID) -> HrResult<String> {
	let mut pstr = std::ptr::null_mut::<u16>();
	ok_to_hrresult(unsafe { ffi::StringFromCLSID(pcvoid(clsid), &mut pstr) })
		.map(|_| htaskmem_ptr_to_str(pstr))
}
