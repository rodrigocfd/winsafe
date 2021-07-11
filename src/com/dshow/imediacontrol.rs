#![allow(non_snake_case)]

use crate::com::idispatch::IDispatchVT;
use crate::com::traits::{ComInterface, PPI};
use crate::ffi::{HRESULT, PSTR};
use crate::structs::IID;

/// [`IMediaControl`](crate::dshow::IMediaControl) virtual table.
pub struct IMediaControlVT {
	pub IDispatchVT: IDispatchVT,
	pub Run: fn(PPI) -> HRESULT,
	pub Pause: fn(PPI) -> HRESULT,
	pub Stop: fn(PPI) -> HRESULT,
	pub GetState: fn(PPI, i32, *mut u32) -> HRESULT,
	pub RenderFile: fn(PPI, PSTR) -> HRESULT,
	pub AddSourceFilter: fn(PPI, PSTR, *mut PPI) -> HRESULT,
	pub GetFilterCollection: fn(PPI, *mut PPI) -> HRESULT,
	pub GetRegFilterCollection: fn(PPI, *mut PPI) -> HRESULT,
	pub StopWhenReady: fn(PPI) -> HRESULT,
}

/// [`IMediaControl`](https://docs.microsoft.com/en-us/windows/win32/api/control/nn-control-imediacontrol)
/// COM interface over
/// [`IMediaControlVT`](crate::dshow::vt::IMediaControlVT). Inherits from
/// [`IDispatch`](crate::IDispatch),
/// [`IUnknown`](crate::IUnknown).
///
/// Automatically calls
/// [`IUnknown::Release`](https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
/// when the object goes out of scope.
pub struct IMediaControl {
	pub(crate) ppvt: PPI,
}

impl_send_sync_fromppvt!(IMediaControl);

impl ComInterface for IMediaControl {
	const IID: IID = IID::new(0x56a868b1, 0x0ad4, 0x11ce, 0xb03a, 0x0020af0ba770);
}

macro_rules! impl_IMediaControl {
	($name:ty, $vt:ty) => {
		use crate::com::dshow::co as dshowco;
		use crate::com::IDispatch;
		use crate::privs::{hr_to_winresult_bool, INFINITE};
		use crate::various::WString;

		impl $name {
			fn imediacontrol_vt(&self) -> &IMediaControlVT {
				unsafe { &**(self.ppvt as *mut *mut _) }
			}

			/// [`IMediaControl::AddSourceFilter`](https://docs.microsoft.com/en-us/windows/win32/api/control/nf-control-imediacontrol-addsourcefilter)
			/// method.
			pub fn AddSourceFilter(&self, fileName: &str) -> WinResult<IDispatch> {
				let mut ppvQueried: PPI = std::ptr::null_mut();
				hr_to_winresult(
					(self.imediacontrol_vt().AddSourceFilter)(
						self.ppvt,
						unsafe { WString::from_str(fileName).as_mut_ptr() }, // BSTR
						&mut ppvQueried as *mut _ as _,
					),
				).map(|_| IDispatch::from(ppvQueried))
			}

			/// [`IMediaControl::GetState`](https://docs.microsoft.com/en-us/windows/win32/api/control/nf-control-imediacontrol-getstate)
			/// method.
			pub fn GetState(&self,
				msTimeout: Option<i32>) -> WinResult<dshowco::FILTER_STATE>
			{
				let mut state = dshowco::FILTER_STATE::Stopped;
				hr_to_winresult(
					(self.imediacontrol_vt().GetState)(
						self.ppvt,
						msTimeout.unwrap_or(INFINITE as _),
						&mut state.0,
					),
				).map(|_| state)
			}

			/// [`IMediaControl::Pause`](https://docs.microsoft.com/en-us/windows/win32/api/control/nf-control-imediacontrol-pause)
			/// method.
			pub fn Pause(&self) -> WinResult<bool> {
				hr_to_winresult_bool((self.imediacontrol_vt().Pause)(self.ppvt))
			}

			/// [`IMediaControl::RenderFile`](https://docs.microsoft.com/en-us/windows/win32/api/control/nf-control-imediacontrol-renderfile)
			/// method.
			pub fn RenderFile(&self, fileName: &str) -> WinResult<()> {
				hr_to_winresult(
					(self.imediacontrol_vt().RenderFile)(
						self.ppvt,
						unsafe { WString::from_str(fileName).as_mut_ptr() }, // BSTR
					),
				)
			}

			/// [`IMediaControl::Run`](https://docs.microsoft.com/en-us/windows/win32/api/control/nf-control-imediacontrol-run)
			/// method.
			pub fn Run(&self) -> WinResult<bool> {
				hr_to_winresult_bool((self.imediacontrol_vt().Run)(self.ppvt))
			}

			/// [`IMediaControl::Stop`](https://docs.microsoft.com/en-us/windows/win32/api/control/nf-control-imediacontrol-stop)
			/// method.
			pub fn Stop(&self) -> WinResult<()> {
				hr_to_winresult((self.imediacontrol_vt().Stop)(self.ppvt))
			}

			/// [`IMediaControl::StopWhenReady`](https://docs.microsoft.com/en-us/windows/win32/api/control/nf-control-imediacontrol-stopwhenready)
			/// method.
			pub fn StopWhenReady(&self) -> WinResult<bool> {
				hr_to_winresult_bool(
					(self.imediacontrol_vt().StopWhenReady)(self.ppvt),
				)
			}
		}
	};
}

impl_IUnknown!(IMediaControl, IMediaControlVT);
impl_IDispatch!(IMediaControl, IMediaControlVT);
impl_IMediaControl!(IMediaControl, IMediaControlVT);
