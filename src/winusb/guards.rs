use crate::decl::*;
use crate::macros::*;
use crate::prelude::*;
use crate::winusb::ffi;

handle_guard! { WinUsbFreeGuard: HUSB;
	ffi::WinUsb_Free;
	/// RAII implementation for [`HUSB`](crate::HUSB) which automatically calls
	/// [`WinUsb_Free`](https://learn.microsoft.com/en-us/windows/win32/api/winusb/nf-winusb-winusb_free)
	/// when the object goes out of scope.
}
