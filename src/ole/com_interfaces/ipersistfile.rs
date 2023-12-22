#![allow(non_camel_case_types, non_snake_case)]

use crate::co;
use crate::decl::*;
use crate::kernel::ffi_types::*;
use crate::ole::privs::*;
use crate::prelude::*;
use crate::vt::*;

/// [`IPersistFile`](crate::IPersistFile) virtual table.
#[repr(C)]
pub struct IPersistFileVT {
	pub IPersistVT: IPersistVT,
	pub IsDirty: fn(COMPTR) -> HRES,
	pub Load: fn(COMPTR, PCSTR, u32) -> HRES,
	pub Save: fn(COMPTR, PCSTR, i32) -> HRES,
	pub SaveCompleted: fn(COMPTR, PCSTR) -> HRES,
	pub GetCurFile: fn(COMPTR, PVOID) -> HRES,
}

com_interface! { IPersistFile: "0000010b-0000-0000-c000-000000000046";
	/// [`IPersistFile`](https://learn.microsoft.com/en-us/windows/win32/api/objidl/nn-objidl-ipersistfile)
	/// COM interface over [`IPersistFileVT`](crate::vt::IPersistFileVT).
	///
	/// Automatically calls
	/// [`Release`](https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
}

impl ole_IPersist for IPersistFile {}
impl ole_IPersistFile for IPersistFile {}

/// This trait is enabled with the `ole` feature, and provides methods for
/// [`IPersistFile`](crate::IPersistFile).
///
/// Prefer importing this trait through the prelude:
///
/// ```no_run
/// use winsafe::prelude::*;
/// ```
pub trait ole_IPersistFile: ole_IUnknown {
	/// [`IPersistFile::IsDirty`](https://learn.microsoft.com/en-us/windows/win32/api/objidl/nf-objidl-ipersistfile-isdirty)
	/// method.
	#[must_use]
	fn IsDirty(&self) -> HrResult<bool> {
		okfalse_to_hrresult(
			unsafe { (vt::<IPersistFileVT>(self).IsDirty)(self.ptr()) },
		)
	}

	/// [`IPersistFile::Load`](https://learn.microsoft.com/en-us/windows/win32/api/objidl/nf-objidl-ipersistfile-load)
	/// method.
	fn Load(&self, file_name: &str, dw_mode: co::STGM) -> HrResult<()> {
		ok_to_hrresult(
			unsafe {
				(vt::<IPersistFileVT>(self).Load)(
					self.ptr(),
					WString::from_str(file_name).as_ptr(),
					dw_mode.raw(),
				)
			},
		)
	}

	/// [`IPersistFile::Save`](https://learn.microsoft.com/en-us/windows/win32/api/objidl/nf-objidl-ipersistfile-save)
	/// method.
	fn Save(&self, file_name: Option<&str>, remember: bool) -> HrResult<()> {
		ok_to_hrresult(
			unsafe {
				(vt::<IPersistFileVT>(self).Save)(
					self.ptr(),
					file_name.map_or(std::ptr::null(), |f| WString::from_str(f).as_ptr()),
					remember as _,
				)
			},
		)
	}
}