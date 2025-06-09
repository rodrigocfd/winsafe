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
	bool_to_sysresult(unsafe { ffi::GetAllUsersProfileDirectoryW(buf.as_mut_ptr(), &mut len) })
		.map(|_| buf.to_string())
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
	bool_to_sysresult(unsafe { ffi::GetDefaultUserProfileDirectoryW(buf.as_mut_ptr(), &mut len) })
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
	bool_to_sysresult(unsafe { ffi::GetProfilesDirectoryW(buf.as_mut_ptr(), &mut len) })
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
	ok_to_hrresult(unsafe {
		ffi::SHCreateItemFromIDList(pidl.0 as _, pcvoid(&T::IID), queried.as_mut())
	})
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
	ok_to_hrresult(unsafe {
		ffi::SHCreateItemFromParsingName(
			WString::from_str(file_or_folder_path).as_ptr(),
			bind_ctx.map_or(std::ptr::null_mut(), |p| p.ptr()),
			pcvoid(&T::IID),
			queried.as_mut(),
		)
	})
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
	ok_to_hrresult(unsafe {
		ffi::SHCreateItemFromRelativeName(
			parent.ptr(),
			WString::from_str(name).as_ptr(),
			bind_ctx.map_or(std::ptr::null_mut(), |p| p.ptr()),
			pcvoid(&T::IID),
			queried.as_mut(),
		)
	})
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
	ok_to_hrresult(unsafe {
		ffi::SHCreateItemInKnownFolder(
			pcvoid(folder_id),
			flags.raw(),
			WString::from_str(item).as_ptr(),
			pcvoid(&T::IID),
			queried.as_mut(),
		)
	})
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
	ok_to_hrresult(unsafe {
		ffi::SHCreateShellItemArrayFromShellItem(item.ptr(), pcvoid(&T::IID), queried.as_mut())
	})
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
/// # Related functions
///
/// * [`SHCreateItemFromIDList`](crate::SHCreateItemFromIDList)
#[must_use]
pub fn SHGetIDListFromObject(obj: &impl ole_IUnknown) -> HrResult<CoTaskMemFreePidlGuard> {
	let mut pidl = PIDL(std::ptr::null_mut());
	unsafe {
		ok_to_hrresult(ffi::SHGetIDListFromObject(obj.ptr(), pvoid(&mut pidl)))
			.map(|_| CoTaskMemFreePidlGuard::new(pidl))
	}
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
		ok_to_hrresult(ffi::SHGetStockIconInfo(siid.raw(), flags.raw(), pvoid(&mut sii)))
			.map(|_| DestroyIconSiiGuard::new(sii))
	}
}
