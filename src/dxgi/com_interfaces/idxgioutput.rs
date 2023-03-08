#![allow(non_camel_case_types, non_snake_case)]

use crate::dxgi::decl::{
	DXGI_FRAME_STATISTICS, DXGI_GAMMA_CONTROL, DXGI_GAMMA_CONTROL_CAPABILITIES,
};
use crate::kernel::ffi_types::{BOOL, HRES, PCVOID, PVOID};
use crate::ole::decl::{ComPtr, HrResult};
use crate::ole::privs::ok_to_hrresult;
use crate::prelude::{dxgi_IDXGIObject, ole_IUnknown};
use crate::vt::IDXGIObjectVT;

/// [`IDXGIAdapter`](crate::IDXGIAdapter) virtual table.
#[repr(C)]
pub struct IDXGIOutputVT {
	pub IDXGIObjectVT: IDXGIObjectVT,
	pub GetDesc: fn(ComPtr, PVOID) -> HRES,
	pub GetDisplayModeList: fn(ComPtr, u32, u32, *mut u32, PVOID) -> HRES,
	pub FindClosestMatchingMode: fn(ComPtr, PCVOID, PVOID, ComPtr) -> HRES,
	pub WaitForVBlank: fn(ComPtr) -> HRES,
	pub TakeOwnership: fn(ComPtr, ComPtr, BOOL) -> HRES,
	pub ReleaseOwnership: fn(ComPtr),
	pub GetGammaControlCapabilities: fn(ComPtr, PVOID) -> HRES,
	pub SetGammaControl: fn(ComPtr, PCVOID) -> HRES,
	pub GetGammaControl: fn(ComPtr, PVOID) -> HRES,
	pub SetDisplaySurface: fn(ComPtr, ComPtr) -> HRES,
	pub GetDisplaySurfaceData: fn(ComPtr, ComPtr) -> HRES,
	pub GetFrameStatistics: fn(ComPtr, PVOID) -> HRES,
}

com_interface! { IDXGIOutput: "ae02eedb-c735-4690-8d52-5a8dc20213aa";
	/// [`IDXGIOutput`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi/nn-dxgi-idxgioutput)
	/// COM interface over [`IDXGIOutputVT`](crate::vt::IDXGIOutputVT).
	///
	/// Automatically calls
	/// [`Release`](https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
}

impl dxgi_IDXGIObject for IDXGIOutput {}
impl dxgi_IDXGIOutput for IDXGIOutput {}

/// This trait is enabled with the `dxgi` feature, and provides methods for
/// [`IDXGIOutput`](crate::IDXGIOutput).
///
/// Prefer importing this trait through the prelude:
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// ```
pub trait dxgi_IDXGIOutput: dxgi_IDXGIObject {
	/// [`IDXGIOutput::GetFrameStatistics`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi/nf-dxgi-idxgioutput-getframestatistics)
	/// method.
	fn GetFrameStatistics(&self,
		stats: &mut DXGI_FRAME_STATISTICS) -> HrResult<()>
	{
		unsafe {
			let vt = self.vt_ref::<IDXGIOutputVT>();
			ok_to_hrresult(
				(vt.GetFrameStatistics)(self.ptr(), stats as *mut _ as _),
			)
		}
	}

	/// [`IDXGIOutput::GetGammaControl`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi/nf-dxgi-idxgioutput-getgammacontrol)
	/// method.
	fn GetGammaControl(&self, array: &mut DXGI_GAMMA_CONTROL) -> HrResult<()> {
		unsafe {
			let vt = self.vt_ref::<IDXGIOutputVT>();
			ok_to_hrresult(
				(vt.GetGammaControl)(self.ptr(), array as *mut _ as _),
			)
		}
	}

	/// [`IDXGIOutput::GetGammaControlCapabilities`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi/nf-dxgi-idxgioutput-getgammacontrolcapabilities)
	/// method.
	fn GetGammaControlCapabilities(&self,
		gamma_caps: &mut DXGI_GAMMA_CONTROL_CAPABILITIES) -> HrResult<()>
	{
		unsafe {
			let vt = self.vt_ref::<IDXGIOutputVT>();
			ok_to_hrresult(
				(vt.GetGammaControlCapabilities)(
					self.ptr(),
					gamma_caps as *mut _ as _,
				),
			)
		}
	}

	/// [`IDXGIOutput::ReleaseOwnership`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi/nf-dxgi-idxgioutput-releaseownership)
	/// method.
	fn ReleaseOwnership(&self) {
		unsafe {
			let vt = self.vt_ref::<IDXGIOutputVT>();
			(vt.ReleaseOwnership)(self.ptr());
		}
	}

	/// [`IDXGIOutput::SetGammaControl`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi/nf-dxgi-idxgioutput-setgammacontrol)
	/// method.
	fn SetGammaControl(&self, array: &DXGI_GAMMA_CONTROL) -> HrResult<()> {
		unsafe {
			let vt = self.vt_ref::<IDXGIOutputVT>();
			ok_to_hrresult(
				(vt.SetGammaControl)(self.ptr(), array as *const _ as _),
			)
		}
	}

	/// [`IDXGIOutput::TakeOwnership`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi/nf-dxgi-idxgioutput-takeownership)
	/// method.
	fn TakeOwnership<T>(&self, device: &T, exclusive: bool) -> HrResult<()>
		where T: ole_IUnknown,
	{
		unsafe {
			let vt = self.vt_ref::<IDXGIOutputVT>();
			ok_to_hrresult(
				(vt.TakeOwnership)(self.ptr(), device.ptr(), exclusive as _),
			)
		}
	}

	/// [`IDXGIOutput::WaitForVBlank`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi/nf-dxgi-idxgioutput-waitforvblank)
	/// method.
	fn WaitForVBlank(&self) -> HrResult<()> {
		unsafe {
			let vt = self.vt_ref::<IDXGIOutputVT>();
			ok_to_hrresult((vt.WaitForVBlank)(self.ptr()))
		}
	}
}
