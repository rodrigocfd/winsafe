#![allow(non_snake_case)]

use crate::decl::*;
use crate::kernel::privs::*;
use crate::macros::*;
use crate::winusb::ffi;

handle! { HUSB;
	/// Handles to an
	/// [USB device](https://learn.microsoft.com/en-us/windows/win32/api/winusb/nf-winusb-winusb_initialize).
	/// Originally `WINUSB_INTERFACE_HANDLE`.
	///
	/// Usually created with
	/// [`HFILE::WinUsbInitialize`](crate::HFILE::WinUsbInitialize) method.
}

impl HUSB {
	/// [`WinUsb_AbortPipe`](https://learn.microsoft.com/en-us/windows/win32/api/winusb/nf-winusb-winusb_abortpipe)
	/// function.
	pub fn AbortPipe(&self, pipe_id: u8) -> SysResult<()> {
		BoolRet(unsafe { ffi::WinUsb_AbortPipe(self.ptr(), pipe_id) }).to_sysresult()
	}

	/// [`WinUsb_FlushPipe`](https://learn.microsoft.com/en-us/windows/win32/api/winusb/nf-winusb-winusb_flushpipe)
	/// function.
	pub fn FlushPipe(&self, pipe_id: u8) -> SysResult<()> {
		BoolRet(unsafe { ffi::WinUsb_FlushPipe(self.ptr(), pipe_id) }).to_sysresult()
	}

	/// [`WinUsb_GetCurrentAlternateSetting`](https://learn.microsoft.com/en-us/windows/win32/api/winusb/nf-winusb-winusb_getcurrentalternatesetting)
	/// function.
	#[must_use]
	pub fn GetCurrentAlternateSetting(&self) -> SysResult<u8> {
		let mut setting_no = 0u8;
		BoolRet(unsafe { ffi::WinUsb_GetCurrentAlternateSetting(self.ptr(), &mut setting_no) })
			.to_sysresult()
			.map(|_| setting_no)
	}

	/// [`WinUsb_GetCurrentFrameNumber`](https://learn.microsoft.com/en-us/windows/win32/api/winusb/nf-winusb-winusb_getcurrentframenumber)
	/// function.
	///
	/// Returns the current frame number and the timestamp, respectively.
	#[must_use]
	pub fn GetCurrentFrameNumber(&self) -> SysResult<(u32, i64)> {
		let mut current_frame_no = 0u32;
		let mut timestamp = 0i64;

		BoolRet(unsafe {
			ffi::WinUsb_GetCurrentFrameNumber(self.ptr(), &mut current_frame_no, &mut timestamp)
		})
		.to_sysresult()
		.map(|_| (current_frame_no, timestamp))
	}

	/// [`WinUsb_SetCurrentAlternateSetting`](https://learn.microsoft.com/en-us/windows/win32/api/winusb/nf-winusb-winusb_setcurrentalternatesetting)
	/// function.
	pub fn SetCurrentAlternateSetting(&self, setting_no: u8) -> SysResult<()> {
		BoolRet(unsafe { ffi::WinUsb_SetCurrentAlternateSetting(self.ptr(), setting_no) })
			.to_sysresult()
	}
}
