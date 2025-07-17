#![allow(non_camel_case_types, non_snake_case)]

use crate::co;
use crate::decl::*;
use crate::dxgi::vts::*;
use crate::kernel::privs::*;
use crate::ole::privs::*;
use crate::prelude::*;

com_interface! { IDXGISwapChain: "310d36a0-d2e7-4c0a-aa04-6a9d23b8886a";
	/// [`IDXGISwapChain`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi/nn-dxgi-idxgiswapchain)
	/// COM interface.
	///
	/// Automatically calls
	/// [`Release`](https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
}

impl dxgi_IDXGIObject for IDXGISwapChain {}
impl dxgi_IDXGIDeviceSubObject for IDXGISwapChain {}
impl dxgi_IDXGISwapChain for IDXGISwapChain {}

/// This trait is enabled with the `dxgi` feature, and provides methods for
/// [`IDXGISwapChain`](crate::IDXGISwapChain).
///
/// Prefer importing this trait through the prelude:
///
/// ```no_run
/// use winsafe::prelude::*;
/// ```
pub trait dxgi_IDXGISwapChain: dxgi_IDXGIDeviceSubObject {
	/// [`IDXGISwapChain::GetBuffer`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi/nf-dxgi-idxgiswapchain-getbuffer)
	/// method.
	#[must_use]
	fn GetBuffer<T: ole_IUnknown>(&self, buffer_index: u32) -> HrResult<T> {
		let mut queried = unsafe { T::null() };
		HrRet(unsafe {
			(vt::<IDXGISwapChainVT>(self).GetBuffer)(
				self.ptr(),
				buffer_index,
				pcvoid(&T::IID),
				queried.as_mut(),
			)
		})
		.to_hrresult()
		.map(|_| queried)
	}

	fn_com_interface_get! { GetContainingOutput: IDXGISwapChainVT => IDXGIOutput;
		/// [`IDXGISwapChain::GetContainingOutput`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi/nf-dxgi-idxgiswapchain-getcontainingoutput)
		/// method.
	}

	/// [`IDXGISwapChain::GetDesc`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi/nf-dxgi-idxgiswapchain-getdesc)
	/// method.
	#[must_use]
	fn GetDesc(&self) -> HrResult<DXGI_SWAP_CHAIN_DESC> {
		let mut desc = DXGI_SWAP_CHAIN_DESC::default();
		HrRet(unsafe { (vt::<IDXGISwapChainVT>(self).GetDesc)(self.ptr(), pvoid(&mut desc)) })
			.to_hrresult()
			.map(|_| desc)
	}

	/// [`IDXGISwapChain::GetFrameStatistics`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi/nf-dxgi-idxgiswapchain-getframestatistics)
	/// method.
	fn GetFrameStatistics(&self) -> HrResult<DXGI_FRAME_STATISTICS> {
		let mut stats = DXGI_FRAME_STATISTICS::default();
		HrRet(unsafe { (vt::<IDXGISwapChainVT>(self).GetDesc)(self.ptr(), pvoid(&mut stats)) })
			.to_hrresult()
			.map(|_| stats)
	}

	/// [`IDXGISwapChain::GetFullscreenState`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi/nf-dxgi-idxgiswapchain-getfullscreenstate)
	/// method.
	#[must_use]
	fn GetFullscreenState(&self) -> HrResult<(bool, Option<IDXGIOutput>)> {
		let mut fullscreen = 0;
		let mut queried = unsafe { IDXGIOutput::null() };

		HrRet(unsafe {
			(vt::<IDXGISwapChainVT>(self).GetFullscreenState)(
				self.ptr(),
				&mut fullscreen,
				queried.as_mut(),
			)
		})
		.to_hrresult()
		.map(|_| (fullscreen != 0, if queried.ptr().is_null() { None } else { Some(queried) }))
	}

	/// [`IDXGISwapChain::GetLastPresentCount`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi/nf-dxgi-idxgiswapchain-getlastpresentcount)
	/// method.
	#[must_use]
	fn GetLastPresentCount(&self) -> HrResult<u32> {
		let mut count = 0u32;
		HrRet(unsafe { (vt::<IDXGISwapChainVT>(self).GetLastPresentCount)(self.ptr(), &mut count) })
			.to_hrresult()
			.map(|_| count)
	}

	/// [`IDXGISwapChain::Present`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi/nf-dxgi-idxgiswapchain-present)
	/// method.
	fn Present(&self, sync_interval: u32, flags: co::DXGI_PRESENT) -> HrResult<()> {
		HrRet(unsafe {
			(vt::<IDXGISwapChainVT>(self).Present)(self.ptr(), sync_interval, flags.raw())
		})
		.to_hrresult()
	}

	/// [`IDXGISwapChain::ResizeBuffers`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi/nf-dxgi-idxgiswapchain-resizebuffers)
	/// method.
	fn ResizeBuffers(
		&self,
		buffer_count: u32,
		width: u32,
		height: u32,
		new_format: co::DXGI_FORMAT,
		swap_chain_flags: co::DXGI_SWAP_CHAIN_FLAG,
	) -> HrResult<()> {
		HrRet(unsafe {
			(vt::<IDXGISwapChainVT>(self).ResizeBuffers)(
				self.ptr(),
				buffer_count,
				width,
				height,
				new_format.raw(),
				swap_chain_flags.raw(),
			)
		})
		.to_hrresult()
	}

	/// [`IDXGISwapChain::ResizeTarget`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi/nf-dxgi-idxgiswapchain-resizetarget)
	/// method.
	fn ResizeTarget(&self, new_target_parameters: &DXGI_MODE_DESC) -> HrResult<()> {
		HrRet(unsafe {
			(vt::<IDXGISwapChainVT>(self).ResizeTarget)(self.ptr(), pcvoid(new_target_parameters))
		})
		.to_hrresult()
	}

	/// [`IDXGISwapChain::SetFullscreenState`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi/nf-dxgi-idxgiswapchain-setfullscreenstate)
	/// method.
	fn SetFullscreenState(
		&self,
		fullscreen: bool,
		target: Option<&impl dxgi_IDXGIOutput>,
	) -> HrResult<()> {
		HrRet(unsafe {
			(vt::<IDXGISwapChainVT>(self).SetFullscreenState)(
				self.ptr(),
				fullscreen as _,
				target.map_or(std::ptr::null_mut(), |t| t.ptr()),
			)
		})
		.to_hrresult()
	}
}
