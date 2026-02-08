#![allow(non_snake_case)]

use crate::co;
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
	HrRet(unsafe { ffi::CreateDXGIFactory(pcvoid(&IDXGIFactory::IID), queried.as_mut()) })
		.to_hrresult()
		.map(|_| queried)
}

/// [`CreateDXGIFactory1`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi/nf-dxgi-createdxgifactory1)
/// function.
#[must_use]
pub fn CreateDXGIFactory1() -> HrResult<IDXGIFactory1> {
	let mut queried = unsafe { IDXGIFactory1::null() };
	HrRet(unsafe { ffi::CreateDXGIFactory1(pcvoid(&IDXGIFactory1::IID), queried.as_mut()) })
		.to_hrresult()
		.map(|_| queried)
}

/// [`CreateDXGIFactory2`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi1_3/nf-dxgi1_3-createdxgifactory2)
/// function.
#[must_use]
pub fn CreateDXGIFactory2(flags: co::DXGI_CREATE_FACTORY) -> HrResult<IDXGIFactory2> {
	let mut queried = unsafe { IDXGIFactory2::null() };
	HrRet(unsafe {
		ffi::CreateDXGIFactory2(flags.raw(), pcvoid(&IDXGIFactory2::IID), queried.as_mut())
	})
	.to_hrresult()
	.map(|_| queried)
}
