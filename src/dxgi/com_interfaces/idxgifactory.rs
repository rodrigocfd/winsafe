#![allow(non_camel_case_types, non_snake_case)]

use crate::co;
use crate::dxgi::decl::IDXGIAdapter;
use crate::kernel::ffi_types::{HANDLE, HRES, PCVOID};
use crate::ole::decl::{ComPtr, HrResult};
use crate::ole::privs::ok_to_hrresult;
use crate::prelude::{dxgi_IDXGIObject, Handle};
use crate::user::decl::HWND;
use crate::vt::IDXGIObjectVT;

/// [`IDXGIFactory`](crate::IDXGIFactory) virtual table.
#[repr(C)]
pub struct IDXGIFactoryVT {
	pub IUnknownVT: IDXGIObjectVT,
	pub EnumAdapters: fn(ComPtr, u32, *const ComPtr) -> HRES,
	pub MakeWindowAssociation: fn(ComPtr, HANDLE, u32) -> HRES,
	pub GetWindowAssociation: fn(ComPtr, *mut HANDLE) -> HRES,
	pub CreateSwapChain: fn(ComPtr, *const ComPtr, PCVOID, *mut ComPtr) -> HRES,
	pub CreateSoftwareAdapter: fn(ComPtr, HANDLE, *mut ComPtr) -> HRES,
}

com_interface! { IDXGIFactory: "7b7166ec-21c7-44ae-b21a-c9ae321ae369";
	/// [`IDXGIFactory`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi/nn-dxgi-idxgifactory)
	/// COM interface over [`IDXGIFactoryVT`](crate::vt::IDXGIFactoryVT).
	///
	/// Automatically calls
	/// [`Release`](https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
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
	/// # let factory = IDXGIFactory::from(unsafe { winsafe::ComPtr::null() });
	///
	/// for adapter in factory.iter_adapters() {
	///     let adapter = adapter?;
	///     // ...
	/// }
	/// # Ok::<_, winsafe::co::HRESULT>(())
	/// ```
	#[must_use]
	fn iter_adapters(&self) -> Box<dyn Iterator<Item = HrResult<IDXGIAdapter>> + '_> {
		Box::new(EnumAdaptersIter::new(self))
	}

	/// [`IDXGIFactory::EnumAdapters`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi/nf-dxgi-idxgifactory-enumadapters)
	/// method.
	///
	/// Prefer using
	/// [`IDXGIFactory::iter_adapters`](crate::prelude::dxgi_IDXGIFactory::iter_adapters),
	/// which is simpler.
	#[must_use]
	fn EnumAdapters(&self, adapter: u32) -> HrResult<IDXGIAdapter> {
		unsafe {
			let mut ppv_queried = ComPtr::null();
			let vt = self.vt_ref::<IDXGIFactoryVT>();
			ok_to_hrresult(
				(vt.EnumAdapters)(self.ptr(), adapter, &mut ppv_queried),
			).map(|_| IDXGIAdapter::from(ppv_queried))
		}
	}

	/// [`IDXGIFactory::GetWindowAssociation`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi/nf-dxgi-idxgifactory-getwindowassociation)
	/// method.
	#[must_use]
	fn GetWindowAssociation(&self) -> HrResult<HWND> {
		let mut hwnd = HWND::NULL;
		unsafe {
			let vt = self.vt_ref::<IDXGIFactoryVT>();
			ok_to_hrresult((vt.GetWindowAssociation)(self.ptr(), &mut hwnd.0))
		}.map(|_| hwnd)
	}

	/// [`IDXGIFactory::MakeWindowAssociation`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi/nf-dxgi-idxgifactory-makewindowassociation)
	/// method.
	fn MakeWindowAssociation(&self,
		hwnd: &HWND, flags: co::DXGI_MWA) -> HrResult<()>
	{
		unsafe {
			let vt = self.vt_ref::<IDXGIFactoryVT>();
			ok_to_hrresult((vt.MakeWindowAssociation)(self.ptr(), hwnd.0, flags.0))
		}
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
