#![allow(non_snake_case)]

use crate::co;
use crate::decl::*;
use crate::mf::{ffi, privs::*};
use crate::ole::privs::*;
use crate::prelude::*;

/// [`MFCreateMediaSession`](https://learn.microsoft.com/en-us/windows/win32/api/mfidl/nf-mfidl-mfcreatemediasession)
/// function.
///
/// # Examples
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// use winsafe::{IMFAttributes, MFCreateMediaSession};
///
/// let media_session = MFCreateMediaSession(None::<&IMFAttributes>)?;
/// # Ok::<_, winsafe::co::HRESULT>(())
/// ```
#[must_use]
pub fn MFCreateMediaSession(
	configuration: Option<&impl mf_IMFAttributes>,
) -> HrResult<IMFMediaSession>
{
	let mut queried = unsafe { IMFMediaSession::null() };
	ok_to_hrresult(
		unsafe {
			ffi::MFCreateMediaSession(
				configuration.map_or(std::ptr::null_mut(), |c| c.ptr()),
				queried.as_mut(),
			)
		},
	).map(|_| queried)
}

/// [`MFCreateSourceResolver`](https://learn.microsoft.com/en-us/windows/win32/api/mfidl/nf-mfidl-mfcreatesourceresolver)
/// function.
///
/// # Examples
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// use winsafe::{IMFSourceResolver, MFCreateSourceResolver};
///
/// let source_resolver = MFCreateSourceResolver()?;
/// # Ok::<_, winsafe::co::HRESULT>(())
/// ```
#[must_use]
pub fn MFCreateSourceResolver() -> HrResult<IMFSourceResolver> {
	let mut queried = unsafe { IMFSourceResolver::null() };
	ok_to_hrresult(unsafe { ffi::MFCreateSourceResolver(queried.as_mut()) })
		.map(|_| queried)
}

/// [`MFCreateTopology`](https://learn.microsoft.com/en-us/windows/win32/api/mfidl/nf-mfidl-mfcreatetopology)
/// function.
///
/// # Examples
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// use winsafe::{IMFTopology, MFCreateTopology};
///
/// let topology = MFCreateTopology()?;
/// # Ok::<_, winsafe::co::HRESULT>(())
/// ```
#[must_use]
pub fn MFCreateTopology() -> HrResult<IMFTopology> {
	let mut queried = unsafe { IMFTopology::null() };
	ok_to_hrresult(unsafe { ffi::MFCreateTopology(queried.as_mut()) })
		.map(|_| queried)
}

/// [`MFCreateTopologyNode`](https://learn.microsoft.com/en-us/windows/win32/api/mfidl/nf-mfidl-mfcreatetopologynode)
/// function.
///
/// # Examples
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// use winsafe::{co, IMFTopologyNode, MFCreateTopologyNode};
///
/// let topology_node = MFCreateTopologyNode(co::MF_TOPOLOGY::OUTPUT_NODE)?;
/// # Ok::<_, winsafe::co::HRESULT>(())
/// ```
#[must_use]
pub fn MFCreateTopologyNode(
	node_type: co::MF_TOPOLOGY,
) -> HrResult<IMFTopologyNode>
{
	let mut queried = unsafe { IMFTopologyNode::null() };
	ok_to_hrresult(
		unsafe { ffi::MFCreateTopologyNode(node_type.raw(), queried.as_mut()) },
	).map(|_| queried)
}

/// [`MFStartup`](https://learn.microsoft.com/en-us/windows/win32/api/mfapi/nf-mfapi-mfstartup)
/// function.
pub fn MFStartup(flags: co::MFSTARTUP) -> HrResult<()> {
	ok_to_hrresult(unsafe { ffi::MFStartup(MF_VERSION, flags.raw()) })
}
