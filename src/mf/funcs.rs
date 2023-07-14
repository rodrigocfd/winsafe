#![allow(non_snake_case)]

use crate::{co, mf};
use crate::mf::decl::{IMFMediaSession, IMFTopology, IMFTopologyNode};
use crate::mf::privs::MF_VERSION;
use crate::ole::decl::HrResult;
use crate::ole::privs::ok_to_hrresult;
use crate::prelude::{mf_IMFAttributes, ole_IUnknown};

/// [`MFCreateMediaSession`](https://learn.microsoft.com/en-us/windows/win32/api/mfidl/nf-mfidl-mfcreatemediasession)
/// function.
#[must_use]
pub fn MFCreateMediaSession(
	configuration: Option<&impl mf_IMFAttributes>) -> HrResult<IMFMediaSession>
{
	let mut queried = unsafe { IMFMediaSession::null() };
	ok_to_hrresult(
		unsafe {
			mf::ffi::MFCreateMediaSession(
				configuration.map_or(std::ptr::null_mut(), |c| c.ptr()),
				queried.as_mut(),
			)
		},
	).map(|_| queried)
}

/// [`MFCreateTopology`](https://learn.microsoft.com/en-us/windows/win32/api/mfidl/nf-mfidl-mfcreatetopology)
/// function.
#[must_use]
pub fn MFCreateTopology() -> HrResult<IMFTopology> {
	let mut queried = unsafe { IMFTopology::null() };
	ok_to_hrresult(unsafe { mf::ffi::MFCreateTopology(queried.as_mut()) })
		.map(|_| queried)
}

/// [`MFCreateTopologyNode`](https://learn.microsoft.com/en-us/windows/win32/api/mfidl/nf-mfidl-mfcreatetopologynode)
/// function.
#[must_use]
pub fn MFCreateTopologyNode(
	node_type: co::MF_TOPOLOGY,
) -> HrResult<IMFTopologyNode>
{
	let mut queried = unsafe { IMFTopologyNode::null() };
	ok_to_hrresult(
		unsafe {
			mf::ffi::MFCreateTopologyNode(node_type.raw(), queried.as_mut())
		},
	).map(|_| queried)
}

/// [`MFStartup`](https://learn.microsoft.com/en-us/windows/win32/api/mfapi/nf-mfapi-mfstartup)
/// function.
pub fn MFStartup(flags: co::MFSTARTUP) -> HrResult<()> {
	ok_to_hrresult(unsafe { mf::ffi::MFStartup(MF_VERSION, flags.raw()) })
}
