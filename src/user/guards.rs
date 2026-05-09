use std::ops::{Deref, DerefMut};

use crate::decl::*;
use crate::kernel::privs::*;
use crate::macros::*;
use crate::prelude::*;
use crate::user::ffi;

/// RAII implementation for clipboard which automatically calls
/// [`CloseClipboard`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-closeclipboard)
/// when the object goes out of scope.
pub struct CloseClipboardGuard {
	hwnd: HWND,
	hclip: HCLIPBOARD,
}

impl Drop for CloseClipboardGuard {
	fn drop(&mut self) {
		unsafe { ffi::CloseClipboard() }; // ignore errors
	}
}

impl Deref for CloseClipboardGuard {
	type Target = HCLIPBOARD;

	fn deref(&self) -> &Self::Target {
		&self.hclip
	}
}
impl DerefMut for CloseClipboardGuard {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.hclip
	}
}

impl CloseClipboardGuard {
	/// Constructs the guard by taking ownership of the handle.
	///
	/// # Safety
	///
	/// Be sure you must call
	/// [`CloseClipboard`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-closeclipboard)
	/// at the end of scope.
	#[must_use]
	pub unsafe fn new(hwnd: &HWND, hclip: HCLIPBOARD) -> Self {
		Self { hwnd: unsafe { hwnd.raw_copy() }, hclip }
	}

	/// Returns a handle to the window associated with the open clipboard.
	#[must_use]
	pub const fn hwnd(&self) -> &HWND {
		&self.hwnd
	}
}

handle_guard! { CloseDesktopGuard: HDESK;
	ffi::CloseDesktop;
	/// RAII implementation for [`HDESK`](crate::decl::HDESK) which
	/// automatically calls
	/// [`CloseDesktop`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-closedesktop)
	/// when the object goes out of scope.
}

handle_guard! { DestroyAcceleratorTableGuard: HACCEL;
	ffi::DestroyAcceleratorTable;
	/// RAII implementation for [`HACCEL`](crate::decl::HACCEL) which
	/// automatically calls
	/// [`DestroyAcceleratorTable`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-destroyacceleratortable)
	/// when the object goes out of scope.
}

handle_guard! { DestroyCursorGuard: HCURSOR;
	ffi::DestroyCursor;
	/// RAII implementation for [`HCURSOR`](crate::decl::HCURSOR) which
	/// automatically calls
	/// [`DestroyCursor`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-destroycursor)
	/// when the object goes out of scope.
}

handle_guard! { DestroyIconGuard: HICON;
	ffi::DestroyIcon;
	/// RAII implementation for [`HICON`](crate::decl::HICON) which
	/// automatically calls
	/// [`DestroyIcon`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-destroyicon)
	/// when the object goes out of scope.
}

handle_guard! { DestroyMenuGuard: HMENU;
	ffi::DestroyMenu;
	/// RAII implementation for [`HMENU`](crate::decl::HMENU) which
	/// automatically calls
	/// [`DestroyMenu`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-destroymenu)
	/// when the object goes out of scope.
}

handle_guard! { EndDeferWindowPosGuard: HDWP;
	ffi::EndDeferWindowPos;
	/// RAII implementation for [`HDWP`](crate::decl::HDWP) which automatically
	/// calls
	/// [`EndDeferWindowPos`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-enddeferwindowpos)
	/// when the object goes out of scope.
}

/// RAII implementation for [`HDC`] which automatically calls
/// [`EndPaint`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-endpaint)
/// when the object goes out of scope.
///
/// The [`PAINTSTRUCT`] object is stored internally, and can be accessed
/// through the [`paintstruct`](crate::guard::EndPaintGuard::paintstruct)
/// method.
pub struct EndPaintGuard {
	hwnd: HWND,
	hdc: HDC,
	ps: PAINTSTRUCT,
}

impl Drop for EndPaintGuard {
	fn drop(&mut self) {
		unsafe { ffi::EndPaint(self.hwnd.ptr(), pcvoid(&self.ps)) };
	}
}

impl Deref for EndPaintGuard {
	type Target = HDC;

	fn deref(&self) -> &Self::Target {
		&self.hdc
	}
}
impl DerefMut for EndPaintGuard {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.hdc
	}
}

impl EndPaintGuard {
	/// Constructs the guard by taking ownership of the objects.
	///
	/// # Safety
	///
	/// Be sure you must call
	/// [`EndPaint`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-endpaint)
	/// at the end of scope.
	#[must_use]
	pub unsafe fn new(hwnd: &HWND, hdc: HDC, ps: PAINTSTRUCT) -> Self {
		Self {
			hwnd: unsafe { hwnd.raw_copy() },
			hdc,
			ps,
		}
	}

	/// Returns a reference to the internal [`PAINTSTRUCT`] object.
	#[must_use]
	pub const fn paintstruct(&self) -> &PAINTSTRUCT {
		&self.ps
	}

	/// Returns a handle to the window being painted.
	#[must_use]
	pub const fn hwnd(&self) -> &HWND {
		&self.hwnd
	}
}

/// RAII implementation for [`HWND`] which automatically calls
/// [`ReleaseCapture`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-releasecapture)
/// when the object goes out of scope.
pub struct ReleaseCaptureGuard {
	hwnd: HWND,
	hwnd_prev: Option<HWND>,
}

impl Drop for ReleaseCaptureGuard {
	fn drop(&mut self) {
		unsafe { ffi::ReleaseCapture() }; // ignore errors
	}
}

impl ReleaseCaptureGuard {
	/// Constructs the guard by taking ownership of the handles.
	///
	/// # Safety
	///
	/// Be sure you must call
	/// [`ReleaseCapture`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-releasecapture)
	/// at the end of scope.
	#[must_use]
	pub unsafe fn new(hwnd: &HWND, hwnd_prev: Option<HWND>) -> Self {
		Self {
			hwnd: unsafe { hwnd.raw_copy() },
			hwnd_prev,
		}
	}

	/// Returns a handle to the window which is currently capturing the mouse.
	#[must_use]
	pub const fn hwnd(&self) -> &HWND {
		return &self.hwnd;
	}

	/// Returns a handle to the window that had previously captured the mouse,
	/// if any.
	#[must_use]
	pub const fn prev_hwnd(&self) -> Option<&HWND> {
		self.hwnd_prev.as_ref()
	}
}

/// RAII implementation for [`HDC`] which automatically calls
/// [`ReleaseDC`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-releasedc)
/// when the object goes out of scope.
pub struct ReleaseDCGuard {
	hwnd: HWND,
	hdc: HDC,
}

impl Drop for ReleaseDCGuard {
	fn drop(&mut self) {
		if let Some(h) = self.hwnd.as_opt() {
			if let Some(dc) = self.hdc.as_opt() {
				unsafe { ffi::ReleaseDC(h.ptr(), dc.ptr()) }; // ignore errors
			}
		}
	}
}

impl Deref for ReleaseDCGuard {
	type Target = HDC;

	fn deref(&self) -> &Self::Target {
		&self.hdc
	}
}
impl DerefMut for ReleaseDCGuard {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.hdc
	}
}

impl ReleaseDCGuard {
	/// Constructs the guard by taking ownership of the handles.
	///
	/// # Safety
	///
	/// Be sure the handle must be freed with
	/// [`ReleaseDC`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-releasedc)
	/// at the end of scope.
	#[must_use]
	pub unsafe fn new(hwnd: &HWND, hdc: HDC) -> Self {
		Self { hwnd: unsafe { hwnd.raw_copy() }, hdc }
	}

	/// Ejects the underlying handle, leaving a [`Handle::INVALID`] in its
	/// place.
	///
	/// Since the internal handle will be invalidated, the destructor will not
	/// run. It's your responsability to run it, otherwise you'll cause a
	/// resource leak.
	#[must_use]
	pub fn leak(&mut self) -> HDC {
		std::mem::replace(&mut self.hdc, HDC::INVALID)
	}

	/// Returns a handle to the window which got the DC.
	#[must_use]
	pub const fn hwnd(&self) -> &HWND {
		&self.hwnd
	}
}
