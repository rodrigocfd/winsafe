#![allow(non_camel_case_types, non_snake_case)]

use crate::co;
use crate::dxgi::decl::{
	DXGI_FRAME_STATISTICS, DXGI_GAMMA_CONTROL, DXGI_GAMMA_CONTROL_CAPABILITIES,
	DXGI_MODE_DESC, DXGI_OUTPUT_DESC,
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
	/// [`IDXGIOutput::FindClosestMatchingMode`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi/nf-dxgi-idxgioutput-findclosestmatchingmode)
	/// method.
	#[must_use]
	fn FindClosestMatchingMode(&self,
		mode_to_match: &DXGI_MODE_DESC,
		device_interface: Option<&impl ole_IUnknown>,
	) -> HrResult<DXGI_MODE_DESC>
	{
		let mut closest_match = DXGI_MODE_DESC::default();
		unsafe {
			let vt = self.vt_ref::<IDXGIOutputVT>();
			ok_to_hrresult(
				(vt.FindClosestMatchingMode)(
					self.ptr(),
					mode_to_match as *const _ as _,
					&mut closest_match as *mut _ as _,
					device_interface.map_or(ComPtr::null(), |p| p.ptr()),
				),
			)
		}.map(|_| closest_match)
	}

	/// [`IDXGIOutput::GetDesc`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi/nf-dxgi-idxgioutput-getdesc)
	/// method.
	#[must_use]
	fn GetDesc(&self) -> HrResult<DXGI_OUTPUT_DESC> {
		let mut desc = DXGI_OUTPUT_DESC::default();
		unsafe {
			let vt = self.vt_ref::<IDXGIOutputVT>();
			ok_to_hrresult(
				(vt.GetDesc)(self.ptr(), &mut desc as *mut _ as _),
			)
		}.map(|_| desc)
	}

	/// [`IDXGIOutput::GetDisplayModeList`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi/nf-dxgi-idxgioutput-getdisplaymodelist)
	/// method.
	#[must_use]
	fn GetDisplayModeList(&self,
		format: co::DXGI_FORMAT,
		flags: co::DXGI_ENUM_MODES,
	) -> HrResult<Vec<DXGI_MODE_DESC>>
	{
		let mut num_modes = u32::default();
		unsafe {
			let vt = self.vt_ref::<IDXGIOutputVT>();
			ok_to_hrresult(
				(vt.GetDisplayModeList)(
					self.ptr(),
					format.raw(),
					flags.raw(),
					&mut num_modes,
					std::ptr::null_mut(),
				),
			)?;
		}

		let mut modes = vec![DXGI_MODE_DESC::default(); num_modes as _];
		unsafe {
			let vt = self.vt_ref::<IDXGIOutputVT>();
			ok_to_hrresult(
				(vt.GetDisplayModeList)(
					self.ptr(),
					format.raw(),
					flags.raw(),
					&mut num_modes,
					modes.as_mut_ptr() as _,
				),
			)
		}.map(|_| modes)
	}

	/// [`IDXGIOutput::GetFrameStatistics`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi/nf-dxgi-idxgioutput-getframestatistics)
	/// method.
	#[must_use]
	fn GetFrameStatistics(&self) -> HrResult<DXGI_FRAME_STATISTICS> {
		let mut stats = DXGI_FRAME_STATISTICS::default();
		unsafe {
			let vt = self.vt_ref::<IDXGIOutputVT>();
			ok_to_hrresult(
				(vt.GetFrameStatistics)(self.ptr(), &mut stats as *mut _ as _),
			)
		}.map(|_| stats)
	}

	/// [`IDXGIOutput::GetGammaControl`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi/nf-dxgi-idxgioutput-getgammacontrol)
	/// method.
	#[must_use]
	fn GetGammaControl(&self) -> HrResult<DXGI_GAMMA_CONTROL> {
		let mut array = DXGI_GAMMA_CONTROL::default();
		unsafe {
			let vt = self.vt_ref::<IDXGIOutputVT>();
			ok_to_hrresult(
				(vt.GetGammaControl)(self.ptr(), &mut array as *mut _ as _),
			)
		}.map(|_| array)
	}

	/// [`IDXGIOutput::GetGammaControlCapabilities`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi/nf-dxgi-idxgioutput-getgammacontrolcapabilities)
	/// method.
	#[must_use]
	fn GetGammaControlCapabilities(&self,
	) -> HrResult<DXGI_GAMMA_CONTROL_CAPABILITIES>
	{
		let mut capa = DXGI_GAMMA_CONTROL_CAPABILITIES::default();
		unsafe {
			let vt = self.vt_ref::<IDXGIOutputVT>();
			ok_to_hrresult(
				(vt.GetGammaControlCapabilities)(
					self.ptr(),
					&mut capa as *mut _ as _,
				),
			)
		}.map(|_| capa)
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
	fn TakeOwnership(&self,
		device: &impl ole_IUnknown, exclusive: bool) -> HrResult<()>
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
