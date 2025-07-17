#![allow(non_camel_case_types, non_snake_case)]

use crate::decl::*;
use crate::dxgi::vts::*;
use crate::kernel::privs::*;
use crate::ole::privs::*;
use crate::prelude::*;

com_interface! { IDXGIDeviceSubObject: "3d3e0379-f9de-4d58-bb6c-18d62992f1a6";
	/// [`IDXGIDeviceSubObject`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi/nn-dxgi-idxgidevicesubobject)
	/// COM interface.
	///
	/// Automatically calls
	/// [`Release`](https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
}

impl dxgi_IDXGIObject for IDXGIDeviceSubObject {}
impl dxgi_IDXGIDeviceSubObject for IDXGIDeviceSubObject {}

/// This trait is enabled with the `dxgi` feature, and provides methods for
/// [`IDXGIDeviceSubObject`](crate::IDXGIDeviceSubObject).
///
/// Prefer importing this trait through the prelude:
///
/// ```no_run
/// use winsafe::prelude::*;
/// ```
pub trait dxgi_IDXGIDeviceSubObject: dxgi_IDXGIObject {
	/// [`IDXGIDeviceSubObject::GetDevice`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi/nf-dxgi-idxgidevicesubobject-getdevice)
	/// method.
	#[must_use]
	fn GetDevice<T>(&self) -> HrResult<T>
	where
		T: ole_IUnknown,
	{
		let mut queried = unsafe { T::null() };
		HrRet(unsafe {
			(vt::<IDXGIDeviceSubObjectVT>(self).GetDevice)(
				self.ptr(),
				pcvoid(&T::IID),
				queried.as_mut(),
			)
		}).to_hrresult()
		.map(|_| queried)
	}
}
