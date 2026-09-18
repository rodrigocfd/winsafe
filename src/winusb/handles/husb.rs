#![allow(non_snake_case)]

use crate::decl::*;
use crate::kernel::privs::*;
use crate::macros::*;
use crate::winusb::ffi;

handle! { HUSB;
	/// Handles to an
	/// [USB device](https://learn.microsoft.com/en-us/windows/win32/api/winusb/nf-winusb-winusb_initialize).
	/// Originally `WINUSB_INTERFACE_HANDLE`.
}

impl HUSB {
	/// [`WinUsb_FlushPipe`](https://learn.microsoft.com/en-us/windows/win32/api/winusb/nf-winusb-winusb_flushpipe)
	/// function.
	pub fn WinUsb_FlushPipe(&self, pipe_id: u8) -> SysResult<()> {
		BoolRet(unsafe { ffi::WinUsb_FlushPipe(self.ptr(), pipe_id) }).to_sysresult()
	}

	/// [`WinUsb_GetCurrentAlternateSetting`](https://learn.microsoft.com/en-us/windows/win32/api/winusb/nf-winusb-winusb_getcurrentalternatesetting)
	/// function.
	#[must_use]
	pub fn WinUsb_GetCurrentAlternateSetting(&self) -> SysResult<u8> {
		let mut setting_no = 0u8;
		BoolRet(unsafe { ffi::WinUsb_GetCurrentAlternateSetting(self.ptr(), &mut setting_no) })
			.to_sysresult()
			.map(|_| setting_no)
	}

	/// [`WinUsb_SetCurrentAlternateSetting`](https://learn.microsoft.com/en-us/windows/win32/api/winusb/nf-winusb-winusb_setcurrentalternatesetting)
	/// function.
	pub fn WinUsb_SetCurrentAlternateSetting(&self, setting_no: u8) -> SysResult<()> {
		BoolRet(unsafe { ffi::WinUsb_SetCurrentAlternateSetting(self.ptr(), setting_no) })
			.to_sysresult()
	}
}
