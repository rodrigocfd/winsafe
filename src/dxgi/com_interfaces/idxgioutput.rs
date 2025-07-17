#![allow(non_camel_case_types, non_snake_case)]

use crate::co;
use crate::decl::*;
use crate::dxgi::vts::*;
use crate::kernel::privs::*;
use crate::ole::privs::*;
use crate::prelude::*;

com_interface! { IDXGIOutput: "ae02eedb-c735-4690-8d52-5a8dc20213aa";
	/// [`IDXGIOutput`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi/nn-dxgi-idxgioutput)
	/// COM interface.
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
/// ```no_run
/// use winsafe::prelude::*;
/// ```
pub trait dxgi_IDXGIOutput: dxgi_IDXGIObject {
	/// [`IDXGIOutput::FindClosestMatchingMode`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi/nf-dxgi-idxgioutput-findclosestmatchingmode)
	/// method.
	#[must_use]
	fn FindClosestMatchingMode(
		&self,
		mode_to_match: &DXGI_MODE_DESC,
		device_interface: Option<&impl ole_IUnknown>,
	) -> HrResult<DXGI_MODE_DESC> {
		let mut closest_match = DXGI_MODE_DESC::default();
		HrRet(unsafe {
			(vt::<IDXGIOutputVT>(self).FindClosestMatchingMode)(
				self.ptr(),
				pcvoid(mode_to_match),
				pvoid(&mut closest_match),
				device_interface.map_or(std::ptr::null_mut(), |p| p.ptr()),
			)
		}).to_hrresult()
		.map(|_| closest_match)
	}

	/// [`IDXGIOutput::GetDesc`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi/nf-dxgi-idxgioutput-getdesc)
	/// method.
	#[must_use]
	fn GetDesc(&self) -> HrResult<DXGI_OUTPUT_DESC> {
		let mut desc = DXGI_OUTPUT_DESC::default();
		HrRet(unsafe { (vt::<IDXGIOutputVT>(self).GetDesc)(self.ptr(), pvoid(&mut desc)) })
		.to_hrresult()
			.map(|_| desc)
	}

	/// [`IDXGIOutput::GetDisplayModeList`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi/nf-dxgi-idxgioutput-getdisplaymodelist)
	/// method.
	#[must_use]
	fn GetDisplayModeList(
		&self,
		format: co::DXGI_FORMAT,
		flags: co::DXGI_ENUM_MODES,
	) -> HrResult<Vec<DXGI_MODE_DESC>> {
		let mut num_modes = 0u32;
		HrRet(unsafe {
			(vt::<IDXGIOutputVT>(self).GetDisplayModeList)(
				self.ptr(),
				format.raw(),
				flags.raw(),
				&mut num_modes,
				std::ptr::null_mut(),
			)
		}).to_hrresult()?;

		let mut modes = vec![DXGI_MODE_DESC::default(); num_modes as _];
		HrRet(unsafe {
			(vt::<IDXGIOutputVT>(self).GetDisplayModeList)(
				self.ptr(),
				format.raw(),
				flags.raw(),
				&mut num_modes,
				modes.as_mut_ptr() as _,
			)
		}).to_hrresult()
		.map(|_| modes)
	}

	/// [`IDXGIOutput::GetDisplaySurfaceData`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi/nf-dxgi-idxgioutput-getdisplaysurfacedata)
	/// method.
	fn GetDisplaySurfaceData(&self, destination: &impl dxgi_IDXGISurface) -> HrResult<()> {
		HrRet(unsafe {
			(vt::<IDXGIOutputVT>(self).GetDisplaySurfaceData)(self.ptr(), destination.ptr())
		}).to_hrresult()
	}

	/// [`IDXGIOutput::GetFrameStatistics`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi/nf-dxgi-idxgioutput-getframestatistics)
	/// method.
	#[must_use]
	fn GetFrameStatistics(&self) -> HrResult<DXGI_FRAME_STATISTICS> {
		let mut stats = DXGI_FRAME_STATISTICS::default();
		HrRet(unsafe {
			(vt::<IDXGIOutputVT>(self).GetFrameStatistics)(self.ptr(), pvoid(&mut stats))
		}).to_hrresult()
		.map(|_| stats)
	}

	/// [`IDXGIOutput::GetGammaControl`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi/nf-dxgi-idxgioutput-getgammacontrol)
	/// method.
	#[must_use]
	fn GetGammaControl(&self) -> HrResult<DXGI_GAMMA_CONTROL> {
		let mut array = DXGI_GAMMA_CONTROL::default();
		HrRet(unsafe {
			(vt::<IDXGIOutputVT>(self).GetGammaControl)(self.ptr(), pvoid(&mut array))
		}).to_hrresult()
		.map(|_| array)
	}

	/// [`IDXGIOutput::GetGammaControlCapabilities`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi/nf-dxgi-idxgioutput-getgammacontrolcapabilities)
	/// method.
	#[must_use]
	fn GetGammaControlCapabilities(&self) -> HrResult<DXGI_GAMMA_CONTROL_CAPABILITIES> {
		let mut capa = DXGI_GAMMA_CONTROL_CAPABILITIES::default();
		HrRet(unsafe {
			(vt::<IDXGIOutputVT>(self).GetGammaControlCapabilities)(self.ptr(), pvoid(&mut capa))
		}).to_hrresult()
		.map(|_| capa)
	}

	/// [`IDXGIOutput::ReleaseOwnership`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi/nf-dxgi-idxgioutput-releaseownership)
	/// method.
	fn ReleaseOwnership(&self) {
		unsafe { (vt::<IDXGIOutputVT>(self).ReleaseOwnership)(self.ptr()) };
	}

	/// [`IDXGIOutput::SetDisplaySurface`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi/nf-dxgi-idxgioutput-setdisplaysurface)
	/// method.
	fn SetDisplaySurface(&self, scanout_surface: &impl dxgi_IDXGISurface) -> HrResult<()> {
		HrRet(unsafe {
			(vt::<IDXGIOutputVT>(self).SetDisplaySurface)(self.ptr(), scanout_surface.ptr())
		}).to_hrresult()
	}

	/// [`IDXGIOutput::SetGammaControl`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi/nf-dxgi-idxgioutput-setgammacontrol)
	/// method.
	fn SetGammaControl(&self, array: &DXGI_GAMMA_CONTROL) -> HrResult<()> {
		HrRet(unsafe {
			(vt::<IDXGIOutputVT>(self).SetGammaControl)(self.ptr(), pcvoid(array))
		}).to_hrresult()
	}

	/// [`IDXGIOutput::TakeOwnership`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi/nf-dxgi-idxgioutput-takeownership)
	/// method.
	fn TakeOwnership(&self, device: &impl ole_IUnknown, exclusive: bool) -> HrResult<()> {
		HrRet(unsafe {
			(vt::<IDXGIOutputVT>(self).TakeOwnership)(self.ptr(), device.ptr(), exclusive as _)
		}).to_hrresult()
	}

	fn_com_noparm! { WaitForVBlank: IDXGIOutputVT;
		/// [`IDXGIOutput::WaitForVBlank`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi/nf-dxgi-idxgioutput-waitforvblank)
		/// method.
	}
}
