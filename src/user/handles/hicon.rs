#![allow(non_camel_case_types, non_snake_case)]

use crate::decl::*;
use crate::guard::*;
use crate::kernel::privs::*;
use crate::user::ffi;

handle! { HICON;
	/// Handle to an
	/// [icon](https://learn.microsoft.com/en-us/windows/win32/winprog/windows-data-types#hicon).
}

impl HICON {
	/// [`CopyIcon`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-copyicon)
	/// function.
	#[must_use]
	pub fn CopyIcon(&self) -> SysResult<DestroyIconGuard> {
		unsafe {
			ptr_to_sysresult_handle(ffi::CopyIcon(self.ptr())).map(|h| DestroyIconGuard::new(h))
		}
	}

	/// [`GetIconInfo`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-geticoninfo)
	/// function.
	#[must_use]
	pub fn GetIconInfo(&self) -> SysResult<ICONINFO> {
		let mut ii = ICONINFO::default();
		bool_to_sysresult(unsafe { ffi::GetIconInfo(self.ptr(), pvoid(&mut ii)) }).map(|_| ii)
	}

	/// [`GetIconInfoEx`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-geticoninfoexw)
	/// function.
	pub fn GetIconInfoEx(&self, icon_info: &mut ICONINFOEX) -> SysResult<()> {
		bool_to_sysresult(unsafe { ffi::GetIconInfoExW(self.ptr(), pvoid(icon_info)) })
	}
}
