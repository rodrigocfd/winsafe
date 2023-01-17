#![allow(non_camel_case_types, non_snake_case)]

use std::any::Any;
use std::ops::Deref;

use crate::gdi;
use crate::prelude::Handle;

/// This trait is enabled with the `gdi` feature, and implements methods for any
/// [`HGDIOBJ`](https://learn.microsoft.com/en-us/windows/win32/winprog/windows-data-types#hgdiobj)
/// handle, which is the base handle for
/// [GDI objects](https://learn.microsoft.com/en-us/windows/win32/sysinfo/gdi-objects).
///
/// Prefer importing this trait through the prelude:
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// ```
pub trait GdiObject: Handle + Any {}

//------------------------------------------------------------------------------

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
