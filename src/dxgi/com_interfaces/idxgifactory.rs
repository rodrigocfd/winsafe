#![allow(non_camel_case_types, non_snake_case)]

use crate::co;
use crate::dxgi::decl::IDXGIAdapter;
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
	/// Returns an iterator over the [`IDXGIAdapter`](crate::IDXGIAdapter)
	/// elements which calls
	/// [`IDXGIFactory::EnumAdapters`](crate::prelude::dxgi_IDXGIFactory::EnumAdapters)
	/// internally.
	///
	/// # Examples
	///
	/// ```rust,no_run
	/// use winsafe::prelude::*;
	/// use winsafe::IDXGIFactory;
	///
	/// let factory: IDXGIFactory; // initialized somewhere
	/// # let factory = unsafe { IDXGIFactory::null() };
	///
	/// for adapter in factory.iter_adapters() {
	///     let adapter = adapter?;
	///     // ...
	/// }
	/// # Ok::<_, winsafe::co::HRESULT>(())
	/// ```
	#[must_use]
	fn iter_adapters(&self,
	) -> Box<dyn Iterator<Item = HrResult<IDXGIAdapter>> + '_>
	{
		Box::new(EnumAdaptersIter::new(self))
	}

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
	/// Prefer using
	/// [`IDXGIFactory::iter_adapters`](crate::prelude::dxgi_IDXGIFactory::iter_adapters),
	/// which is simpler.
	#[must_use]
	fn EnumAdapters(&self, adapter: u32) -> HrResult<IDXGIAdapter> {
		let mut queried = unsafe { IDXGIAdapter::null() };
		ok_to_hrresult(
			unsafe {
				(vt::<IDXGIFactoryVT>(self).EnumAdapters)(
					self.ptr(),
					adapter,
					queried.as_mut(),
				)
			},
		).map(|_| queried)
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

//------------------------------------------------------------------------------

struct EnumAdaptersIter<'a, I>
	where I: dxgi_IDXGIFactory,
{
	fact: &'a I,
	cur_index: u32,
}

impl<'a, I> Iterator for EnumAdaptersIter<'a, I>
	where I: dxgi_IDXGIFactory,
{
	type Item = HrResult<IDXGIAdapter>;

	fn next(&mut self) -> Option<Self::Item> {
		if self.cur_index == 0xffff_ffff {
			None
		} else {
			match self.fact.EnumAdapters(self.cur_index) {
				Err(err) => {
					self.cur_index = 0xffff_ffff; // no further iterations will be made
					match err {
						co::HRESULT::DXGI_ERROR_NOT_FOUND => None, // no more entries
						_ => Some(Err(err)), // actual error
					}
				},
				Ok(adapter) => {
					self.cur_index += 1;
					Some(Ok(adapter))
				},
			}
		}
	}
}

impl<'a, I> EnumAdaptersIter<'a, I>
	where I: dxgi_IDXGIFactory,
{
	fn new(fact: &'a I) -> Self {
		Self { fact, cur_index: 0 }
	}
}
