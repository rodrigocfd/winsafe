#![allow(non_snake_case)]

use crate::advapi_shell::ffi;
use crate::co;
use crate::decl::*;
use crate::guard::*;
use crate::kernel::privs::*;
use crate::ole::privs::*;

/// [`ShellExecuteEx`](https://learn.microsoft.com/en-us/windows/win32/api/shellapi/nf-shellapi-shellexecuteexw)
/// function.
///
/// Fill the [`SHELLEXECUTEINFO`](crate::SHELLEXECUTEINFO) fields you need, and
/// leave the others as default. The needed mask flags will be automatically
/// set.
///
/// # Examples
///
/// ```no_run
/// use winsafe::{self as w, prelude::*, co};
///
/// w::ShellExecuteEx(&w::SHELLEXECUTEINFO {
///     file: "C:\\Temp\\foo.exe",
///     show: co::SW::SHOW,
///     ..Default::default()
/// })?;
/// # w::SysResult::Ok(())
/// ```
pub fn ShellExecuteEx(exec_info: &SHELLEXECUTEINFO) -> SysResult<()> {
	let mut buf = exec_info.to_raw();
	bool_to_sysresult(unsafe { ffi::ShellExecuteExW(pvoid(&mut buf.raw)) })
}

/// [`SHGetKnownFolderPath`](https://learn.microsoft.com/en-us/windows/win32/api/shlobj_core/nf-shlobj_core-shgetknownfolderpath)
/// function.
///
/// # Examples
///
/// Retrieving documents folder:
///
/// ```no_run
/// use winsafe::{self as w, prelude::*, co};
///
/// let docs_folder = w::SHGetKnownFolderPath(
///     &co::KNOWNFOLDERID::Documents,
///     co::KF::DEFAULT,
///     None,
/// )?;
///
/// println!("Docs folder: {}", docs_folder);
/// # w::HrResult::Ok(())
/// ```
#[must_use]
pub fn SHGetKnownFolderPath(
	folder_id: &co::KNOWNFOLDERID,
	flags: co::KF,
	token: Option<&HACCESSTOKEN>,
) -> HrResult<String> {
	let mut pstr = std::ptr::null_mut::<u16>();
	ok_to_hrresult(unsafe {
		ffi::SHGetKnownFolderPath(
			pcvoid(folder_id),
			flags.raw(),
			token.map_or(std::ptr::null_mut(), |t| t.ptr()),
			&mut pstr,
		)
	})
	.map(|_| {
		let path = unsafe { WString::from_wchars_nullt(pstr) };
		let _ = unsafe { CoTaskMemFreeGuard::new(pstr as _, 0) };
		path.to_string()
	})
}
