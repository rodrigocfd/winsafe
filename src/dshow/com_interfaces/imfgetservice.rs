#![allow(non_snake_case)]

use crate::ffi_types::{HRES, PCVOID};
use crate::ole::decl::{ComPtr, GUID, HrResult};
use crate::ole::privs::ok_to_hrresult;
use crate::prelude::{ComInterface, OleIUnknown};
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
/// # use winsafe::{CLSID, co::CLSCTX, CoCreateInstance};
/// # let vmr = CoCreateInstance::<IBaseFilter>(&CLSID::new(0,0,0,0,0), None, CLSCTX::INPROC_SERVER)?;
///
/// let get_svc = vmr.QueryInterface::<IMFGetService>()?;
/// # Ok::<_, winsafe::co::HRESULT>(())
/// ```
#[cfg_attr(docsrs, doc(cfg(feature = "dshow")))]
pub struct IMFGetService(ComPtr);

impl_iunknown!(IMFGetService, 0xfa993888, 0x4383, 0x415a, 0xa930, 0xdd472a8cf6f7);
impl DshowIMFGetService for IMFGetService {}

/// [`IMFGetService`](crate::IMFGetService) methods methods from `dshow`
/// feature.
#[cfg_attr(docsrs, doc(cfg(feature = "dshow")))]
pub trait DshowIMFGetService: OleIUnknown {
	/// [`IMFGetService::GetService`](https://docs.microsoft.com/en-us/windows/win32/api/mfidl/nf-mfidl-imfgetservice-getservice)
	/// method.
	///
	/// # Examples
	///
	/// ```rust,no_run
	/// use winsafe::prelude::*;
	/// use winsafe::{GUID, IMFGetService, IMFVideoDisplayControl};
	///
	/// let get_svc: IMFGetService; // initialized somewhere
	/// # use winsafe::{CLSID, co::CLSCTX, CoCreateInstance};
	/// # let get_svc = CoCreateInstance::<IMFGetService>(&CLSID::new(0,0,0,0,0), None, CLSCTX::INPROC_SERVER)?;
	///
	/// let controller_evr = get_svc
	///     .GetService::<IMFVideoDisplayControl>(
	///         &GUID::MR_VIDEO_RENDER_SERVICE,
	///     )?;
	/// # Ok::<_, winsafe::co::HRESULT>(())
	/// ```
	fn GetService<T: ComInterface>(&self, service_guid: &GUID) -> HrResult<T> {
		let mut ppv_queried = ComPtr::null();
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IMFGetServiceVT);
			ok_to_hrresult(
				(vt.GetService)(
					self.ptr(),
					service_guid as *const _ as _,
					&T::IID as *const _ as _,
					&mut ppv_queried,
				),
			)
		}.map(|_| T::from(ppv_queried))
	}
}
