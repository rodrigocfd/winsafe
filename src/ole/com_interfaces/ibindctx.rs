#![allow(non_camel_case_types, non_snake_case)]

use crate::decl::*;
use crate::kernel::privs::*;
use crate::ole::{privs::*, vts::*};
use crate::prelude::*;

com_interface! { IBindCtx: "0000000e-0000-0000-c000-000000000046";
	/// [`IBindCtx`](https://learn.microsoft.com/en-us/windows/win32/api/objidl/nn-objidl-ibindctx)
	/// COM interface.
	///
	/// Automatically calls
	/// [`Release`](https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
	///
	/// Can be created with [`CreateBindCtx`](crate::CreateBindCtx).
}

impl ole_IBindCtx for IBindCtx {}

/// This trait is enabled with the `ole` feature, and provides methods for
/// [`IBindCtx`](crate::IBindCtx).
///
/// Prefer importing this trait through the prelude:
///
/// ```no_run
/// use winsafe::prelude::*;
/// ```
pub trait ole_IBindCtx: ole_IUnknown {
	/// [`IBindCtx::GetBindOptions`](https://learn.microsoft.com/en-us/windows/win32/api/objidl/nf-objidl-ibindctx-getbindoptions)
	/// method.
	#[must_use]
	fn GetBindOptions(&self) -> HrResult<BIND_OPTS3> {
		let mut bo = BIND_OPTS3::default();
		ok_to_hrresult(unsafe {
			(vt::<IBindCtxVT>(self).GetBindOptions)(self.ptr(), pvoid(&mut bo))
		})
		.map(|_| bo)
	}

	/// [`IBindCtx::GetObjectParam`](https://learn.microsoft.com/en-us/windows/win32/api/objidl/nf-objidl-ibindctx-getobjectparam)
	/// method.
	#[must_use]
	fn GetObjectParam<T>(&self, key: &str) -> HrResult<T>
	where
		T: ole_IUnknown,
	{
		let mut queried = unsafe { T::null() };
		ok_to_hrresult(unsafe {
			(vt::<IBindCtxVT>(self).GetObjectParam)(
				self.ptr(),
				WString::from_str(key).as_ptr(),
				queried.as_mut(),
			)
		})
		.map(|_| queried)
	}

	fn_com_noparm! { ReleaseBoundObjects: IBindCtxVT;
		/// [`IBindCtx::ReleaseBoundObjects`](https://learn.microsoft.com/en-us/windows/win32/api/objidl/nf-objidl-ibindctx-releaseboundobjects)
		/// method.
	}

	/// [`IBindCtx::RevokeObjectParam`](https://learn.microsoft.com/en-us/windows/win32/api/objidl/nf-objidl-ibindctx-revokeobjectparam)
	/// method.
	fn RevokeObjectParam(&self, key: &str) -> HrResult<()> {
		ok_to_hrresult(unsafe {
			(vt::<IBindCtxVT>(self).RevokeObjectParam)(self.ptr(), WString::from_str(key).as_ptr())
		})
	}
}
