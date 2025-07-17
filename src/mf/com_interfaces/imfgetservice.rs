#![allow(non_camel_case_types, non_snake_case)]

use crate::co;
use crate::decl::*;
use crate::kernel::privs::*;
use crate::mf::vts::*;
use crate::ole::privs::*;
use crate::prelude::*;

com_interface! { IMFGetService: "fa993888-4383-415a-a930-dd472a8cf6f7";
	/// [`IMFGetService`](https://learn.microsoft.com/en-us/windows/win32/api/mfidl/nn-mfidl-imfgetservice)
	/// COM interface.
	///
	/// Automatically calls
	/// [`IUnknown::Release`](https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
	///
	/// # Examples
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*};
	///
	/// let vmr: w::IBaseFilter; // initialized somewhere
	/// # let vmr = unsafe { w::IBaseFilter::null() };
	///
	/// let get_svc = vmr.QueryInterface::<w::IMFGetService>()?;
	/// # w::HrResult::Ok(())
	/// ```
}

impl mf_IMFGetService for IMFGetService {}

/// This trait is enabled with the `mf` feature, and provides methods for
/// [`IMFGetService`](crate::IMFGetService).
///
/// Prefer importing this trait through the prelude:
///
/// ```no_run
/// use winsafe::prelude::*;
/// ```
pub trait mf_IMFGetService: ole_IUnknown {
	/// [`IMFGetService::GetService`](https://learn.microsoft.com/en-us/windows/win32/api/mfidl/nf-mfidl-imfgetservice-getservice)
	/// method.
	///
	/// # Examples
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*, co};
	///
	/// let get_svc: w::IMFGetService; // initialized somewhere
	/// # let get_svc = unsafe { w::IMFGetService::null() };
	///
	/// let controller_evr = get_svc
	///     .GetService::<w::IMFVideoDisplayControl>(
	///         &co::MF_SERVICE::MR_VIDEO_RENDER_SERVICE,
	///     )?;
	/// # w::HrResult::Ok(())
	/// ```
	#[must_use]
	fn GetService<T: ole_IUnknown>(&self, service_id: &co::MF_SERVICE) -> HrResult<T>
	{
		let mut queried = unsafe { T::null() };
		HrRet(unsafe {
			(vt::<IMFGetServiceVT>(self).GetService)(
				self.ptr(),
				pcvoid(service_id),
				pcvoid(&T::IID),
				queried.as_mut(),
			)
		}).to_hrresult()
		.map(|_| queried)
	}
}
