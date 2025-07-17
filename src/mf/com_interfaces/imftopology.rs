#![allow(non_camel_case_types, non_snake_case)]

use crate::decl::*;
use crate::mf::vts::*;
use crate::ole::privs::*;
use crate::prelude::*;

com_interface! { IMFTopology: "83cf873a-f6da-4bc8-823f-bacfd55dc433";
	/// [`IMFTopology`](https://learn.microsoft.com/en-us/windows/win32/api/mfidl/nn-mfidl-imftopology)
	/// COM interface.
	///
	/// Automatically calls
	/// [`Release`](https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
	///
	/// Usually created with [`MFCreateTopology`](crate::MFCreateTopology)
	/// function.
	///
	/// # Examples
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*};
	///
	/// let topology = w::MFCreateTopology()?;
	/// # w::HrResult::Ok(())
	/// ```
}

impl mf_IMFAttributes for IMFTopology {}
impl mf_IMFTopology for IMFTopology {}

/// This trait is enabled with the `mf` feature, and provides methods for
/// [`IMFTopology`](crate::IMFTopology).
///
/// Prefer importing this trait through the prelude:
///
/// ```no_run
/// use winsafe::prelude::*;
/// ```
pub trait mf_IMFTopology: mf_IMFAttributes {
	/// [`IMFTopology::AddNode`](https://learn.microsoft.com/en-us/windows/win32/api/mfidl/nf-mfidl-imftopology-addnode)
	/// method.
	fn AddNode(&self, node: &impl mf_IMFTopologyNode) -> HrResult<()> {
		HrRet(unsafe { (vt::<IMFTopologyVT>(self).AddNode)(self.ptr(), node.ptr()) }).to_hrresult()
	}

	fn_com_noparm! { Clear: IMFTopologyVT;
		/// [`IMFTopology::Clear`](https://learn.microsoft.com/en-us/windows/win32/api/mfidl/nf-mfidl-imftopology-clear)
		/// method.
	}

	/// [`IMFTopology::CloneFrom`](https://learn.microsoft.com/en-us/windows/win32/api/mfidl/nf-mfidl-imftopology-clonefrom)
	/// method.
	fn CloneFrom(&self, topology: &impl mf_IMFTopology) -> HrResult<()> {
		HrRet(unsafe { (vt::<IMFTopologyVT>(self).CloneFrom)(self.ptr(), topology.ptr()) })
			.to_hrresult()
	}

	/// [`IMFTopology::GetNode`](https://learn.microsoft.com/en-us/windows/win32/api/mfidl/nf-mfidl-imftopology-getnode)
	/// method.
	#[must_use]
	fn GetNode(&self, index: u16) -> HrResult<IMFTopologyNode> {
		let mut queried = unsafe { IMFTopologyNode::null() };
		HrRet(unsafe { (vt::<IMFTopologyVT>(self).GetNode)(self.ptr(), index, queried.as_mut()) })
			.to_hrresult()
			.map(|_| queried)
	}

	/// [`IMFTopology::GetNodeByID`](https://learn.microsoft.com/en-us/windows/win32/api/mfidl/nf-mfidl-imftopology-getnodebyid)
	/// method.
	#[must_use]
	fn GetNodeByID(&self, topo_node_id: u64) -> HrResult<IMFTopologyNode> {
		let mut queried = unsafe { IMFTopologyNode::null() };
		HrRet(unsafe {
			(vt::<IMFTopologyVT>(self).GetNodeByID)(self.ptr(), topo_node_id, queried.as_mut())
		})
		.to_hrresult()
		.map(|_| queried)
	}

	/// [`IMFTopology::GetNodeCount`](https://learn.microsoft.com/en-us/windows/win32/api/mfidl/nf-mfidl-imftopology-getnodecount)
	/// method.
	#[must_use]
	fn GetNodeCount(&self) -> HrResult<u16> {
		let mut nodes = 0u16;
		HrRet(unsafe { (vt::<IMFTopologyVT>(self).GetNodeCount)(self.ptr(), &mut nodes) })
			.to_hrresult()
			.map(|_| nodes)
	}

	fn_com_interface_get! { GetOutputNodeCollection: IMFTopologyVT => IMFCollection;
		/// [`IMFTopology::GetOutputNodeCollection`](https://learn.microsoft.com/en-us/windows/win32/api/mfidl/nf-mfidl-imftopology-getoutputnodecollection)
		/// method.
	}

	fn_com_interface_get! { GetSourceNodeCollection: IMFTopologyVT => IMFCollection;
		/// [`IMFTopology::GetSourceNodeCollection`](https://learn.microsoft.com/en-us/windows/win32/api/mfidl/nf-mfidl-imftopology-getsourcenodecollection)
		/// method.
	}

	/// [`IMFTopology::GetTopologyID`](https://learn.microsoft.com/en-us/windows/win32/api/mfidl/nf-mfidl-imftopology-gettopologyid)
	/// method.
	#[must_use]
	fn GetTopologyID(&self) -> HrResult<u64> {
		let mut id = 0u64;
		HrRet(unsafe { (vt::<IMFTopologyVT>(self).GetTopologyID)(self.ptr(), &mut id) })
			.to_hrresult()
			.map(|_| id)
	}

	/// [`IMFTopology::RemoveNode`](https://learn.microsoft.com/en-us/windows/win32/api/mfidl/nf-mfidl-imftopology-removenode)
	/// method.
	fn RemoveNode(&self, node: &impl mf_IMFTopologyNode) -> HrResult<()> {
		HrRet(unsafe { (vt::<IMFTopologyVT>(self).RemoveNode)(self.ptr(), node.ptr()) })
			.to_hrresult()
	}
}
