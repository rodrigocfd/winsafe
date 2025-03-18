#![allow(non_camel_case_types, non_snake_case)]

use crate::decl::*;
use crate::guard::*;
use crate::kernel::privs::*;
use crate::prelude::*;
use crate::user::ffi;

handle! { HACCEL;
	/// Handle to an
	/// [accelerator table](https://learn.microsoft.com/en-us/windows/win32/winprog/windows-data-types#haccel).
}

impl user_Haccel for HACCEL {}

/// This trait is enabled with the `user` feature, and provides methods for
/// [`HACCEL`](crate::HACCEL).
///
/// Prefer importing this trait through the prelude:
///
/// ```no_run
/// use winsafe::prelude::*;
/// ```
pub trait user_Haccel: Handle {
	/// [`CreateAcceleratorTable`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createacceleratortablew)
	/// function.
	#[must_use]
	fn CreateAcceleratorTable(accel: &[ACCEL]) -> SysResult<DestroyAcceleratorTableGuard> {
		// For some reason, debug builds were randomly crashing with error 998:
		// Invalid access to memory location.
		// So, allocate an HGLOBAL buffer and copy the ACCEL array onto it.
		let hg_buf = HGLOBAL::GlobalAlloc(
			crate::co::GMEM::ZEROINIT,
			std::mem::size_of::<ACCEL>() * accel.len(),
		)?;
		{
			let sli =
				unsafe { std::slice::from_raw_parts_mut(hg_buf.ptr() as *mut ACCEL, accel.len()) };
			sli.iter_mut()
				.zip(accel.iter())
				.for_each(|(buf2, src)| *buf2 = *src);
		}

		unsafe {
			ptr_to_sysresult_handle(ffi::CreateAcceleratorTableW(hg_buf.ptr(), accel.len() as _))
				.map(|h| DestroyAcceleratorTableGuard::new(h))
		}
	}
}
