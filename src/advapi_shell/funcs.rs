#![allow(non_snake_case)]

use crate::advapi_shell::ffi;
use crate::co;
use crate::decl::*;
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
	BoolRet(unsafe { ffi::ShellExecuteExW(pvoid(&mut buf.raw)) }).to_sysresult()
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
	HrRet(unsafe {
		ffi::SHGetKnownFolderPath(
			pcvoid(folder_id),
			flags.raw(),
			token.map_or(std::ptr::null_mut(), |t| t.ptr()),
			&mut pstr,
		)
	})
	.to_hrresult()
	.map(|_| htaskmem_ptr_to_str(pstr))
}
