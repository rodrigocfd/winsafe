#![allow(non_camel_case_types, non_snake_case)]

use crate::co;
use crate::decl::*;
use crate::ole::{privs::*, vts::*};
use crate::prelude::*;

com_interface! { IPersistFile: "0000010b-0000-0000-c000-000000000046";
	/// [`IPersistFile`](https://learn.microsoft.com/en-us/windows/win32/api/objidl/nn-objidl-ipersistfile)
	/// COM interface.
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
	/// [`IPersistFile::GetCurFile`](https://learn.microsoft.com/en-us/windows/win32/api/objidl/nf-objidl-ipersistfile-getcurfile)
	/// method.
	#[must_use]
	fn GetCurFile(&self) -> HrResult<String> {
		let mut pstr = std::ptr::null_mut::<u16>();
		HrRet(unsafe { (vt::<IPersistFileVT>(self).GetCurFile)(self.ptr(), &mut pstr) })
			.to_hrresult()
			.map(|_| htaskmem_ptr_to_str(pstr))
	}

	/// [`IPersistFile::IsDirty`](https://learn.microsoft.com/en-us/windows/win32/api/objidl/nf-objidl-ipersistfile-isdirty)
	/// method.
	#[must_use]
	fn IsDirty(&self) -> HrResult<bool> {
		HrRet(unsafe { (vt::<IPersistFileVT>(self).IsDirty)(self.ptr()) }).to_bool_hrresult()
	}

	/// [`IPersistFile::Load`](https://learn.microsoft.com/en-us/windows/win32/api/objidl/nf-objidl-ipersistfile-load)
	/// method.
	fn Load(&self, file_name: &str, dw_mode: co::STGM) -> HrResult<()> {
		HrRet(unsafe {
			(vt::<IPersistFileVT>(self).Load)(
				self.ptr(),
				WString::from_str(file_name).as_ptr(),
				dw_mode.raw(),
			)
		})
		.to_hrresult()
	}

	/// [`IPersistFile::Save`](https://learn.microsoft.com/en-us/windows/win32/api/objidl/nf-objidl-ipersistfile-save)
	/// method.
	fn Save(&self, file_name: Option<&str>, remember: bool) -> HrResult<()> {
		HrRet(unsafe {
			(vt::<IPersistFileVT>(self).Save)(
				self.ptr(),
				WString::from_opt_str(file_name).as_ptr(),
				remember as _,
			)
		})
		.to_hrresult()
	}

	/// [`IPersistFile::SaveCompleted`](https://learn.microsoft.com/en-us/windows/win32/api/objidl/nf-objidl-ipersistfile-savecompleted)
	/// method.
	fn SaveCompleted(&self, file_name: &str) -> HrResult<()> {
		HrRet(unsafe {
			(vt::<IPersistFileVT>(self).SaveCompleted)(
				self.ptr(),
				WString::from_str(file_name).as_ptr(),
			)
		})
		.to_hrresult()
	}
}
