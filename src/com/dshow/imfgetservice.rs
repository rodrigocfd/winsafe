#![allow(non_snake_case)]

use crate::aliases::WinResult;
use crate::com::iunknown::{ComInterface, ComPtr, IUnknownT, IUnknownVT};
use crate::ffi::{HRESULT, PCVOID};
use crate::privs::hr_to_winresult;
use crate::structs::GUID;

/// [`IMFGetService`](crate::dshow::IMFGetService) virtual table.
#[repr(C)]
pub struct IMFGetServiceVT {
	pub IUnknownVT: IUnknownVT,
	pub GetService: fn(ComPtr, PCVOID, PCVOID, *mut ComPtr) -> HRESULT,
}

/// [`IMFGetService`](https://docs.microsoft.com/en-us/windows/win32/api/mfidl/nn-mfidl-imfgetservice)
/// COM interface over [`IMFGetServiceVT`](crate::dshow::vt::IMFGetServiceVT).
///
/// Automatically calls
/// [`IUnknown::Release`](https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
/// when the object goes out of scope.
///
/// # Examples
///
/// ```rust,ignore
/// use winsafe::prelude::*;
/// use winsafe::dshow;
///
/// let vmr: dshow::IBaseFilter; // initialized somewhere
///
/// let get_svc = vmr.QueryInterface::<dshow::IMFGetService>()?;
/// ```
pub struct IMFGetService(ComPtr);

impl_iunknown!(IMFGetService, 0xfa993888, 0x4383, 0x415a, 0xa930, 0xdd472a8cf6f7);
impl IMFGetServiceT for IMFGetService {}

/// Exposes the [`IMFGetService`](crate::dshow::IMFGetService) methods.
pub trait IMFGetServiceT: IUnknownT {
	/// [`IMFGetService::GetService`](https://docs.microsoft.com/en-us/windows/win32/api/mfidl/nf-mfidl-imfgetservice-getservice)
	/// method.
	///
	/// # Examples
	///
	/// ```rust,ignore
	/// use winsafe::prelude::*;
	/// use winsafe::dshow;
	///
	/// let get_svc: dshow::IMFGetService; // initialized somewhere
	///
	/// let controller_evr = get_svc
	///     .GetService::<dshow::IMFVideoDisplayControl>(
	///         &dshow::guid::MR_VIDEO_RENDER_SERVICE,
	///     )?;
	/// ```
	fn GetService<T: ComInterface>(&self, service_guid: &GUID) -> WinResult<T> {
		let mut ppv_queried = ComPtr::null();
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IMFGetServiceVT);
			hr_to_winresult(
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
