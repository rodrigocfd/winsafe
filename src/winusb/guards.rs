use crate::decl::*;
use crate::macros::*;
use crate::prelude::*;
use crate::winusb::ffi;

/// RAII implementation for
/// [`USB_CONFIGURATION_DESCRIPTOR`](crate::USB_CONFIGURATION_DESCRIPTOR) which
/// automatically releases the allocated memory.
pub struct UsbConfiguratorDescriptorGuard {
	data: Vec<u8>,
}

impl UsbConfiguratorDescriptorGuard {
	#[must_use]
	pub(in crate::winusb) fn new(num_bytes: usize) -> Self {
		Self { data: vec![0; num_bytes] }
	}

	#[must_use]
	pub(in crate::winusb) const fn as_mut_ptr(&mut self) -> *mut u8 {
		self.data.as_mut_ptr()
	}

	/// Returns a reference to the
	/// [`USB_CONFIGURATION_DESCRIPTOR`](crate::USB_CONFIGURATION_DESCRIPTOR)
	/// header.
	#[must_use]
	pub const fn header(&self) -> &USB_CONFIGURATION_DESCRIPTOR {
		unsafe { &*(self as *const Self as *const _) }
	}
}

handle_guard! { WinUsbFreeGuard: HUSB;
	ffi::WinUsb_Free;
	/// RAII implementation for [`HUSB`](crate::HUSB) which automatically calls
	/// [`WinUsb_Free`](https://learn.microsoft.com/en-us/windows/win32/api/winusb/nf-winusb-winusb_free)
	/// when the object goes out of scope.
}
