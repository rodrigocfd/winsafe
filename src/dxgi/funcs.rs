#![allow(non_snake_case)]

use crate::decl::*;
use crate::dxgi::ffi;
use crate::kernel::privs::*;
use crate::ole::privs::*;
use crate::prelude::*;

/// [`CreateDXGIFactory`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi/nf-dxgi-createdxgifactory)
/// function.
#[must_use]
pub fn CreateDXGIFactory() -> HrResult<IDXGIFactory> {
	let mut queried = unsafe { IDXGIFactory::null() };
	ok_to_hrresult(unsafe { ffi::CreateDXGIFactory(pcvoid(&IDXGIFactory::IID), queried.as_mut()) })
		.map(|_| queried)
}

/// [`CreateDXGIFactory1`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi/nf-dxgi-createdxgifactory1)
/// function.
#[must_use]
pub fn CreateDXGIFactory1() -> HrResult<IDXGIFactory1> {
	let mut queried = unsafe { IDXGIFactory1::null() };
	ok_to_hrresult(unsafe {
		ffi::CreateDXGIFactory1(pcvoid(&IDXGIFactory1::IID), queried.as_mut())
	})
	.map(|_| queried)
}
