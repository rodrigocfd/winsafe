#![allow(non_camel_case_types, non_snake_case)]

use crate::{co, shell};
use crate::kernel::decl::{HINSTANCE, SysResult, WString};
use crate::kernel::privs::bool_to_sysresult;
use crate::prelude::Handle;
use crate::user::decl::{HICON, HWND};

impl shell_Hwnd for HWND {}

/// This trait is enabled with the `shell` feature, and provides methods for
/// [`HWND`](crate::HWND).
///
/// Prefer importing this trait through the prelude:
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// ```
pub trait shell_Hwnd: Handle {
	/// [`DragAcceptFiles`](https://learn.microsoft.com/en-us/windows/win32/api/shellapi/nf-shellapi-dragacceptfiles)
	/// method.
	fn DragAcceptFiles(&self, accept: bool) {
		unsafe { shell::ffi::DragAcceptFiles(self.as_ptr(), accept as _); }
	}

	/// [`ShellAbout`](https://learn.microsoft.com/en-us/windows/win32/api/shellapi/nf-shellapi-shellaboutw)
	/// method.
	fn ShellAbout(&self,
		title_bar: &str,
		first_line: Option<&str>,
		other_stuff: Option<&str>,
		hicon: Option<&HICON>,
	) -> SysResult<()>
	{
		let mut wapp = WString::from_str(title_bar);
		if let Some(line) = first_line {
			wapp.append("#");
			wapp.append(line);
		}

		bool_to_sysresult(
			unsafe {
				shell::ffi::ShellAboutW(
					self.as_ptr(),
					wapp.as_ptr(),
					WString::from_opt_str(other_stuff).as_ptr(),
					hicon.map_or(std::ptr::null_mut(), |h| h.as_ptr()),
				)
			},
		)
	}

	/// [`ShellExecute`](https://learn.microsoft.com/en-us/windows/win32/api/shellapi/nf-shellapi-shellexecutew)
	/// method.
	fn ShellExecute(&self,
		operation: &str,
		file: &str,
		parameters: Option<&str>,
		directory: Option<&str>,
		show_cmd: co::SW,
	) -> Result<HINSTANCE, co::SE_ERR>
	{
		let ret = unsafe {
			shell::ffi::ShellExecuteW(
				self.as_ptr(),
				WString::from_str(operation).as_ptr(),
				WString::from_str(file).as_ptr(),
				parameters.map_or(std::ptr::null(), |lp| WString::from_str(lp).as_ptr()),
				directory.map_or(std::ptr::null(), |lp| WString::from_str(lp).as_ptr()),
				show_cmd.0,
			)
		};

		if ret <= 32 as _ {
			Err(co::SE_ERR(ret as _))
		} else {
			Ok(unsafe { HINSTANCE::from_ptr(ret as _) })
		}
	}
}
