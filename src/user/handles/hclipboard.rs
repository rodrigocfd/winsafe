#![allow(non_camel_case_types, non_snake_case)]

use crate::co;
use crate::decl::*;
use crate::kernel::privs::*;
use crate::user::ffi;

handle! { HCLIPBOARD;
	/// Handle to the
	/// [clipboard](https://learn.microsoft.com/en-us/windows/win32/dataxchg/about-the-clipboard).
	///
	/// This handle doesn't exist natively, it's just an abstraction to safely
	/// group the related clipboard operations.
}

impl HCLIPBOARD {
	/// [`EmptyClipboard`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-emptyclipboard)
	/// function.
	pub fn EmptyClipboard(&self) -> SysResult<()> {
		bool_to_sysresult(unsafe { ffi::EmptyClipboard() })
	}

	/// [`GetClipboardData`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getclipboarddata)
	/// function.
	///
	/// Calls [`HGLOBAL::GlobalSize`](crate::HGLOBAL::GlobalSize) and
	/// [`HGLOBAL::GlobalLock`](crate::HGLOBAL::GlobalLock) internally to
	/// retrieve a copy of the raw clipboard data.
	///
	/// Note that you should not trust the clipboard format – the binary data
	/// can be anything, despite what the format says. Be careful when parsing
	/// the binary into your desired format.
	///
	/// # Examples
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*, co};
	///
	/// let hclip = w::HWND::NULL.OpenClipboard()?;
	/// let data = hclip.GetClipboardData(co::CF::TEXT)?;
	/// # w::SysResult::Ok(())
	/// ```
	#[must_use]
	pub fn GetClipboardData(&self, format: co::CF) -> SysResult<Vec<u8>> {
		let hglobal = unsafe {
			ptr_to_sysresult(ffi::GetClipboardData(format.raw() as _))
				.map(|p| HGLOBAL::from_ptr(p))?
		};
		let copied = {
			let block = hglobal.GlobalLock()?;
			block.as_slice().to_vec()
		};
		Ok(copied)
	}

	/// [`GetClipboardSequenceNumber`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getclipboardsequencenumber)
	/// function.
	#[must_use]
	pub fn GetClipboardSequenceNumber(&self) -> u32 {
		unsafe { ffi::GetClipboardSequenceNumber() }
	}

	/// [`SetClipboardData`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setclipboarddata)
	/// function.
	///
	/// Calls [`HGLOBAL::GlobalAlloc`](crate::HGLOBAL::GlobalAlloc) and
	/// [`HGLOBAL::GlobalLock`](crate::HGLOBAL::GlobalLock) internally before
	/// copying the data into the clipboard.
	///
	/// # Examples
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*, co};
	///
	/// let hclip = w::HWND::NULL.OpenClipboard()?;
	///
	/// let str_nullt = "foo"
	///     .as_bytes()
	///     .iter()
	///     .map(|ch| *ch)
	///     .chain(std::iter::once(0)) // null-terminated
	///     .collect::<Vec<_>>();
	///
	/// hclip.SetClipboardData(co::CF::TEXT, &str_nullt)?;
	/// # w::SysResult::Ok(())
	/// ```
	pub fn SetClipboardData(&self, format: co::CF, data: &[u8]) -> SysResult<()> {
		let mut hglobal = HGLOBAL::GlobalAlloc(co::GMEM::MOVEABLE, data.len())?;
		{
			let mut block = hglobal.GlobalLock()?;
			block.as_mut_slice().copy_from_slice(data); // copy the contents into HGLOBAL
		}
		ptr_to_sysresult(unsafe {
			ffi::SetClipboardData(format.raw() as _, hglobal.leak().ptr() as _)
		})?;
		Ok(())
	}
}
