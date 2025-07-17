#![allow(non_camel_case_types, non_snake_case)]

use crate::decl::*;
use crate::ole::{privs::*, vts::*};
use crate::prelude::*;

com_interface! { IPersistStream: "00000109-0000-0000-c000-000000000046";
	/// [`IPersistStream`](https://learn.microsoft.com/en-us/windows/win32/api/objidl/nn-objidl-ipersiststream)
	/// COM interface.
	///
	/// Automatically calls
	/// [`Release`](https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
}

impl ole_IPersist for IPersistStream {}
impl ole_IPersistStream for IPersistStream {}

/// This trait is enabled with the `ole` feature, and provides methods for
/// [`IPersistStream`](crate::IPersistStream).
///
/// Prefer importing this trait through the prelude:
///
/// ```no_run
/// use winsafe::prelude::*;
/// ```
pub trait ole_IPersistStream: ole_IPersist {
	/// [`IPersistStream::GetSizeMax`](https://learn.microsoft.com/en-us/windows/win32/api/objidl/nf-objidl-ipersiststream-getsizemax)
	/// method.
	#[must_use]
	fn GetSizeMax(&self) -> HrResult<u64> {
		let mut max = 0u64;
		HrRet(unsafe { (vt::<IPersistStreamVT>(self).GetSizeMax)(self.ptr(), &mut max) })
			.to_hrresult()
			.map(|_| max)
	}

	/// [`IPersistStream::IsDirty`](https://learn.microsoft.com/en-us/windows/win32/api/objidl/nf-objidl-ipersiststream-isdirty)
	/// method.
	#[must_use]
	fn IsDirty(&self) -> HrResult<bool> {
		HrRet(unsafe { (vt::<IPersistStreamVT>(self).IsDirty)(self.ptr()) }).to_bool_hrresult()
	}

	/// [`IPersistStream::Load`](https://learn.microsoft.com/en-us/windows/win32/api/objidl/nf-objidl-ipersiststream-load)
	/// method.
	fn Load(&self, stream: &impl ole_IStream) -> HrResult<()> {
		HrRet(unsafe { (vt::<IPersistStreamVT>(self).Load)(self.ptr(), stream.ptr()) })
			.to_hrresult()
	}

	/// [`IPersistStream::Save`](https://learn.microsoft.com/en-us/windows/win32/api/objidl/nf-objidl-ipersiststream-save)
	/// method.
	fn Save(&self, stream: &impl ole_IStream, clear_dirty: bool) -> HrResult<()> {
		HrRet(unsafe {
			(vt::<IPersistStreamVT>(self).Save)(self.ptr(), stream.ptr(), clear_dirty as _)
		})
		.to_hrresult()
	}
}
