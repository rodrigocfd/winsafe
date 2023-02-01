use std::marker::PhantomData;
use std::ops::Deref;

use crate::prelude::{Handle, user_Hwnd};
use crate::user;
use crate::user::decl::{HDC, HDWP, HWND, PAINTSTRUCT};

/// RAII implementation for clipboard which automatically calls
/// [`CloseClipboard`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-closeclipboard)
/// when the object goes out of scope.
pub struct ClipboardGuard<'a> {
	pub(crate) _hwnd: PhantomData<&'a ()>,
}

impl<'a> Drop for ClipboardGuard<'a> {
	fn drop(&mut self) {
		unsafe { user::ffi::CloseClipboard(); } // ignore errors
	}
}

/// RAII implementation for [`HDC`](crate::HDC) which automatically calls
/// [`EndPaint`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-endpaint)
/// when the object goes out of scope.
///
/// The [`PAINTSTRUCT`](crate::PAINTSTRUCT) object is stored internally, and can
/// be accessed through the
/// [`paintstruct`](crate::guard::HdcPaintGuard::paintstruct) method.
pub struct HdcPaintGuard<'a, H>
	where H: user_Hwnd,
{
	pub(crate) hwnd: &'a H,
	pub(crate) hdc: HDC,
	pub(crate) ps: PAINTSTRUCT,
}

impl<'a, H> Drop for HdcPaintGuard<'a, H>
	where H: user_Hwnd,
{
	fn drop(&mut self) {
		unsafe {
			user::ffi::EndPaint(self.hwnd.as_ptr(), &self.ps as *const _ as _);
		}
	}
}

impl<'a, H> Deref for HdcPaintGuard<'a, H>
	where H: user_Hwnd,
{
	type Target = HDC;

	fn deref(&self) -> &Self::Target {
		&self.hdc
	}
}

impl<'a, H> HdcPaintGuard<'a, H>
	where H: user_Hwnd,
{
	/// Returns a reference to the internal [`PAINTSTRUCT`](crate::PAINTSTRUCT)
	/// object.
	#[must_use]
	pub const fn paintstruct(&self) -> &PAINTSTRUCT {
		&self.ps
	}
}

/// RAII implementation for [`HDC`](crate::HDC) which automatically calls
/// [`ReleaseDC`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-releasedc)
/// when the object goes out of scope.
pub struct HdcReleaseGuard<'a, H>
	where H: user_Hwnd,
{
	pub(crate) hwnd: &'a H,
	pub(crate) hdc: HDC,
}

impl<'a, H> Drop for HdcReleaseGuard<'a, H>
	where H: user_Hwnd,
{
	fn drop(&mut self) {
		if let Some(h) = self.hwnd.as_opt() {
			if let Some(dc) = self.hdc.as_opt() {
				unsafe { user::ffi::ReleaseDC(h.as_ptr(), dc.as_ptr()); } // ignore errors
			}
		}
	}
}

impl<'a, H> Deref for HdcReleaseGuard<'a, H>
	where H: user_Hwnd,
{
	type Target = HDC;

	fn deref(&self) -> &Self::Target {
		&self.hdc
	}
}

impl<'a, H> HdcReleaseGuard<'a, H>
	where H: user_Hwnd,
{
	/// Ejects the underlying handle, leaving a
	/// [`Handle::INVALID`](crate::prelude::Handle::INVALID) in its place.
	///
	/// # Safety
	///
	/// Since the internal handle will be invalidated, the destructor will not
	/// run. It's your responsability to run it, otherwise you'll cause a
	/// resource leak.
	#[must_use]
	pub unsafe fn leak(&mut self) -> HDC {
		std::mem::replace(&mut self.hdc, HDC::INVALID)
	}
}

handle_guard! { HdwpGuard: HDWP;
	user::ffi::EndDeferWindowPos;
	/// RAII implementation for [`HDWP`](crate::HDWP) which automatically calls
	/// [`EndDeferWindowPos`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-enddeferwindowpos)
	/// when the object goes out of scope.
}

/// RAII implementation for [`HWND`](crate::HWND) which automatically calls
/// [`ReleaseCapture`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-releasecapture)
/// when the object goes out of scope.
pub struct HwndCaptureGuard<'a, H>
	where H: user_Hwnd,
{
	pub(crate) _hwnd: &'a H,
	pub(crate) hwnd_prev: Option<HWND>,
}

impl<'a, H> Drop for HwndCaptureGuard<'a, H>
	where H: user_Hwnd,
{
	fn drop(&mut self) {
		unsafe { user::ffi::ReleaseCapture(); } // ignore errors
	}
}

impl<'a, H> HwndCaptureGuard<'a, H>
	where H: user_Hwnd,
{
	/// Returns a handle to the window that had previously captured the mouse,
	/// if any.
	#[must_use]
	pub const fn prev_hwnd(&self) -> Option<&HWND> {
		self.hwnd_prev.as_ref()
	}
}
