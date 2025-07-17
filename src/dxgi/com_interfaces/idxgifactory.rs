#![allow(non_camel_case_types, non_snake_case)]

use crate::co;
use crate::decl::*;
use crate::dxgi::{iterators::*, vts::*};
use crate::kernel::privs::*;
use crate::ole::privs::*;
use crate::prelude::*;

com_interface! { IDXGIFactory: "7b7166ec-21c7-44ae-b21a-c9ae321ae369";
	/// [`IDXGIFactory`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi/nn-dxgi-idxgifactory)
	/// COM interface.
	///
	/// Automatically calls
	/// [`Release`](https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
	///
	/// Usually created with [`CreateDXGIFactory`](crate::CreateDXGIFactory)
	/// function.
	///
	/// # Examples
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*};
	///
	/// let factory = w::CreateDXGIFactory()?;
	/// # w::HrResult::Ok(())
	/// ```
}

impl dxgi_IDXGIObject for IDXGIFactory {}
impl dxgi_IDXGIFactory for IDXGIFactory {}

/// This trait is enabled with the `dxgi` feature, and provides methods for
/// [`IDXGIFactory`](crate::IDXGIFactory).
///
/// Prefer importing this trait through the prelude:
///
/// ```no_run
/// use winsafe::prelude::*;
/// ```
pub trait dxgi_IDXGIFactory: dxgi_IDXGIObject {
	/// [`IDXGIFactory::CreateSoftwareAdapter`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi/nf-dxgi-idxgifactory-createsoftwareadapter)
	/// method.
	#[must_use]
	fn CreateSoftwareAdapter(&self, hmodule: &HINSTANCE) -> HrResult<IDXGIAdapter> {
		let mut queried = unsafe { IDXGIAdapter::null() };
		HrRet(unsafe {
			(vt::<IDXGIFactoryVT>(self).CreateSoftwareAdapter)(
				self.ptr(),
				hmodule.ptr(),
				queried.as_mut(),
			)
		}).to_hrresult()
		.map(|_| queried)
	}

	/// [`IDXGIFactory::CreateSwapChain`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi/nf-dxgi-idxgifactory-createswapchain)
	/// method.
	#[must_use]
	fn CreateSwapChain(
		&self,
		device: &impl ole_IUnknown,
		desc: &DXGI_SWAP_CHAIN_DESC,
	) -> HrResult<IDXGISwapChain> {
		let mut queried = unsafe { IDXGISwapChain::null() };
		HrRet(unsafe {
			(vt::<IDXGIFactoryVT>(self).CreateSwapChain)(
				self.ptr(),
				device.ptr(),
				pcvoid(desc),
				queried.as_mut(),
			)
		}).to_hrresult()
		.map(|_| queried)
	}

	/// [`IDXGIFactory::EnumAdapters`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi/nf-dxgi-idxgifactory-enumadapters)
	/// method.
	///
	/// Returns an iterator over [`IDXGIAdapter`](crate::IDXGIAdapter) elements.
	///
	/// # Examples
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*};
	///
	/// let factory: w::IDXGIFactory; // initialized somewhere
	/// # let factory = unsafe { w::IDXGIFactory::null() };
	///
	/// for adapter in factory.EnumAdapters() {
	///     let adapter = adapter?;
	///     // ...
	/// }
	///
	/// // Collecting into a Vec
	/// let adapters: Vec<w::IDXGIAdapter> =
	///     factory.EnumAdapters()
	///         .collect::<w::HrResult<_>>()?;
	/// # w::HrResult::Ok(())
	/// ```
	#[must_use]
	fn EnumAdapters(&self) -> impl Iterator<Item = HrResult<IDXGIAdapter>> + '_ {
		IdxgifactoryEnumadaptersIter::new(self)
	}

	/// [`IDXGIFactory::GetWindowAssociation`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi/nf-dxgi-idxgifactory-getwindowassociation)
	/// method.
	#[must_use]
	fn GetWindowAssociation(&self) -> HrResult<HWND> {
		let mut hwnd = HWND::NULL;
		HrRet(unsafe {
			(vt::<IDXGIFactoryVT>(self).GetWindowAssociation)(self.ptr(), hwnd.as_mut())
		}).to_hrresult()
		.map(|_| hwnd)
	}

	/// [`IDXGIFactory::MakeWindowAssociation`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi/nf-dxgi-idxgifactory-makewindowassociation)
	/// method.
	fn MakeWindowAssociation(&self, hwnd: &HWND, flags: co::DXGI_MWA) -> HrResult<()> {
		HrRet(unsafe {
			(vt::<IDXGIFactoryVT>(self).MakeWindowAssociation)(self.ptr(), hwnd.ptr(), flags.raw())
		}).to_hrresult()
	}
}
