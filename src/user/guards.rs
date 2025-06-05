use std::ops::{Deref, DerefMut};

use crate::decl::*;
use crate::kernel::privs::*;
use crate::prelude::*;
use crate::user::ffi;

/// RAII implementation for clipboard which automatically calls
/// [`CloseClipboard`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-closeclipboard)
/// when the object goes out of scope.
pub struct CloseClipboardGuard<'a> {
	_hwnd: &'a HWND,
	hclip: HCLIPBOARD,
}

impl<'a> Drop for CloseClipboardGuard<'a> {
	fn drop(&mut self) {
		unsafe {
			ffi::CloseClipboard(); // ignore errors
		}
	}
}

impl<'a> Deref for CloseClipboardGuard<'a> {
	type Target = HCLIPBOARD;

	fn deref(&self) -> &Self::Target {
		&self.hclip
	}
}
impl<'a> DerefMut for CloseClipboardGuard<'a> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.hclip
	}
}

impl<'a> CloseClipboardGuard<'a> {
	/// Constructs the guard by taking ownership of the handle.
	///
	/// # Safety
	///
	/// Be sure you must call
	/// [`CloseClipboard`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-closeclipboard)
	/// at the end of scope.
	#[must_use]
	pub const unsafe fn new(hwnd: &'a HWND, hclip: HCLIPBOARD) -> Self {
		Self { _hwnd: hwnd, hclip }
	}
}

handle_guard! { CloseDesktopGuard: HDESK;
	ffi::CloseDesktop;
	/// RAII implementation for [`HDESK`](crate::HDESK) which automatically
	/// calls
	/// [`CloseDesktop`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-closedesktop)
	/// when the object goes out of scope.
}

handle_guard! { DestroyAcceleratorTableGuard: HACCEL;
	ffi::DestroyAcceleratorTable;
	/// RAII implementation for [`HACCEL`](crate::HACCEL) which automatically
	/// calls
	/// [`DestroyAcceleratorTable`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-destroyacceleratortable)
	/// when the object goes out of scope.
}

handle_guard! { DestroyCursorGuard: HCURSOR;
	ffi::DestroyCursor;
	/// RAII implementation for [`HCURSOR`](crate::HCURSOR) which automatically
	/// calls
	/// [`DestroyCursor`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-destroycursor)
	/// when the object goes out of scope.
}

handle_guard! { DestroyIconGuard: HICON;
	ffi::DestroyIcon;
	/// RAII implementation for [`HICON`](crate::HICON) which automatically
	/// calls
	/// [`DestroyIcon`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-destroyicon)
	/// when the object goes out of scope.
}

handle_guard! { DestroyMenuGuard: HMENU;
	ffi::DestroyMenu;
	/// RAII implementation for [`HMENU`](crate::HMENU) which automatically
	/// calls
	/// [`DestroyMenu`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-destroymenu)
	/// when the object goes out of scope.
}

handle_guard! { EndDeferWindowPosGuard: HDWP;
	ffi::EndDeferWindowPos;
	/// RAII implementation for [`HDWP`](crate::HDWP) which automatically calls
	/// [`EndDeferWindowPos`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-enddeferwindowpos)
	/// when the object goes out of scope.
}

/// RAII implementation for [`HDC`](crate::HDC) which automatically calls
/// [`EndPaint`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-endpaint)
/// when the object goes out of scope.
///
/// The [`PAINTSTRUCT`](crate::PAINTSTRUCT) object is stored internally, and can
/// be accessed through the
/// [`paintstruct`](crate::guard::EndPaintGuard::paintstruct) method.
pub struct EndPaintGuard<'a> {
	hwnd: &'a HWND,
	hdc: HDC,
	ps: PAINTSTRUCT,
}

impl<'a> Drop for EndPaintGuard<'a> {
	fn drop(&mut self) {
		unsafe {
			ffi::EndPaint(self.hwnd.ptr(), pcvoid(&self.ps));
		}
	}
}

impl<'a> Deref for EndPaintGuard<'a> {
	type Target = HDC;

	fn deref(&self) -> &Self::Target {
		&self.hdc
	}
}

impl<'a> DerefMut for EndPaintGuard<'a> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.hdc
	}
}

impl<'a> EndPaintGuard<'a> {
	/// Constructs the guard by taking ownership of the objects.
	///
	/// # Safety
	///
	/// Be sure you must call
	/// [`EndPaint`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-endpaint)
	/// at the end of scope.
	#[must_use]
	pub const unsafe fn new(hwnd: &'a HWND, hdc: HDC, ps: PAINTSTRUCT) -> Self {
		Self { hwnd, hdc, ps }
	}

	/// Returns a reference to the internal [`PAINTSTRUCT`](crate::PAINTSTRUCT)
	/// object.
	#[must_use]
	pub const fn paintstruct(&self) -> &PAINTSTRUCT {
		&self.ps
	}
}

/// RAII implementation for [`HWND`](crate::HWND) which automatically calls
/// [`ReleaseCapture`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-releasecapture)
/// when the object goes out of scope.
pub struct ReleaseCaptureGuard<'a> {
	_hwnd: &'a HWND,
	hwnd_prev: Option<HWND>,
}

impl<'a> Drop for ReleaseCaptureGuard<'a> {
	fn drop(&mut self) {
		unsafe {
			ffi::ReleaseCapture(); // ignore errors
		}
	}
}

impl<'a> ReleaseCaptureGuard<'a> {
	/// Constructs the guard by taking ownership of the handles.
	///
	/// # Safety
	///
	/// Be sure you must call
	/// [`ReleaseCapture`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-releasecapture)
	/// at the end of scope.
	#[must_use]
	pub const unsafe fn new(hwnd: &'a HWND, hwnd_prev: Option<HWND>) -> Self {
		Self { _hwnd: hwnd, hwnd_prev }
	}

	/// Returns a handle to the window that had previously captured the mouse,
	/// if any.
	#[must_use]
	pub const fn prev_hwnd(&self) -> Option<&HWND> {
		self.hwnd_prev.as_ref()
	}
}

/// RAII implementation for [`HDC`](crate::HDC) which automatically calls
/// [`ReleaseDC`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-releasedc)
/// when the object goes out of scope.
pub struct ReleaseDCGuard<'a> {
	hwnd: &'a HWND,
	hdc: HDC,
}

impl<'a> Drop for ReleaseDCGuard<'a> {
	fn drop(&mut self) {
		if let Some(h) = self.hwnd.as_opt() {
			if let Some(dc) = self.hdc.as_opt() {
				unsafe {
					ffi::ReleaseDC(h.ptr(), dc.ptr()); // ignore errors
				}
			}
		}
	}
}

impl<'a> Deref for ReleaseDCGuard<'a> {
	type Target = HDC;

	fn deref(&self) -> &Self::Target {
		&self.hdc
	}
}

impl<'a> DerefMut for ReleaseDCGuard<'a> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.hdc
	}
}

impl<'a> ReleaseDCGuard<'a> {
	/// Constructs the guard by taking ownership of the handles.
	///
	/// # Safety
	///
	/// Be sure the handle must be freed with
	/// [`ReleaseDC`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-releasedc)
	/// at the end of scope.
	#[must_use]
	pub const unsafe fn new(hwnd: &'a HWND, hdc: HDC) -> Self {
		Self { hwnd, hdc }
	}

	/// Ejects the underlying handle, leaving a
	/// [`Handle::INVALID`](crate::prelude::Handle::INVALID) in its place.
	///
	/// Since the internal handle will be invalidated, the destructor will not
	/// run. It's your responsability to run it, otherwise you'll cause a
	/// resource leak.
	#[must_use]
	pub fn leak(&mut self) -> HDC {
		std::mem::replace(&mut self.hdc, HDC::INVALID)
	}
}
