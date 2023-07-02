#![allow(non_camel_case_types, non_snake_case)]

use crate::co;
use crate::dxgi::decl::IDXGIAdapter;
use crate::dxgi::iterators::IdxgifactoryEnumadaptersIter;
use crate::kernel::decl::HINSTANCE;
use crate::kernel::ffi_types::{COMPTR, HANDLE, HRES, PCVOID};
use crate::ole::decl::HrResult;
use crate::ole::privs::{ok_to_hrresult, vt};
use crate::prelude::{dxgi_IDXGIObject, Handle, ole_IUnknown};
use crate::user::decl::HWND;
use crate::vt::IDXGIObjectVT;

/// [`IDXGIFactory`](crate::IDXGIFactory) virtual table.
#[repr(C)]
pub struct IDXGIFactoryVT {
	pub IDXGIObjectVT: IDXGIObjectVT,
	pub EnumAdapters: fn(COMPTR, u32, *const COMPTR) -> HRES,
	pub MakeWindowAssociation: fn(COMPTR, HANDLE, u32) -> HRES,
	pub GetWindowAssociation: fn(COMPTR, *mut HANDLE) -> HRES,
	pub CreateSwapChain: fn(COMPTR, *const COMPTR, PCVOID, *mut COMPTR) -> HRES,
	pub CreateSoftwareAdapter: fn(COMPTR, HANDLE, *mut COMPTR) -> HRES,
}

com_interface! { IDXGIFactory: "7b7166ec-21c7-44ae-b21a-c9ae321ae369";
	/// [`IDXGIFactory`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi/nn-dxgi-idxgifactory)
	/// COM interface over [`IDXGIFactoryVT`](crate::vt::IDXGIFactoryVT).
	///
	/// Automatically calls
	/// [`Release`](https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
	///
	/// Created with [`CreateDXGIFactory`](crate::CreateDXGIFactory) function.
	///
	/// # Examples
	///
	/// ```rust,no_run
	/// use winsafe::prelude::*;
	/// use winsafe::CreateDXGIFactory;
	///
	/// let factory = CreateDXGIFactory()?;
	/// # Ok::<_, winsafe::co::HRESULT>(())
	/// ```
}

impl dxgi_IDXGIObject for IDXGIFactory {}
impl dxgi_IDXGIFactory for IDXGIFactory {}

/// This trait is enabled with the `dxgi` feature, and provides methods for
/// [`IDXGIFactory`](crate::IDXGIFactory).
///
/// Prefer importing this trait through the prelude:
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// ```
pub trait dxgi_IDXGIFactory: dxgi_IDXGIObject {
	/// [`IDXGIFactory::CreateSoftwareAdapter`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi/nf-dxgi-idxgifactory-createsoftwareadapter)
	/// method.
	#[must_use]
	fn CreateSoftwareAdapter(&self,
		hmodule: &HINSTANCE) -> HrResult<IDXGIAdapter>
	{
		let mut queried = unsafe { IDXGIAdapter::null() };
		ok_to_hrresult(
			unsafe {
				(vt::<IDXGIFactoryVT>(self).CreateSoftwareAdapter)(
					self.ptr(),
					hmodule.ptr(),
					queried.as_mut(),
				)
			},
		).map(|_| queried)
	}

	/// [`IDXGIFactory::EnumAdapters`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi/nf-dxgi-idxgifactory-enumadapters)
	/// method.
	///
	/// Returns an iterator over [`IDXGIAdapter`](crate::IDXGIAdapter) elements.
	///
	/// # Examples
	///
	/// ```rust,no_run
	/// use winsafe::prelude::*;
	/// use winsafe::{HrResult, IDXGIAdapter, IDXGIFactory};
	///
	/// let factory: IDXGIFactory; // initialized somewhere
	/// # let factory = unsafe { IDXGIFactory::null() };
	///
	/// for adapter in factory.EnumAdapters() {
	///     let adapter = adapter?;
	///     // ...
	/// }
	///
	/// // Collecting into a Vec
	/// let adapters: Vec<IDXGIAdapter> =
	///     factory.EnumAdapters()
	///         .collect::<HrResult<Vec<_>>>()?;
	/// # Ok::<_, winsafe::co::HRESULT>(())
	/// ```
	#[must_use]
	fn EnumAdapters(&self,
	) -> Box<dyn Iterator<Item = HrResult<IDXGIAdapter>> + '_>
	{
		Box::new(IdxgifactoryEnumadaptersIter::new(self))
	}

	/// [`IDXGIFactory::GetWindowAssociation`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi/nf-dxgi-idxgifactory-getwindowassociation)
	/// method.
	#[must_use]
	fn GetWindowAssociation(&self) -> HrResult<HWND> {
		let mut hwnd = HWND::NULL;
		ok_to_hrresult(
			unsafe {
				(vt::<IDXGIFactoryVT>(self).GetWindowAssociation)(
					self.ptr(),
					hwnd.as_mut(),
				)
			},
		).map(|_| hwnd)
	}

	/// [`IDXGIFactory::MakeWindowAssociation`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi/nf-dxgi-idxgifactory-makewindowassociation)
	/// method.
	fn MakeWindowAssociation(&self,
		hwnd: &HWND, flags: co::DXGI_MWA) -> HrResult<()>
	{
		ok_to_hrresult(
			unsafe {
				(vt::<IDXGIFactoryVT>(self).MakeWindowAssociation)(
					self.ptr(),
					hwnd.ptr(),
					flags.raw(),
				)
			},
		)
	}
}
