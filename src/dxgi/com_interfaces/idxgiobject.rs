#![allow(non_camel_case_types, non_snake_case)]

use crate::decl::*;
use crate::dxgi::vts::*;
use crate::kernel::privs::*;
use crate::ole::privs::*;
use crate::prelude::*;

com_interface! { IDXGIObject: "aec22fb8-76f3-4639-9be0-28eb43a67a2e";
	/// [`IDXGIObject`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi/nn-dxgi-idxgiobject)
	/// COM interface.
	///
	/// Automatically calls
	/// [`Release`](https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
}

impl dxgi_IDXGIObject for IDXGIObject {}

/// This trait is enabled with the `dxgi` feature, and provides methods for
/// [`IDXGIObject`](crate::IDXGIObject).
///
/// Prefer importing this trait through the prelude:
///
/// ```no_run
/// use winsafe::prelude::*;
/// ```
pub trait dxgi_IDXGIObject: ole_IUnknown {
	/// [`IDXGIObject::GetParent`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi/nf-dxgi-idxgiobject-getparent)
	/// method.
	#[must_use]
	fn GetParent<T>(&self) -> HrResult<T>
	where
		T: ole_IUnknown,
	{
		let mut queried = unsafe { T::null() };
		ok_to_hrresult(unsafe {
			(vt::<IDXGIObjectVT>(self).GetParent)(self.ptr(), pcvoid(&T::IID), queried.as_mut())
		})
		.map(|_| queried)
	}

	/// [`IDXGIObject::SetPrivateData`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi/nf-dxgi-idxgiobject-setprivatedata)
	/// method.
	///
	/// Note: a copy of the data is made.
	fn SetPrivateData<T>(&self, name: &GUID, data: &T) -> HrResult<()>
	where
		T: Sized,
	{
		ok_to_hrresult(unsafe {
			(vt::<IDXGIObjectVT>(self).SetPrivateData)(
				self.ptr(),
				pcvoid(name),
				std::mem::size_of::<T>() as _,
				pcvoid(data),
			)
		})
	}

	/// [`IDXGIObject::SetPrivateDataInterface`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi/nf-dxgi-idxgiobject-setprivatedatainterface)
	/// method.
	fn SetPrivateDataInterface<T>(&self, obj: &T) -> HrResult<()>
	where
		T: ole_IUnknown,
	{
		ok_to_hrresult(unsafe {
			(vt::<IDXGIObjectVT>(self).SetPrivateDataInterface)(
				self.ptr(),
				pcvoid(&T::IID),
				obj.ptr(),
			)
		})
	}
}
