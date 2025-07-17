#![allow(non_snake_case)]

use crate::co;
use crate::decl::*;
use crate::guard::*;
use crate::mf::{ffi, privs::*};
use crate::ole::privs::*;
use crate::prelude::*;

/// [`MFCreateAsyncResult`](https://learn.microsoft.com/en-us/windows/win32/api/mfapi/nf-mfapi-mfcreateasyncresult)
/// function.
#[must_use]
pub fn MFCreateAsyncResult(
	object: Option<&impl ole_IUnknown>,
	callback: &IMFAsyncCallback,
	state: Option<&impl ole_IUnknown>,
) -> HrResult<IMFAsyncResult> {
	let mut queried = unsafe { IMFAsyncResult::null() };
	HrRet(unsafe {
		ffi::MFCreateAsyncResult(
			object.map_or(std::ptr::null_mut(), |o| o.ptr()),
			callback.ptr(),
			state.map_or(std::ptr::null_mut(), |s| s.ptr()),
			queried.as_mut(),
		)
	})
	.to_hrresult()
	.map(|_| queried)
}

/// [`MFCreateMediaSession`](https://learn.microsoft.com/en-us/windows/win32/api/mfidl/nf-mfidl-mfcreatemediasession)
/// function.
///
/// # Examples
///
/// ```no_run
/// use winsafe::{self as w, prelude::*};
///
/// let media_session = w::MFCreateMediaSession(None::<&w::IMFAttributes>)?;
/// # w::HrResult::Ok(())
/// ```
#[must_use]
pub fn MFCreateMediaSession(
	configuration: Option<&impl mf_IMFAttributes>,
) -> HrResult<IMFMediaSession> {
	let mut queried = unsafe { IMFMediaSession::null() };
	HrRet(unsafe {
		ffi::MFCreateMediaSession(
			configuration.map_or(std::ptr::null_mut(), |c| c.ptr()),
			queried.as_mut(),
		)
	})
	.to_hrresult()
	.map(|_| queried)
}

/// [`MFCreateMFByteStreamOnStream`](https://learn.microsoft.com/en-us/windows/win32/api/mfidl/nf-mfidl-mfcreatemfbytestreamonstream)
/// function.
///
/// # Examples
///
/// ```no_run
/// use winsafe::{self as w, prelude::*};
///
/// let raw_data: Vec<u8>; // initialized somewhere
/// # let raw_data = Vec::<u8>::new();
///
/// let stream = w::SHCreateMemStream(&raw_data)?;
/// let byte_stream = w::MFCreateMFByteStreamOnStream(&stream)?;
/// # w::HrResult::Ok(())
/// ```
#[must_use]
pub fn MFCreateMFByteStreamOnStream(stream: &impl ole_IStream) -> HrResult<IMFByteStream> {
	let mut queried = unsafe { IMFByteStream::null() };
	HrRet(unsafe { ffi::MFCreateMFByteStreamOnStream(stream.ptr(), queried.as_mut()) })
		.to_hrresult()
		.map(|_| queried)
}

/// [`MFCreateSourceResolver`](https://learn.microsoft.com/en-us/windows/win32/api/mfidl/nf-mfidl-mfcreatesourceresolver)
/// function.
///
/// # Examples
///
/// ```no_run
/// use winsafe::{self as w, prelude::*};
///
/// let source_resolver = w::MFCreateSourceResolver()?;
/// # w::HrResult::Ok(())
/// ```
#[must_use]
pub fn MFCreateSourceResolver() -> HrResult<IMFSourceResolver> {
	let mut queried = unsafe { IMFSourceResolver::null() };
	HrRet(unsafe { ffi::MFCreateSourceResolver(queried.as_mut()) })
		.to_hrresult()
		.map(|_| queried)
}

/// [`MFCreateTopology`](https://learn.microsoft.com/en-us/windows/win32/api/mfidl/nf-mfidl-mfcreatetopology)
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
#[must_use]
pub fn MFCreateTopology() -> HrResult<IMFTopology> {
	let mut queried = unsafe { IMFTopology::null() };
	HrRet(unsafe { ffi::MFCreateTopology(queried.as_mut()) })
		.to_hrresult()
		.map(|_| queried)
}

/// [`MFCreateTopologyNode`](https://learn.microsoft.com/en-us/windows/win32/api/mfidl/nf-mfidl-mfcreatetopologynode)
/// function.
///
/// # Examples
///
/// ```no_run
/// use winsafe::{self as w, prelude::*, co};
///
/// let topology_node = w::MFCreateTopologyNode(co::MF_TOPOLOGY::OUTPUT_NODE)?;
/// # w::HrResult::Ok(())
/// ```
#[must_use]
pub fn MFCreateTopologyNode(node_type: co::MF_TOPOLOGY) -> HrResult<IMFTopologyNode> {
	let mut queried = unsafe { IMFTopologyNode::null() };
	HrRet(unsafe { ffi::MFCreateTopologyNode(node_type.raw(), queried.as_mut()) })
		.to_hrresult()
		.map(|_| queried)
}

/// [`MFStartup`](https://learn.microsoft.com/en-us/windows/win32/api/mfapi/nf-mfapi-mfstartup)
/// function.
///
/// In the original C implementation, you must call
/// [`MFShutdown`](https://learn.microsoft.com/en-us/windows/win32/api/mfapi/nf-mfapi-mfshutdown)
/// as a cleanup operation.
///
/// Here, the cleanup is performed automatically, because `MFStartup` returns a
/// [`MFShutdownGuard`](crate::guard::MFShutdownGuard), which automatically
/// calls `MFShutdown` when the guard goes out of scope. You must, however, keep
/// the guard alive, otherwise the cleanup will be performed right away.
///
/// # Examples
///
/// ```no_run
/// use winsafe::{self as w, prelude::*, co};
///
/// let _mf_guard = w::MFStartup( // keep guard alive
///     co::MFSTARTUP::NOSOCKET,
/// )?;
///
/// // program runs...
///
/// // MFShutdown() automatically called
/// # w::HrResult::Ok(())
/// ```
pub fn MFStartup(flags: co::MFSTARTUP) -> HrResult<MFShutdownGuard> {
	unsafe {
		HrRet(ffi::MFStartup(MF_VERSION, flags.raw()))
			.to_hrresult()
			.map(|_| MFShutdownGuard::new())
	}
}
