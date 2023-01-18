use std::ops::Deref;

use crate::{co, gdi};
use crate::prelude::{gdi_Hdc, GdiObject, Handle};
use crate::user::decl::HDC;

/// RAII implementation for a [`GdiObject`](crate::prelude::GdiObject) which
/// automatically calls
/// [`DeleteObject`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-deleteobject)
/// when the object goes out of scope.
pub struct GdiObjectGuard<T>
	where T: GdiObject,
{
	pub(crate) handle: T,
}

impl<T> Drop for GdiObjectGuard<T>
	where T: GdiObject,
{
	fn drop(&mut self) {
		if let Some(h) = self.handle.as_opt() {
			unsafe { gdi::ffi::DeleteObject(h.as_ptr()); } // ignore errors
		}
	}
}

impl<T> Deref for GdiObjectGuard<T>
	where T: GdiObject,
{
	type Target = T;

	fn deref(&self) -> &Self::Target {
		&self.handle
	}
}

handle_guard! { HdcDeleteGuard: HDC;
	gdi::ffi::DeleteDC;
	/// RAII implementation for [`HDC`](crate::HDC) which automatically calls
	/// [`DeleteDC`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-deletedc)
	/// when the object goes out of scope.
}

/// RAII implementation for
/// [`HDC::SelectObject`](crate::prelude::gdi_Hdc::SelectObject) calls, which
/// automatically selects the previous GDI object at the end of the scope.
pub struct SelectObjectGuard<'a, H, G>
	where H: gdi_Hdc,
		G: GdiObject,
{
	pub(crate) hdc: &'a H,
	pub(crate) prev_hgdi: G,
	pub(crate) region: Option<co::REGION>,
}

impl<'a, H, G> Drop for SelectObjectGuard<'a, H, G>
	where H: gdi_Hdc,
		G: GdiObject,
{
	fn drop(&mut self) {
		if let Some(h) = self.hdc.as_opt() {
			if let Some(g) = self.prev_hgdi.as_opt() {
				unsafe { gdi::ffi::SelectObject(h.as_ptr(), g.as_ptr()); } // ignore errors
			}
		}
	}
}

impl<'a, H, G> SelectObjectGuard<'a, H, G>
	where H: gdi_Hdc,
		G: GdiObject,
{
	/// Returns a handle to the object that has been replaced.
	pub const fn prev_object(&self) -> &G {
		&self.prev_hgdi
	}

	/// Returns the region information returned by the source
	/// [`HDC::SelectObject`](crate::prelude::gdi_Hdc::SelectObject) call, if
	/// the [`GdiObject`](crate::prelude::GdiObject) was an
	/// [`HRGN`](crate::HRGN); otherwise returns `None`.
	pub const fn region(&self) -> Option<co::REGION> {
		self.region
	}
}
