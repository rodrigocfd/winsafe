#![allow(non_snake_case)]

use crate::co;
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

	/// [`WinUsb_GetDescriptor`](https://learn.microsoft.com/en-us/windows/win32/api/winusb/nf-winusb-winusb_getdescriptor)
	/// function.
	///
	/// Note that, currently, not all descriptors are implemented. You can query
	/// only those defined in [`UsbDescr`](crate::UsbDescr) enum, otherwise this
	/// method will return
	/// [`ERROR::INVALID_PARAMETER`](crate::co::ERROR::INVALID_PARAMETER).
	#[must_use]
	pub fn GetDescriptor(
		&self,
		descriptor_type: co::USB_DESCRIPTOR_TYPE,
		index: u8,
		lang_id: LANGID,
	) -> SysResult<UsbDescr> {
		let mut bytes_read = 0u32;
		match descriptor_type {
			co::USB_DESCRIPTOR_TYPE::DEVICE => {
				let mut obj = USB_DEVICE_DESCRIPTOR::default();
				let ret = BoolRet(unsafe {
					ffi::WinUsb_GetDescriptor(
						self.ptr(),
						descriptor_type.raw(),
						index,
						lang_id.raw(),
						&mut obj as *mut _ as _,
						std::mem::size_of::<USB_DEVICE_DESCRIPTOR>() as _,
						&mut bytes_read,
					)
				})
				.to_sysresult()
				.map(|_| UsbDescr::Device(obj));

				if bytes_read == std::mem::size_of::<USB_DEVICE_DESCRIPTOR>() as _ {
					ret
				} else {
					Err(co::ERROR::INCORRECT_SIZE)
				}
			},
			co::USB_DESCRIPTOR_TYPE::CONFIGURATION => {
				let mut obj = USB_CONFIGURATION_DESCRIPTOR::default();
				let ret = BoolRet(unsafe {
					ffi::WinUsb_GetDescriptor(
						self.ptr(),
						descriptor_type.raw(),
						index,
						lang_id.raw(),
						&mut obj as *mut _ as _,
						std::mem::size_of::<USB_CONFIGURATION_DESCRIPTOR>() as _,
						&mut bytes_read,
					)
				})
				.to_sysresult();

				if ret.is_err() {
					return Err(ret.unwrap_err());
				} else if bytes_read != std::mem::size_of::<USB_CONFIGURATION_DESCRIPTOR>() as _ {
					return Err(co::ERROR::INCORRECT_SIZE);
				}

				let mut obj2 = UsbConfiguratorDescriptorGuard::new(obj.wTotalLength as _);
				let ret2 = BoolRet(unsafe {
					ffi::WinUsb_GetDescriptor(
						self.ptr(),
						descriptor_type.raw(),
						index,
						lang_id.raw(),
						obj2.as_mut_ptr(),
						obj.wTotalLength as _,
						&mut bytes_read,
					)
				})
				.to_sysresult()
				.map(|_| UsbDescr::Configuration(obj2));

				if bytes_read == std::mem::size_of::<USB_DEVICE_DESCRIPTOR>() as _ {
					ret2
				} else {
					Err(co::ERROR::INCORRECT_SIZE)
				}
			},
			_ => Err(co::ERROR::INVALID_PARAMETER),
		}
	}

	/// [`WinUsb_ResetPipe`](https://learn.microsoft.com/en-us/windows/win32/api/winusb/nf-winusb-winusb_resetpipe)
	/// function.
	pub fn ResetPipe(&self, pipe_id: u8) -> SysResult<()> {
		BoolRet(unsafe { ffi::WinUsb_ResetPipe(self.ptr(), pipe_id) }).to_sysresult()
	}

	/// [`WinUsb_SetCurrentAlternateSetting`](https://learn.microsoft.com/en-us/windows/win32/api/winusb/nf-winusb-winusb_setcurrentalternatesetting)
	/// function.
	pub fn SetCurrentAlternateSetting(&self, setting_no: u8) -> SysResult<()> {
		BoolRet(unsafe { ffi::WinUsb_SetCurrentAlternateSetting(self.ptr(), setting_no) })
			.to_sysresult()
	}
}
