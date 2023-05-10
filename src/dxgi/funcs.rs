#![allow(non_snake_case)]

use crate::dxgi;
use crate::dxgi::decl::IDXGIFactory;
use crate::ole::decl::HrResult;
use crate::ole::privs::ok_to_hrresult;
use crate::prelude::ole_IUnknown;

/// [`CreateDXGIFactory`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi/nf-dxgi-createdxgifactory)
/// function.
#[must_use]
pub fn CreateDXGIFactory() -> HrResult<IDXGIFactory> {
	let mut queried = unsafe { IDXGIFactory::null() };
	ok_to_hrresult(
		unsafe {
			dxgi::ffi::CreateDXGIFactory(
				&IDXGIFactory::IID as *const _ as _,
				queried.as_mut(),
			)
		},
	).map(|_| queried)
}
