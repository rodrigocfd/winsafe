#![allow(non_snake_case)]

use crate::{co, shell};
use crate::kernel::decl::{GetLastError, HACCESSTOKEN, HLOCAL, WinResult,
	WString};
use crate::kernel::privs::bool_to_winresult;
use crate::ole::decl::{CoTaskMemFree, HrResult};
use crate::ole::privs::ok_to_hrresult;
use crate::prelude::{Handle, KernelHlocal};
use crate::shell::decl::{KNOWNFOLDERID, NOTIFYICONDATA, SHFILEINFO,
	SHFILEOPSTRUCT};

/// [`CommandLineToArgv`](https://docs.microsoft.com/en-us/windows/win32/api/shellapi/nf-shellapi-commandlinetoargvw)
/// function.
///
/// # Examples
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// use winsafe::{CommandLineToArgv, GetCommandLine};
///
/// let args = CommandLineToArgv(&GetCommandLine())?;
/// for arg in args.iter() {
///     println!("{}", arg);
/// }
/// # Ok::<_, winsafe::co::ERROR>(())
/// ```
#[cfg_attr(docsrs, doc(cfg(feature = "shell")))]
pub fn CommandLineToArgv(cmd_line: &str) -> WinResult<Vec<String>> {
	let mut num_args = i32::default();
	let lp_arr = unsafe {
		shell::ffi::CommandLineToArgvW(
			WString::from_str(cmd_line).as_ptr(),
			&mut num_args,
		)
	};
	if lp_arr.is_null() {
		return Err(GetLastError());
	}

	let mut strs = Vec::with_capacity(num_args as _);
	for lp in unsafe { std::slice::from_raw_parts(lp_arr, num_args as _) }.iter() {
		strs.push(WString::from_wchars_nullt(*lp).to_string());
	}

	(HLOCAL(lp_arr as _))
		.LocalFree()
		.map(|_| strs)
}

/// [`SHAddToRecentDocs`](https://docs.microsoft.com/en-us/windows/win32/api/shlobj_core/nf-shlobj_core-shaddtorecentdocs)
/// function.
///
/// **Note:** The `pv` type varies according to `uFlags`. If you set it wrong,
/// you're likely to cause a buffer overrun.
#[cfg_attr(docsrs, doc(cfg(feature = "shell")))]
pub unsafe fn SHAddToRecentDocs<T>(flags: co::SHARD, pv: &T) {
	shell::ffi::SHAddToRecentDocs(flags.0, pv as *const _ as _);
}

/// [`Shell_NotifyIcon`](https://docs.microsoft.com/en-us/windows/win32/api/shellapi/nf-shellapi-shell_notifyiconw)
/// function.
#[cfg_attr(docsrs, doc(cfg(feature = "shell")))]
pub fn Shell_NotifyIcon(
	message: co::NIM, data: &mut NOTIFYICONDATA) -> WinResult<()>
{
	bool_to_winresult(
		unsafe { shell::ffi::Shell_NotifyIconW(message.0, data as *mut _ as _) },
	)
}

/// [`SHFileOperation`](https://docs.microsoft.com/en-us/windows/win32/api/shellapi/nf-shellapi-shfileoperationw)
/// function.
#[cfg_attr(docsrs, doc(cfg(feature = "shell")))]
pub fn SHFileOperation(file_op: &mut SHFILEOPSTRUCT) -> WinResult<()> {
	match unsafe {
		shell::ffi::SHFileOperationW(file_op as *mut _ as _)
	} {
		0 => Err(GetLastError()),
		_ => Ok(()),
	}
}

/// [`SHGetFileInfo`](https://docs.microsoft.com/en-us/windows/win32/api/shellapi/nf-shellapi-shgetfileinfow)
/// function.
///
/// **Note:** If you are returning an icon in the `hIcon` member of
/// [`SHFILEINFO`](crate::SHFILEINFO), it must be paired with an
/// [`HICON::DestroyIcon`](crate::prelude::UserHicon::DestroyIcon) call.
#[cfg_attr(docsrs, doc(cfg(feature = "shell")))]
pub fn SHGetFileInfo(
	path: &str, file_attrs: co::FILE_ATTRIBUTE,
	shfi: &mut SHFILEINFO, flags: co::SHGFI) -> WinResult<u32>
{
	match unsafe {
		shell::ffi::SHGetFileInfoW(
			WString::from_str(path).as_ptr(),
			file_attrs.0,
			shfi as *mut _ as _,
			std::mem::size_of::<SHFILEINFO>() as _,
			flags.0,
		)
	} {
		0 => Err(GetLastError()),
		n => Ok(n as _),
	}
}

/// [`SHGetKnownFolderPath`](https://docs.microsoft.com/en-us/windows/win32/api/shlobj_core/nf-shlobj_core-shgetknownfolderpath)
/// function.
#[cfg_attr(docsrs, doc(cfg(feature = "shell")))]
pub fn SHGetKnownFolderPath(
	folder_id: &KNOWNFOLDERID,
	flags: co::KF,
	token: Option<HACCESSTOKEN>) -> HrResult<String>
{
	let mut pstr: *mut u16 = std::ptr::null_mut();
	ok_to_hrresult(
		unsafe {
			shell::ffi::SHGetKnownFolderPath(
				folder_id as *const _ as _,
				flags.0,
				token.map_or(std::ptr::null_mut(), |t| t.as_ptr()),
				&mut pstr,
			)
		},
	).map(|_| {
		let path = WString::from_wchars_nullt(pstr);
		CoTaskMemFree(pstr);
		path.to_string()
	})
}
