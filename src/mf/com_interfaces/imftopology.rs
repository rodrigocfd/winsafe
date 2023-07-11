#![allow(non_camel_case_types, non_snake_case)]

use crate::kernel::ffi_types::{COMPTR, HRES};
use crate::ole::decl::HrResult;
use crate::ole::privs::{ok_to_hrresult, vt};
use crate::prelude::mf_IMFAttributes;
use crate::vt::IMFAttributesVT;

/// [`IMFTopology`](crate::IMFTopology) virtual table.
#[repr(C)]
pub struct IMFTopologyVT {
	pub IMFAttributesVT: IMFAttributesVT,
	pub GetTopologyID: fn(COMPTR, *mut u64) -> HRES,
	pub AddNode: fn(COMPTR, COMPTR) -> HRES,
	pub RemoveNode: fn(COMPTR, COMPTR) -> HRES,
	pub GetNodeCount: fn(COMPTR, *mut u16) -> HRES,
	pub GetNode: fn(COMPTR, u16, *mut COMPTR) -> HRES,
	pub Clear: fn(COMPTR) -> HRES,
	pub CloneFrom: fn(COMPTR, COMPTR) -> HRES,
	pub GetNodeByID: fn(COMPTR, u64, *mut COMPTR) -> HRES,
	pub GetSourceNodeCollection: fn(COMPTR, *mut COMPTR) -> HRES,
	pub GetOutputNodeCollection: fn(COMPTR, *mut COMPTR) -> HRES,
}

com_interface! { IMFTopology: "83cf873a-f6da-4bc8-823f-bacfd55dc433";
	/// [`IMFTopology`](https://learn.microsoft.com/en-us/windows/win32/api/mfidl/nn-mfidl-imftopology)
	/// COM interface over [`IMFTopologyVT`](crate::vt::IMFTopologyVT).
	///
	/// Automatically calls
	/// [`Release`](https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
}

impl mf_IMFAttributes for IMFTopology {}
impl mf_IMFTopology for IMFTopology {}

/// This trait is enabled with the `mf` feature, and provides methods for
/// [`IMFTopology`](crate::IMFTopology).
///
/// Prefer importing this trait through the prelude:
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// ```
pub trait mf_IMFTopology: mf_IMFAttributes {
	fn_com_noparm! { Clear: IMFTopologyVT;
		/// [`IMFTopology::Clear`](https://learn.microsoft.com/en-us/windows/win32/api/mfidl/nf-mfidl-imftopology-clear)
		/// method.
	}

	/// [`IMFTopology::CloneFrom`](https://learn.microsoft.com/en-us/windows/win32/api/mfidl/nf-mfidl-imftopology-clonefrom)
	/// method.
	fn CloneFrom(&self, topology: &impl mf_IMFTopology) -> HrResult<()> {
		ok_to_hrresult(
			unsafe {
				(vt::<IMFTopologyVT>(self).CloneFrom)(self.ptr(), topology.ptr())
			},
		)
	}

	/// [`IMFTopology::GetNodeCount`](https://learn.microsoft.com/en-us/windows/win32/api/mfidl/nf-mfidl-imftopology-getnodecount)
	/// method.
	#[must_use]
	fn GetNodeCount(&self) -> HrResult<u16> {
		let mut nodes = u16::default();
		ok_to_hrresult(
			unsafe {
				(vt::<IMFTopologyVT>(self).GetNodeCount)(self.ptr(), &mut nodes)
			},
		).map(|_| nodes)
	}
}
