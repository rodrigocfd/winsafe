#![allow(non_camel_case_types, non_snake_case)]

use crate::co;
use crate::dxgi::decl::{DXGI_ADAPTER_DESC, IDXGIOutput};
use crate::kernel::decl::GUID;
use crate::kernel::ffi_types::{COMPTR, HRES, PCVOID, PVOID};
use crate::ole::decl::HrResult;
use crate::ole::privs::{ok_to_hrresult, vt};
use crate::prelude::{dxgi_IDXGIObject, ole_IUnknown};
use crate::vt::IDXGIObjectVT;

/// [`IDXGIAdapter`](crate::IDXGIAdapter) virtual table.
#[repr(C)]
pub struct IDXGIAdapterVT {
	pub IDXGIObjectVT: IDXGIObjectVT,
	pub EnumOutputs: fn(COMPTR, u32, *mut COMPTR) -> HRES,
	pub GetDesc: fn(COMPTR, PVOID) -> HRES,
	pub CheckInterfaceSupport: fn(COMPTR, PCVOID, *mut i64) -> HRES,
}

com_interface! { IDXGIAdapter: "2411e7e1-12ac-4ccf-bd14-9798e8534dc0";
	/// [`IDXGIAdapter`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi/nn-dxgi-idxgiadapter)
	/// COM interface over [`IDXGIAdapterVT`](crate::vt::IDXGIAdapterVT).
	///
	/// Automatically calls
	/// [`Release`](https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
}

impl dxgi_IDXGIObject for IDXGIAdapter {}
impl dxgi_IDXGIAdapter for IDXGIAdapter {}

/// This trait is enabled with the `dxgi` feature, and provides methods for
/// [`IDXGIAdapter`](crate::IDXGIAdapter).
///
/// Prefer importing this trait through the prelude:
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// ```
pub trait dxgi_IDXGIAdapter: dxgi_IDXGIObject {
	/// Returns an iterator over the [`IDXGIOutput`](crate::IDXGIOutput)
	/// elements which calls
	/// [`IDXGIAdapter::EnumOutputs`](crate::prelude::dxgi_IDXGIAdapter::EnumOutputs)
	/// internally.
	///
	/// # Examples
	///
	/// ```rust,no_run
	/// use winsafe::prelude::*;
	/// use winsafe::IDXGIAdapter;
	///
	/// let adapter: IDXGIAdapter; // initialized somewhere
	/// # let adapter = unsafe { IDXGIAdapter::null() };
	///
	/// for output in adapter.iter_outputs() {
	///     let output = output?;
	///     // ...
	/// }
	/// # Ok::<_, winsafe::co::HRESULT>(())
	/// ```
	#[must_use]
	fn iter_outputs(&self,
	) -> Box<dyn Iterator<Item = HrResult<IDXGIOutput>> + '_>
	{
		Box::new(EnumOutputsIter::new(self))
	}

	/// [`IDXGIAdapter::CheckInterfaceSupport`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi/nf-dxgi-idxgiadapter-checkinterfacesupport)
	/// method.
	#[must_use]
	fn CheckInterfaceSupport(&self, interface_name: &GUID) -> HrResult<i64> {
		let mut umd_ver = i64::default();
		ok_to_hrresult(
			unsafe {
				(vt::<IDXGIAdapterVT>(self).CheckInterfaceSupport)(
					self.ptr(),
					interface_name as *const _ as _,
					&mut umd_ver,
				)
			},
		).map(|_| umd_ver)
	}

	/// [`IDXGIAdapter::EnumOutputs`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi/nf-dxgi-idxgiadapter-enumoutputs)
	/// method.
	///
	/// Prefer using
	/// [`IDXGIAdapter::iter_outputs`](crate::prelude::dxgi_IDXGIAdapter::iter_outputs),
	/// which is simpler.
	#[must_use]
	fn EnumOutputs(&self, output: u32) -> HrResult<IDXGIOutput> {
		let mut queried = unsafe { IDXGIOutput::null() };
		ok_to_hrresult(
			unsafe {
				(vt::<IDXGIAdapterVT>(self).EnumOutputs)(
					self.ptr(),
					output,
					queried.as_mut(),
				)
			},
		).map(|_| queried)
	}

	/// [`IDXGIAdapter::GetDesc`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi/nf-dxgi-idxgiadapter-getdesc)
	/// method.
	///
	/// # Examples
	///
	/// ```rust,no_run
	/// use winsafe::prelude::*;
	/// use winsafe::{IDXGIAdapter, DXGI_ADAPTER_DESC};
	///
	/// let adapter: IDXGIAdapter; // initialized somewhere
	/// # let adapter = unsafe { IDXGIAdapter::null() };
	/// let mut desc = DXGI_ADAPTER_DESC::default();
	///
	/// adapter.GetDesc(&mut desc)?;
	/// # Ok::<_, winsafe::co::HRESULT>(())
	/// ```
	fn GetDesc(&self, desc: &mut DXGI_ADAPTER_DESC) -> HrResult<()> {
		ok_to_hrresult(
			unsafe {
				(vt::<IDXGIAdapterVT>(self).GetDesc)(
					self.ptr(),
					desc as *mut _ as _,
				)
			},
		)
	}
}

//------------------------------------------------------------------------------

struct EnumOutputsIter<'a, I>
	where I: dxgi_IDXGIAdapter,
{
	adapter: &'a I,
	cur_index: u32,
}

impl<'a, I> Iterator for EnumOutputsIter<'a, I>
	where I: dxgi_IDXGIAdapter,
{
	type Item = HrResult<IDXGIOutput>;

	fn next(&mut self) -> Option<Self::Item> {
		if self.cur_index == 0xffff_ffff {
			None
		} else {
			match self.adapter.EnumOutputs(self.cur_index) {
				Err(err) => {
					self.cur_index = 0xffff_ffff; // no further iterations will be made
					match err {
						co::HRESULT::DXGI_ERROR_NOT_FOUND => None, // no more entries
						_ => Some(Err(err)), // actual error
					}
				},
				Ok(output) => {
					self.cur_index += 1;
					Some(Ok(output))
				},
			}
		}
	}
}

impl<'a, I> EnumOutputsIter<'a, I>
	where I: dxgi_IDXGIAdapter,
{
	fn new(adapter: &'a I) -> Self {
		Self { adapter, cur_index: 0 }
	}
}
