#![allow(non_camel_case_types, non_snake_case)]

use crate::co;
use crate::decl::*;
use crate::kernel::privs::*;
use crate::ole::{privs::*, vts::*};
use crate::prelude::*;

com_interface! { IPersist: "0000010c-0000-0000-c000-000000000046";
	/// [`IPersist`](https://learn.microsoft.com/en-us/windows/win32/api/objidl/nn-objidl-ipersist)
	/// COM interface.
	///
	/// Automatically calls
	/// [`Release`](https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
}

impl ole_IPersist for IPersist {}

/// This trait is enabled with the `ole` feature, and provides methods for
/// [`IPersist`](crate::IPersist).
///
/// Prefer importing this trait through the prelude:
///
/// ```no_run
/// use winsafe::prelude::*;
/// ```
pub trait ole_IPersist: ole_IUnknown {
	/// [`IPersist::GetClassID`](https://learn.microsoft.com/en-us/windows/win32/api/objidl/nf-objidl-ipersist-getclassid)
	/// method.
	#[must_use]
	fn GetClassID(&self) -> HrResult<co::CLSID> {
		let mut clsid = co::CLSID::default();
		HrRet(unsafe { (vt::<IPersistVT>(self).GetClassID)(self.ptr(), pvoid(&mut clsid)) })
			.to_hrresult()
			.map(|_| clsid)
	}
}
