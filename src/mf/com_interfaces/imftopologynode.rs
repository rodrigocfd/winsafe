#![allow(non_camel_case_types, non_snake_case)]

use crate::co;
use crate::decl::*;
use crate::mf::vts::*;
use crate::ole::privs::*;
use crate::prelude::*;

com_interface! { IMFTopologyNode: "83cf873a-f6da-4bc8-823f-bacfd55dc430";
	/// [`IMFTopologyNode`](https://learn.microsoft.com/en-us/windows/win32/api/mfidl/nn-mfidl-imftopologynode)
	/// COM interface.
	///
	/// Automatically calls
	/// [`Release`](https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
	///
	/// Usually created with
	/// [`MFCreateTopologyNode`](crate::MFCreateTopologyNode) function.
	///
	/// # Examples
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*, co};
	///
	/// let topology_node = w::MFCreateTopologyNode(co::MF_TOPOLOGY::OUTPUT_NODE)?;
	/// # w::HrResult::Ok(())
	/// ```
}

impl mf_IMFAttributes for IMFTopologyNode {}
impl mf_IMFTopologyNode for IMFTopologyNode {}

/// This trait is enabled with the `mf` feature, and provides methods for
/// [`IMFTopologyNode`](crate::IMFTopologyNode).
///
/// Prefer importing this trait through the prelude:
///
/// ```no_run
/// use winsafe::prelude::*;
/// ```
pub trait mf_IMFTopologyNode: mf_IMFAttributes {
	/// [`IMFTopologyNode::CloneFrom`](https://learn.microsoft.com/en-us/windows/win32/api/mfidl/nf-mfidl-imftopologynode-clonefrom)
	/// method.
	fn CloneFrom(&self, node: &impl mf_IMFTopologyNode) -> HrResult<()> {
		HrRet(unsafe { (vt::<IMFTopologyNodeVT>(self).CloneFrom)(self.ptr(), node.ptr()) })
			.to_hrresult()
	}

	/// [`IMFTopologyNode::ConnectOutput`](https://learn.microsoft.com/en-us/windows/win32/api/mfidl/nf-mfidl-imftopologynode-connectoutput)
	/// method.
	fn ConnectOutput(
		&self,
		output_index: u32,
		downstream_node: &impl mf_IMFTopologyNode,
		input_index_on_downstream_node: u32,
	) -> HrResult<()> {
		HrRet(unsafe {
			(vt::<IMFTopologyNodeVT>(self).ConnectOutput)(
				self.ptr(),
				output_index,
				downstream_node.ptr(),
				input_index_on_downstream_node,
			)
		})
		.to_hrresult()
	}

	/// [`IMFTopologyNode::DisconnectOutput`](https://learn.microsoft.com/en-us/windows/win32/api/mfidl/nf-mfidl-imftopologynode-disconnectoutput)
	/// method.
	#[must_use]
	fn DisconnectOutput(&self, output_index: u32) -> HrResult<()> {
		HrRet(unsafe { (vt::<IMFTopologyNodeVT>(self).DisconnectOutput)(self.ptr(), output_index) })
			.to_hrresult()
	}

	/// [`IMFTopologyNode::GetInput`](https://learn.microsoft.com/en-us/windows/win32/api/mfidl/nf-mfidl-imftopologynode-getinput)
	/// method.
	///
	/// Returns the node and the index of the output stream that is connected to
	/// this node's input stream.
	#[must_use]
	fn GetInput(&self, input_index: u32) -> HrResult<(IMFTopologyNode, u32)> {
		let mut queried = unsafe { IMFTopologyNode::null() };
		let mut output_index_downstream_node = 0u32;
		HrRet(unsafe {
			(vt::<IMFTopologyNodeVT>(self).GetInput)(
				self.ptr(),
				input_index,
				queried.as_mut(),
				&mut output_index_downstream_node,
			)
		})
		.to_hrresult()
		.map(|_| (queried, output_index_downstream_node))
	}

	/// [`IMFTopologyNode::GetInputCount`](https://learn.microsoft.com/en-us/windows/win32/api/mfidl/nf-mfidl-imftopologynode-getinputcount)
	/// method.
	#[must_use]
	fn GetInputCount(&self) -> HrResult<u32> {
		let mut c = 0u32;
		HrRet(unsafe { (vt::<IMFTopologyNodeVT>(self).GetInputCount)(self.ptr(), &mut c) })
			.to_hrresult()
			.map(|_| c)
	}

	/// [`IMFTopologyNode::GetNodeType`](https://learn.microsoft.com/en-us/windows/win32/api/mfidl/nf-mfidl-imftopologynode-getnodetype)
	/// method.
	#[must_use]
	fn GetNodeType(&self) -> HrResult<co::MF_TOPOLOGY> {
		let mut ty = co::MF_TOPOLOGY::default();
		HrRet(unsafe { (vt::<IMFTopologyNodeVT>(self).GetNodeType)(self.ptr(), ty.as_mut()) })
			.to_hrresult()
			.map(|_| ty)
	}

	/// [`IMFTopologyNode::GetObject`](https://learn.microsoft.com/en-us/windows/win32/api/mfidl/nf-mfidl-imftopologynode-getobject)
	/// method.
	#[must_use]
	fn GetObject<T>(&self) -> HrResult<T>
	where
		T: ole_IUnknown,
	{
		let mut queried = unsafe { T::null() };
		HrRet(unsafe { (vt::<IMFTopologyNodeVT>(self).GetObject)(self.ptr(), queried.as_mut()) })
			.to_hrresult()
			.map(|_| queried)
	}

	/// [`IMFTopologyNode::GetOutput`](https://learn.microsoft.com/en-us/windows/win32/api/mfidl/nf-mfidl-imftopologynode-getoutput)
	/// method.
	///
	/// Returns the node and the index of the input stream that is connected to
	/// this node's output stream.
	#[must_use]
	fn GetOutput(&self, output_index: u32) -> HrResult<(IMFTopologyNode, u32)> {
		let mut queried = unsafe { IMFTopologyNode::null() };
		let mut input_index_downstream_node = 0u32;
		HrRet(unsafe {
			(vt::<IMFTopologyNodeVT>(self).GetOutput)(
				self.ptr(),
				output_index,
				queried.as_mut(),
				&mut input_index_downstream_node,
			)
		})
		.to_hrresult()
		.map(|_| (queried, input_index_downstream_node))
	}

	/// [`IMFTopologyNode::GetOutputCount`](https://learn.microsoft.com/en-us/windows/win32/api/mfidl/nf-mfidl-imftopologynode-getoutputcount)
	/// method.
	#[must_use]
	fn GetOutputCount(&self) -> HrResult<u32> {
		let mut c = 0u32;
		HrRet(unsafe { (vt::<IMFTopologyNodeVT>(self).GetOutputCount)(self.ptr(), &mut c) })
			.to_hrresult()
			.map(|_| c)
	}

	/// [`IMFTopologyNode::GetTopoNodeID`](https://learn.microsoft.com/en-us/windows/win32/api/mfidl/nf-mfidl-imftopologynode-gettoponodeid)
	/// method.
	#[must_use]
	fn GetTopoNodeID(&self) -> HrResult<u64> {
		let mut id = 0u64;
		HrRet(unsafe { (vt::<IMFTopologyNodeVT>(self).GetTopoNodeID)(self.ptr(), &mut id) })
			.to_hrresult()
			.map(|_| id)
	}

	/// [`IMFTopologyNode::SetObject`](https://learn.microsoft.com/en-us/windows/win32/api/mfidl/nf-mfidl-imftopologynode-setobject)
	/// method
	fn SetObject(&self, object: &impl ole_IUnknown) -> HrResult<()> {
		HrRet(unsafe { (vt::<IMFTopologyNodeVT>(self).SetObject)(self.ptr(), object.ptr()) })
			.to_hrresult()
	}

	/// [`IMFTopologyNode::SetTopoNodeID`](https://learn.microsoft.com/en-us/windows/win32/api/mfidl/nf-mfidl-imftopologynode-settoponodeid)
	/// method.
	fn SetTopoNodeID(&self, topo_id: u64) -> HrResult<()> {
		HrRet(unsafe { (vt::<IMFTopologyNodeVT>(self).SetTopoNodeID)(self.ptr(), topo_id) })
			.to_hrresult()
	}
}
