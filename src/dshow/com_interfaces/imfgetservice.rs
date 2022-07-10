#![allow(non_camel_case_types, non_snake_case)]

use crate::co;
use crate::ffi_types::{HRES, PCVOID};
use crate::ole::decl::{ComPtr, HrResult};
use crate::ole::privs::ok_to_hrresult;
use crate::prelude::{ComInterface, ole_IUnknown};
use crate::vt::IUnknownVT;

/// [`IMFGetService`](crate::IMFGetService) virtual table.
#[cfg_attr(docsrs, doc(cfg(feature = "dshow")))]
#[repr(C)]
pub struct IMFGetServiceVT {
	pub IUnknownVT: IUnknownVT,
	pub GetService: fn(ComPtr, PCVOID, PCVOID, *mut ComPtr) -> HRES,
}

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
/// # use winsafe::{co::CLSID, co::CLSCTX, CoCreateInstance};
/// # let vmr = CoCreateInstance::<IBaseFilter>(&CLSID::new("00000000-0000-0000-0000-000000000000"), None, CLSCTX::INPROC_SERVER)?;
///
/// let get_svc = vmr.QueryInterface::<IMFGetService>()?;
/// # Ok::<_, winsafe::co::HRESULT>(())
/// ```
#[cfg_attr(docsrs, doc(cfg(feature = "dshow")))]
pub struct IMFGetService(ComPtr);

impl_iunknown!(IMFGetService, "fa993888-4383-415a-a930-dd472a8cf6f7");
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
	/// # use winsafe::{co::CLSID, co::CLSCTX, CoCreateInstance};
	/// # let get_svc = CoCreateInstance::<IMFGetService>(&CLSID::new("00000000-0000-0000-0000-000000000000"), None, CLSCTX::INPROC_SERVER)?;
	///
	/// let controller_evr = get_svc
	///     .GetService::<IMFVideoDisplayControl>(
	///         &co::DSHOW_SERVICE::MR_VIDEO_RENDER_SERVICE,
	///     )?;
	/// # Ok::<_, co::HRESULT>(())
	/// ```
	#[must_use]
	fn GetService<T>(&self, service_id: &co::DSHOW_SERVICE) -> HrResult<T>
		where T: ComInterface,
	{
		let mut ppv_queried = ComPtr::null();
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IMFGetServiceVT);
			ok_to_hrresult(
				(vt.GetService)(
					self.ptr(),
					service_id as *const _ as _,
					&T::IID as *const _ as _,
					&mut ppv_queried,
				),
			)
		}.map(|_| T::from(ppv_queried))
	}
}