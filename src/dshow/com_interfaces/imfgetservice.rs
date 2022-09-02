#![allow(non_camel_case_types, non_snake_case)]

use crate::co;
use crate::kernel::ffi_types::{HRES, PCVOID};
use crate::ole::decl::{ComPtr, HrResult};
use crate::ole::privs::ok_to_hrresult;
use crate::prelude::ole_IUnknown;
use crate::vt::IUnknownVT;

/// [`IMFGetService`](crate::IMFGetService) virtual table.
#[cfg_attr(docsrs, doc(cfg(feature = "dshow")))]
#[repr(C)]
pub struct IMFGetServiceVT {
	pub IUnknownVT: IUnknownVT,
	pub GetService: fn(ComPtr, PCVOID, PCVOID, *mut ComPtr) -> HRES,
}

com_interface! { IMFGetService: "dshow";
	"fa993888-4383-415a-a930-dd472a8cf6f7";
	/// [`IMFGetService`](https://docs.microsoft.com/en-us/windows/win32/api/mfidl/nn-mfidl-imfgetservice)
	/// COM interface over [`IMFGetServiceVT`](crate::vt::IMFGetServiceVT).
	///
	/// Automatically calls
	/// [`IUnknown::Release`](https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
	///
	/// # Examples
	///
	/// ```rust,no_run
	/// use winsafe::prelude::*;
	/// use winsafe::{IBaseFilter, IMFGetService};
	///
	/// let vmr: IBaseFilter; // initialized somewhere
	/// # let vmr = IBaseFilter::from(unsafe { winsafe::ComPtr::null() });
	///
	/// let get_svc = vmr.QueryInterface::<IMFGetService>()?;
	/// # Ok::<_, winsafe::co::HRESULT>(())
	/// ```
}

impl dshow_IMFGetService for IMFGetService {}

/// This trait is enabled with the `dshow` feature, and provides methods for
/// [`IMFGetService`](crate::IMFGetService).
///
/// Prefer importing this trait through the prelude:
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// ```
#[cfg_attr(docsrs, doc(cfg(feature = "dshow")))]
pub trait dshow_IMFGetService: ole_IUnknown {
	/// [`IMFGetService::GetService`](https://docs.microsoft.com/en-us/windows/win32/api/mfidl/nf-mfidl-imfgetservice-getservice)
	/// method.
	///
	/// # Examples
	///
	/// ```rust,no_run
	/// use winsafe::prelude::*;
	/// use winsafe::{co, IMFGetService, IMFVideoDisplayControl};
	///
	/// let get_svc: IMFGetService; // initialized somewhere
	/// # let get_svc = IMFGetService::from(unsafe { winsafe::ComPtr::null() });
	///
	/// let controller_evr = get_svc
	///     .GetService::<IMFVideoDisplayControl>(
	///         &co::DSHOW_SERVICE::MR_VIDEO_RENDER_SERVICE,
	///     )?;
	/// # Ok::<_, co::HRESULT>(())
	/// ```
	#[must_use]
	fn GetService<T>(&self, service_id: &co::DSHOW_SERVICE) -> HrResult<T>
		where T: ole_IUnknown,
	{
		unsafe {
			let mut ppv_queried = ComPtr::null();
			let vt = &**(self.ptr().0 as *mut *mut IMFGetServiceVT);
			ok_to_hrresult(
				(vt.GetService)(
					self.ptr(),
					service_id as *const _ as _,
					&T::IID as *const _ as _,
					&mut ppv_queried,
				),
			).map(|_| T::from(ppv_queried))
		}
	}
}
