#![allow(non_snake_case)]

use crate::co;
use crate::decl::*;
use crate::guard::*;
use crate::kernel::privs::*;
use crate::ole::privs::*;
use crate::prelude::*;
use crate::shell::ffi;

/// [`CommandLineToArgv`](https://learn.microsoft.com/en-us/windows/win32/api/shellapi/nf-shellapi-commandlinetoargvw)
/// function.
///
/// # Examples
///
/// ```no_run
/// use winsafe::{self as w, prelude::*};
///
/// let args = w::CommandLineToArgv(&w::GetCommandLine())?;
/// for arg in args.iter() {
///     println!("{}", arg);
/// }
/// # w::SysResult::Ok(())
/// ```
#[must_use]
pub fn CommandLineToArgv(cmd_line: &str) -> SysResult<Vec<String>> {
	let mut num_args = 0i32;
	let lp_arr =
		unsafe { ffi::CommandLineToArgvW(WString::from_str(cmd_line).as_ptr(), &mut num_args) };
	if lp_arr.is_null() {
		return Err(GetLastError());
	}

	let mut strs = Vec::with_capacity(num_args as _);
	for lp in unsafe { std::slice::from_raw_parts(lp_arr, num_args as _) }.iter() {
		strs.push(unsafe { WString::from_wchars_nullt(*lp) }.to_string());
	}

	let _ = unsafe { LocalFreeGuard::new(HLOCAL::from_ptr(lp_arr as _)) };
	Ok(strs)
}

/// [`GetAllUsersProfileDirectory`](https://learn.microsoft.com/en-us/windows/win32/api/userenv/nf-userenv-getallusersprofiledirectoryw)
/// function.
///
/// # Related functions
///
/// * [`GetDefaultUserProfileDirectory`](crate::GetDefaultUserProfileDirectory)
/// * [`GetProfilesDirectory`](crate::GetProfilesDirectory)
#[must_use]
pub fn GetAllUsersProfileDirectory() -> SysResult<String> {
	let mut len = 0u32;
	unsafe {
		ffi::GetAllUsersProfileDirectoryW(std::ptr::null_mut(), &mut len);
	}
	match GetLastError() {
		co::ERROR::INSUFFICIENT_BUFFER => {},
		e => return Err(e),
	}

	let mut buf = WString::new_alloc_buf(len as _);
	BoolRet(unsafe { ffi::GetAllUsersProfileDirectoryW(buf.as_mut_ptr(), &mut len) })
		.to_sysresult()
		.map(|_| buf.to_string())
}

/// [`GetCurrentProcessExplicitAppUserModelID`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-getcurrentprocessexplicitappusermodelid)
/// function.
///
/// # Related functions
///
/// * [`SetCurrentProcessExplicitAppUserModelID`](crate::SetCurrentProcessExplicitAppUserModelID)
#[must_use]
pub fn GetCurrentProcessExplicitAppUserModelID() -> HrResult<String> {
	let mut pstr = std::ptr::null_mut() as *mut u16;
	HrRet(unsafe { ffi::GetCurrentProcessExplicitAppUserModelID(&mut pstr) }).to_hrresult()?;
	let app_name = unsafe { WString::from_wchars_nullt(pstr) }.to_string();
	let _ = unsafe { CoTaskMemFreeGuard::new(pstr as _, 0) };
	Ok(app_name)
}

/// [`GetDefaultUserProfileDirectory`](https://learn.microsoft.com/en-us/windows/win32/api/userenv/nf-userenv-getdefaultuserprofiledirectoryw)
/// function.
///
/// # Related functions
///
/// * [`GetAllUsersProfileDirectory`](crate::GetAllUsersProfileDirectory)
/// * [`GetProfilesDirectory`](crate::GetProfilesDirectory)
#[must_use]
pub fn GetDefaultUserProfileDirectory() -> SysResult<String> {
	let mut len = 0u32;
	unsafe {
		ffi::GetDefaultUserProfileDirectoryW(std::ptr::null_mut(), &mut len);
	}
	match GetLastError() {
		co::ERROR::INSUFFICIENT_BUFFER => {},
		e => return Err(e),
	}

	let mut buf = WString::new_alloc_buf(len as _);
	BoolRet(unsafe { ffi::GetDefaultUserProfileDirectoryW(buf.as_mut_ptr(), &mut len) })
		.to_sysresult()
		.map(|_| buf.to_string())
}

/// [`GetProfilesDirectory`](https://learn.microsoft.com/en-us/windows/win32/api/userenv/nf-userenv-getprofilesdirectoryw)
/// function.
///
/// # Related functions
///
/// * [`GetAllUsersProfileDirectory`](crate::GetAllUsersProfileDirectory)
/// * [`GetDefaultUserProfileDirectory`](crate::GetDefaultUserProfileDirectory)
#[must_use]
pub fn GetProfilesDirectory() -> SysResult<String> {
	let mut len = 0u32;
	unsafe {
		ffi::GetProfilesDirectoryW(std::ptr::null_mut(), &mut len);
	}
	match GetLastError() {
		co::ERROR::INSUFFICIENT_BUFFER => {},
		e => return Err(e),
	}

	let mut buf = WString::new_alloc_buf(len as _);
	BoolRet(unsafe { ffi::GetProfilesDirectoryW(buf.as_mut_ptr(), &mut len) })
		.to_sysresult()
		.map(|_| buf.to_string())
}

/// [`PathCombine`](https://learn.microsoft.com/en-us/windows/win32/api/shlwapi/nf-shlwapi-pathcombinew)
/// function.
///
/// # Examples
///
/// ```no_run
/// use winsafe::{self as w, prelude::*};
///
/// let full = w::PathCombine(
///     Some("C:"),
///     Some("One\\Two\\Three"),
/// )?;
///
/// // full = "C:\\One\\Two\\Three"
/// # w::HrResult::Ok(())
/// ```
pub fn PathCombine(str_dir: Option<&str>, str_file: Option<&str>) -> HrResult<String> {
	let mut buf = WString::new_alloc_buf(MAX_PATH);
	if unsafe {
		ffi::PathCombineW(
			buf.as_mut_ptr(),
			WString::from_opt_str(str_dir).as_ptr(),
			WString::from_opt_str(str_file).as_ptr(),
		)
	}
	.is_null()
	{
		Err(co::HRESULT::E_INVALIDARG)
	} else {
		Ok(buf.to_string())
	}
}

/// [`PathCommonPrefix`](https://learn.microsoft.com/en-us/windows/win32/api/shlwapi/nf-shlwapi-pathcommonprefixw)
/// function.
///
/// # Examples
///
/// ```no_run
/// use winsafe::{self as w, prelude::*};
///
/// if let Some(common_prefix) = w::PathCommonPrefix(
///     "C:\\temp\\one\\foo.txt",
///     "C:\\temp\\two\\bar.txt",
/// ) {
///     println!("Common prefix: {}", common_prefix); // "C:\\temp"
/// }
/// ```
pub fn PathCommonPrefix(file1: &str, file2: &str) -> Option<String> {
	let mut buf = WString::new_alloc_buf(MAX_PATH);
	match unsafe {
		ffi::PathCommonPrefixW(
			WString::from_str(file1).as_ptr(),
			WString::from_str(file2).as_ptr(),
			buf.as_mut_ptr(),
		)
	} {
		0 => None,
		_ => Some(buf.to_string()),
	}
}

/// [`PathSkipRoot`](https://learn.microsoft.com/en-us/windows/win32/api/shlwapi/nf-shlwapi-pathskiprootw)
/// function.
pub fn PathSkipRoot(str_path: &str) -> Option<String> {
	let buf = WString::from_str(str_path);
	unsafe { ffi::PathSkipRootW(buf.as_ptr()).as_ref() }
		.map(|ptr| unsafe { WString::from_wchars_nullt(ptr) }.to_string())
}

/// [`PathStripPath`](https://learn.microsoft.com/en-us/windows/win32/api/shlwapi/nf-shlwapi-pathstrippathw)
/// function.
pub fn PathStripPath(str_path: &str) -> String {
	let mut buf = WString::from_str(str_path);
	unsafe {
		ffi::PathStripPathW(buf.as_mut_ptr());
	}
	buf.to_string()
}

/// [`PathUndecorate`](https://learn.microsoft.com/en-us/windows/win32/api/shlwapi/nf-shlwapi-pathundecoratew)
/// function.
pub fn PathUndecorate(str_path: &str) -> String {
	let mut buf = WString::from_str(str_path);
	unsafe {
		ffi::PathUndecorateW(buf.as_mut_ptr());
	}
	buf.to_string()
}

/// [`PathUnquoteSpaces`](https://learn.microsoft.com/en-us/windows/win32/api/shlwapi/nf-shlwapi-pathunquotespacesw)
/// function.
pub fn PathUnquoteSpaces(str_path: &str) -> String {
	let mut buf = WString::from_str(str_path);
	unsafe {
		ffi::PathUnquoteSpacesW(buf.as_mut_ptr());
	}
	buf.to_string()
}

/// [`SetCurrentProcessExplicitAppUserModelID`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-setcurrentprocessexplicitappusermodelid)
/// function.
///
/// # Related functions
///
/// * [`GetCurrentProcessExplicitAppUserModelID`](crate::GetCurrentProcessExplicitAppUserModelID)
pub fn SetCurrentProcessExplicitAppUserModelID(app_id: &str) -> HrResult<()> {
	HrRet(unsafe {
		ffi::SetCurrentProcessExplicitAppUserModelID(WString::from_str(app_id).as_ptr())
	})
	.to_hrresult()
}

/// [`SHAddToRecentDocs`](https://learn.microsoft.com/en-us/windows/win32/api/shlobj_core/nf-shlobj_core-shaddtorecentdocs)
/// function.
///
/// # Safety
///
/// The `pv` type varies according to `uFlags`. If you set it wrong, you're
/// likely to cause a buffer overrun.
pub unsafe fn SHAddToRecentDocs<T>(flags: co::SHARD, pv: &T) {
	unsafe {
		ffi::SHAddToRecentDocs(flags.raw(), pcvoid(pv));
	}
}

/// [`SHBindToParent`](https://learn.microsoft.com/en-us/windows/win32/api/shlobj_core/nf-shlobj_core-shbindtoparent)
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
/// let f = w::SHCreateItemFromParsingName::<w::IShellItem>(
///     "C:\\Temp",
///     None::<&w::IBindCtx>,
/// )?;
///
/// let pidl = w::SHGetIDListFromObject(&f)?;
///
/// let (f2, pidl2) = w::SHBindToParent::<w::IShellFolder>(&pidl)?;
/// # w::HrResult::Ok(())
/// ```
#[must_use]
pub fn SHBindToParent<T>(pidl: &PIDL) -> HrResult<(T, PIDL)>
where
	T: ole_IUnknown,
{
	unsafe {
		let mut queried = T::null();
		let mut pidl_last = PIDL::from_ptr(std::ptr::null_mut()); // belongs to the system

		HrRet(ffi::SHBindToParent(
			pidl.ptr() as _,
			pcvoid(&T::IID),
			queried.as_mut(),
			pvoid(&mut pidl_last),
		))
		.to_hrresult()
		.map(|_| (queried, pidl_last))
	}
}

/// [`SHCreateItemFromIDList`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-shcreateitemfromidlist)
/// function.
///
/// # Related functions
///
/// * [`SHGetIDListFromObject`](crate::SHGetIDListFromObject)
#[must_use]
pub fn SHCreateItemFromIDList<T>(pidl: &PIDL) -> HrResult<T>
where
	T: shell_IShellItem,
{
	let mut queried = unsafe { T::null() };
	HrRet(unsafe {
		ffi::SHCreateItemFromIDList(pidl.ptr() as _, pcvoid(&T::IID), queried.as_mut())
	})
	.to_hrresult()
	.map(|_| queried)
}

/// [`SHCreateItemFromParsingName`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-shcreateitemfromparsingname)
/// function.
///
/// # Examples
///
/// ```no_run
/// use winsafe::{self as w, prelude::*};
///
/// let shi = w::SHCreateItemFromParsingName::<w::IShellItem2>(
///     "C:\\Temp\\foo.txt",
///     None::<&w::IBindCtx>,
/// )?;
/// # w::HrResult::Ok(())
/// ```
#[must_use]
pub fn SHCreateItemFromParsingName<T>(
	file_or_folder_path: &str,
	bind_ctx: Option<&impl ole_IBindCtx>,
) -> HrResult<T>
where
	T: shell_IShellItem,
{
	let mut queried = unsafe { T::null() };
	HrRet(unsafe {
		ffi::SHCreateItemFromParsingName(
			WString::from_str(file_or_folder_path).as_ptr(),
			bind_ctx.map_or(std::ptr::null_mut(), |p| p.ptr()),
			pcvoid(&T::IID),
			queried.as_mut(),
		)
	})
	.to_hrresult()
	.map(|_| queried)
}

/// [`SHCreateItemFromRelativeName`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-shcreateitemfromrelativename)
/// function.
#[must_use]
pub fn SHCreateItemFromRelativeName<T>(
	parent: &impl shell_IShellItem,
	name: &str,
	bind_ctx: Option<&impl ole_IBindCtx>,
) -> HrResult<T>
where
	T: shell_IShellItem,
{
	let mut queried = unsafe { T::null() };
	HrRet(unsafe {
		ffi::SHCreateItemFromRelativeName(
			parent.ptr(),
			WString::from_str(name).as_ptr(),
			bind_ctx.map_or(std::ptr::null_mut(), |p| p.ptr()),
			pcvoid(&T::IID),
			queried.as_mut(),
		)
	})
	.to_hrresult()
	.map(|_| queried)
}

/// [`SHCreateItemInKnownFolder`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-shcreateiteminknownfolder)
/// function.
#[must_use]
pub fn SHCreateItemInKnownFolder<T>(
	folder_id: &co::KNOWNFOLDERID,
	flags: co::KF,
	item: &str,
) -> HrResult<T>
where
	T: shell_IShellItem,
{
	let mut queried = unsafe { T::null() };
	HrRet(unsafe {
		ffi::SHCreateItemInKnownFolder(
			pcvoid(folder_id),
			flags.raw(),
			WString::from_str(item).as_ptr(),
			pcvoid(&T::IID),
			queried.as_mut(),
		)
	})
	.to_hrresult()
	.map(|_| queried)
}

/// [`SHCreateShellItemArray`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-shcreateshellitemarray)
/// method.
#[must_use]
pub fn SHCreateShellItemArray(
	pidl_parent: Option<&PIDL>,
	folder: Option<&impl shell_IShellFolder>,
	pidl_children: &[&PIDL],
) -> HrResult<IShellItemArray> {
	let mut queried = unsafe { IShellItemArray::null() };
	let pidl_ptrs = pidl_children.iter().map(|p| p.ptr()).collect::<Vec<_>>();

	HrRet(unsafe {
		ffi::SHCreateShellItemArray(
			pidl_parent.map_or(std::ptr::null(), |p| p.ptr() as _),
			folder.map_or(std::ptr::null_mut(), |p| p.ptr()),
			pidl_ptrs.len() as _,
			vec_ptr(&pidl_ptrs) as _,
			queried.as_mut(),
		)
	})
	.to_hrresult()
	.map(|_| queried)
}

/// [`SHCreateShellItemArrayFromShellItem`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-shcreateshellitemarrayfromshellitem)
/// function.
#[must_use]
pub fn SHCreateShellItemArrayFromShellItem<T>(item: &impl shell_IShellItem) -> HrResult<T>
where
	T: shell_IShellItemArray,
{
	let mut queried = unsafe { T::null() };
	HrRet(unsafe {
		ffi::SHCreateShellItemArrayFromShellItem(item.ptr(), pcvoid(&T::IID), queried.as_mut())
	})
	.to_hrresult()
	.map(|_| queried)
}

/// [`SHCreateMemStream`](https://learn.microsoft.com/en-us/windows/win32/api/shlwapi/nf-shlwapi-shcreatememstream)
/// function.
///
/// # Examples
///
/// Loading from a `Vec`:
///
/// ```no_run
/// use winsafe::{self as w, prelude::*};
///
/// let raw_data: Vec<u8>; // initialized somewhere
/// # let raw_data = Vec::<u8>::new();
///
/// let stream = w::SHCreateMemStream(&raw_data)?;
/// # w::HrResult::Ok(())
/// ```
#[must_use]
pub fn SHCreateMemStream(src: &[u8]) -> HrResult<IStream> {
	let p = unsafe { ffi::SHCreateMemStream(vec_ptr(src), src.len() as _) };
	if p.is_null() { Err(co::HRESULT::E_OUTOFMEMORY) } else { Ok(unsafe { IStream::from_ptr(p) }) }
}

/// [`SHGetIDListFromObject`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-shgetidlistfromobject)
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
/// let f = w::SHCreateItemFromParsingName::<w::IShellItem>(
///     "C:\\Temp",
///     None::<&w::IBindCtx>,
/// )?;
///
/// let pidl = w::SHGetIDListFromObject(&f)?;
/// # w::HrResult::Ok(())
/// ```
///
/// # Related functions
///
/// * [`SHCreateItemFromIDList`](crate::SHCreateItemFromIDList)
#[must_use]
pub fn SHGetIDListFromObject(obj: &impl ole_IUnknown) -> HrResult<CoTaskMemFreePidlGuard> {
	unsafe {
		let mut pidl = PIDL::from_ptr(std::ptr::null_mut());
		HrRet(ffi::SHGetIDListFromObject(obj.ptr(), pvoid(&mut pidl)))
			.to_hrresult()
			.map(|_| CoTaskMemFreePidlGuard::new(pidl))
	}
}

/// [`SHGetPropertyStoreFromIDList`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-shgetpropertystorefromidlist)
/// function.
#[must_use]
pub fn SHGetPropertyStoreFromIDList<T>(pidl: &PIDL, flags: co::GPS) -> HrResult<T>
where
	T: oleaut_IPropertyStore,
{
	let mut queried = unsafe { T::null() };
	HrRet(unsafe {
		ffi::SHGetPropertyStoreFromIDList(
			pidl.ptr() as _,
			flags.raw(),
			pcvoid(&T::IID),
			queried.as_mut(),
		)
	})
	.to_hrresult()
	.map(|_| queried)
}

/// [`SHGetPropertyStoreFromParsingName`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-shgetpropertystorefromparsingname)
/// function.
#[must_use]
pub fn SHGetPropertyStoreFromParsingName<T>(
	path: &str,
	bind_ctx: Option<&impl ole_IBindCtx>,
	flags: co::GPS,
) -> HrResult<T>
where
	T: oleaut_IPropertyStore,
{
	let mut queried = unsafe { T::null() };
	HrRet(unsafe {
		ffi::SHGetPropertyStoreFromParsingName(
			WString::from_str(path).as_ptr(),
			bind_ctx.map_or(std::ptr::null_mut(), |p| p.ptr()),
			flags.raw(),
			pcvoid(&T::IID),
			queried.as_mut(),
		)
	})
	.to_hrresult()
	.map(|_| queried)
}

/// [`Shell_NotifyIcon`](https://learn.microsoft.com/en-us/windows/win32/api/shellapi/nf-shellapi-shell_notifyiconw)
/// function.
pub fn Shell_NotifyIcon(message: co::NIM, data: &NOTIFYICONDATA) -> HrResult<()> {
	match unsafe { ffi::Shell_NotifyIconW(message.raw(), pcvoid(data)) } {
		0 => Err(co::HRESULT::E_FAIL),
		_ => Ok(()),
	}
}

/// [`SHFileOperation`](https://learn.microsoft.com/en-us/windows/win32/api/shellapi/nf-shellapi-shfileoperationw)
/// function.
pub fn SHFileOperation(file_op: &mut SHFILEOPSTRUCT) -> HrResult<()> {
	unsafe {
		match { ffi::SHFileOperationW(pvoid(file_op)) } {
			0 => Ok(()),
			de => Err(co::HRESULT::from_raw(de as _)),
		}
	}
}

/// [`SHGetFileInfo`](https://learn.microsoft.com/en-us/windows/win32/api/shellapi/nf-shellapi-shgetfileinfow)
/// function.
pub fn SHGetFileInfo(
	path: &str,
	file_attrs: co::FILE_ATTRIBUTE,
	flags: co::SHGFI,
) -> HrResult<(u32, DestroyIconShfiGuard)> {
	let mut shfi = SHFILEINFO::default();
	unsafe {
		match ffi::SHGetFileInfoW(
			WString::from_str(path).as_ptr(),
			file_attrs.raw(),
			pvoid(&mut shfi),
			std::mem::size_of::<SHFILEINFO>() as _,
			flags.raw(),
		) {
			0 => Err(co::HRESULT::E_FAIL),
			n => Ok((n as _, DestroyIconShfiGuard::new(shfi))),
		}
	}
}

/// [`SHGetStockIconInfo`](https://learn.microsoft.com/en-us/windows/win32/api/shellapi/nf-shellapi-shgetstockiconinfo)
/// function.
///
/// # Examples
///
/// Loading the small (16x16 pixels) camera icon from the system:
///
/// ```no_run
/// use winsafe::{self as w, prelude::*, co};
///
/// let sii = w::SHGetStockIconInfo(
///     co::SIID::DEVICECAMERA,
///     co::SHGSI::ICON | co::SHGSI::SMALLICON,
/// )?;
///
/// println!("HICON handle: {}", sii.hIcon);
/// # w::AnyResult::Ok(())
/// ```
pub fn SHGetStockIconInfo(siid: co::SIID, flags: co::SHGSI) -> HrResult<DestroyIconSiiGuard> {
	let mut sii = SHSTOCKICONINFO::default();
	unsafe {
		HrRet(ffi::SHGetStockIconInfo(siid.raw(), flags.raw(), pvoid(&mut sii)))
			.to_hrresult()
			.map(|_| DestroyIconSiiGuard::new(sii))
	}
}
