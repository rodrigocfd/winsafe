#![allow(non_camel_case_types, non_snake_case)]

use crate::decl::*;
use crate::ole::privs::*;
use crate::oleaut::vts::*;
use crate::prelude::*;

com_interface! { ITypeInfo: "00020401-0000-0000-c000-000000000046";
	/// [`ITypeInfo`](https://learn.microsoft.com/en-us/windows/win32/api/oaidl/nn-oaidl-itypeinfo)
	/// COM interface.
	///
	/// Automatically calls
	/// [`Release`](https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
}

impl oleaut_ITypeInfo for ITypeInfo {}

/// This trait is enabled with the `oleaut` feature, and provides methods for
/// [`ITypeInfo`](crate::ITypeInfo).
///
/// Prefer importing this trait through the prelude:
///
/// ```no_run
/// use winsafe::prelude::*;
/// ```
pub trait oleaut_ITypeInfo: ole_IUnknown {
	/// [`ITypeInfo::CreateInstance`](https://learn.microsoft.com/en-us/windows/win32/api/oaidl/nf-oaidl-itypeinfo-createinstance)
	/// method.
	#[must_use]
	fn CreateInstance<T>(&self, iunk_outer: Option<&mut IUnknown>) -> HrResult<T>
		where T: ole_IUnknown,
	{
		let (mut queried, mut queried_outer) = unsafe {(
			T::null(),
			IUnknown::null(),
		)};

		ok_to_hrresult(
			unsafe {
				(vt::<ITypeInfoVT>(self).CreateInstance)(
					self.ptr(),
					iunk_outer.as_ref()
						.map_or(std::ptr::null_mut(), |_| queried_outer.as_mut()),
					&T::IID as *const _ as _,
					queried.as_mut(),
				)
			},
		).map(|_| queried)
	}
}
