#![allow(non_camel_case_types, non_snake_case)]

use crate::decl::*;
use crate::dxgi::{iterators::*, vts::*};
use crate::kernel::privs::*;
use crate::ole::privs::*;
use crate::prelude::*;

com_interface! { IDXGIAdapter: "2411e7e1-12ac-4ccf-bd14-9798e8534dc0";
	/// [`IDXGIAdapter`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi/nn-dxgi-idxgiadapter)
	/// COM interface.
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
/// ```no_run
/// use winsafe::prelude::*;
/// ```
pub trait dxgi_IDXGIAdapter: dxgi_IDXGIObject {
	/// [`IDXGIAdapter::CheckInterfaceSupport`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi/nf-dxgi-idxgiadapter-checkinterfacesupport)
	/// method.
	#[must_use]
	fn CheckInterfaceSupport(&self, interface_name: &GUID) -> HrResult<i64> {
		let mut umd_ver = i64::default();
		ok_to_hrresult(unsafe {
			(vt::<IDXGIAdapterVT>(self).CheckInterfaceSupport)(
				self.ptr(),
				pcvoid(interface_name),
				&mut umd_ver,
			)
		})
		.map(|_| umd_ver)
	}

	/// [`IDXGIAdapter::EnumOutputs`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi/nf-dxgi-idxgiadapter-enumoutputs)
	/// method.
	///
	/// Returns an iterator over [`IDXGIOutput`](crate::IDXGIOutput) elements.
	///
	/// # Examples
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*};
	///
	/// let adapter: w::IDXGIAdapter; // initialized somewhere
	/// # let adapter = unsafe { w::IDXGIAdapter::null() };
	///
	/// for output in adapter.EnumOutputs() {
	///     let output = output?;
	///     // ...
	/// }
	///
	/// // Collecting into a Vec
	/// let outputs: Vec<w::IDXGIOutput> =
	///     adapter.EnumOutputs()
	///         .collect::<w::HrResult<Vec<_>>>()?;
	/// # w::HrResult::Ok(())
	/// ```
	#[must_use]
	fn EnumOutputs(&self) -> impl Iterator<Item = HrResult<IDXGIOutput>> + '_ {
		IdxgiadapterEnumoutputsIter::new(self)
	}

	/// [`IDXGIAdapter::GetDesc`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi/nf-dxgi-idxgiadapter-getdesc)
	/// method.
	#[must_use]
	fn GetDesc(&self) -> HrResult<DXGI_ADAPTER_DESC> {
		let mut desc = DXGI_ADAPTER_DESC::default();
		ok_to_hrresult(unsafe {
			(vt::<IDXGIAdapterVT>(self).GetDesc)(self.ptr(), pvoid(&mut desc))
		})
		.map(|_| desc)
	}
}
