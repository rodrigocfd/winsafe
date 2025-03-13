#![allow(non_camel_case_types, non_snake_case)]

use crate::co;
use crate::decl::*;
use crate::kernel::privs::*;
use crate::ole::privs::*;
use crate::oleaut::vts::*;
use crate::prelude::*;

com_interface! { IDispatch: "00020400-0000-0000-c000-000000000046";
	/// [`IDispatch`](https://learn.microsoft.com/en-us/windows/win32/api/oaidl/nn-oaidl-idispatch)
	/// COM interface.
	///
	/// Automatically calls
	/// [`Release`](https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
}

impl oleaut_IDispatch for IDispatch {}

/// This trait is enabled with the `oleaut` feature, and provides methods for
/// [`IDispatch`](crate::IDispatch).
///
/// Prefer importing this trait through the prelude:
///
/// ```no_run
/// use winsafe::prelude::*;
/// ```
pub trait oleaut_IDispatch: ole_IUnknown {
	/// [`IDispatch::GetIDsOfNames`](https://learn.microsoft.com/en-us/windows/win32/api/oaidl/nf-oaidl-idispatch-getidsofnames)
	/// method.
	#[must_use]
	fn GetIDsOfNames(&self, names: &[impl AsRef<str>], lcid: LCID) -> HrResult<Vec<i32>> {
		let (_wstrs, pwstrs) = create_wstr_ptr_vecs(Some(names));
		let mut ids = vec![i32::default(); names.len()];

		ok_to_hrresult(unsafe {
			(vt::<IDispatchVT>(self).GetIDsOfNames)(
				self.ptr(),
				&co::IID::default() as *const _ as _,
				vec_ptr(&pwstrs),
				names.len() as _,
				lcid.into(),
				ids.as_mut_ptr() as _,
			)
		})
		.map(|_| ids)
	}

	/// [`IDispatch::GetTypeInfoCount`](https://learn.microsoft.com/en-us/windows/win32/api/oaidl/nf-oaidl-idispatch-gettypeinfocount)
	/// method.
	#[must_use]
	fn GetTypeInfoCount(&self) -> HrResult<u32> {
		let mut count = u32::default();
		ok_to_hrresult(unsafe {
			(vt::<IDispatchVT>(self).GetTypeInfoCount)(self.ptr(), &mut count)
		})
		.map(|_| count)
	}

	/// [`IDispatch::GetTypeInfo`](https://learn.microsoft.com/en-us/windows/win32/api/oaidl/nf-oaidl-idispatch-gettypeinfo)
	/// method.
	#[must_use]
	fn GetTypeInfo(&self, info_type: u32, lcid: LCID) -> HrResult<ITypeInfo> {
		let mut queried = unsafe { ITypeInfo::null() };
		ok_to_hrresult(unsafe {
			(vt::<IDispatchVT>(self).GetTypeInfo)(
				self.ptr(),
				info_type,
				lcid.into(),
				queried.as_mut(),
			)
		})
		.map(|_| queried)
	}
}
