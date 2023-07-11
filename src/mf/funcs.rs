#![allow(non_snake_case)]

use crate::mf;
use crate::mf::decl::IMFMediaSession;
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
