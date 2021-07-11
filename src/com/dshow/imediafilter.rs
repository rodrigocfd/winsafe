#![allow(non_snake_case)]

use crate::com::ipersist::IPersistVT;
use crate::com::traits::{ComInterface, PPI};
use crate::ffi::{HRESULT, PVOID};
use crate::structs::IID;

/// [`IMediaFilter`](crate::dshow::IMediaFilter) virtual table.
pub struct IMediaFilterVT {
	pub IPersistVT: IPersistVT,
	pub Stop: fn(PPI) -> HRESULT,
	pub Pause: fn(PPI) -> HRESULT,
   pub Run: fn(PPI, i64) -> HRESULT,
	pub GetState: fn(PPI, i64, PVOID, *mut u32) -> HRESULT,
	pub SetSyncSource: fn(PPI, PPI) -> HRESULT,
	pub GetSyncSource: fn(PPI, *mut PPI) -> HRESULT,
}

/// [`IMediaFilter`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nn-strmif-imediafilter)
/// COM interface over [`IMediaFilterVT`](crate::dshow::vt::IMediaFilterVT).
/// Inherits from [`IPersist`](crate::IPersist),
/// [`IUnknown`](crate::IUnknown).
///
/// Automatically calls
/// [`IUnknown::Release`](https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
/// when the object goes out of scope.
pub struct IMediaFilter {
	pub(crate) ppvt: PPI,
}

impl_send_sync_fromppvt!(IMediaFilter);

impl ComInterface for IMediaFilter {
	const IID: IID = IID::new(0x56a86899, 0x0ad4, 0x11ce, 0xb03a, 0x0020af0ba770);
}

macro_rules! impl_IMediaFilter {
	($name:ty, $vt:ty) => {
		use crate::privs::hr_to_winresult_bool;

		impl $name {
			fn imediafilter_vt(&self) -> &IMediaFilterVT {
				unsafe { &**(self.ppvt as *mut *mut _) }
			}

			/// [`IMediaFilter::Pause`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-imediafilter-pause)
			/// method.
			pub fn Pause(&self) -> WinResult<bool> {
				hr_to_winresult_bool((self.imediafilter_vt().Pause)(self.ppvt))
			}

			/// [`IMediaFilter::Run`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-imediafilter-run)
			/// method.
			pub fn Run(&self, tStart: i64) -> WinResult<bool> {
				hr_to_winresult_bool(
					(self.imediafilter_vt().Run)(self.ppvt, tStart),
				)
			}

			/// [`IMediaFilter::Stop`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-imediafilter-stop)
			/// method.
			pub fn Stop(&self) -> WinResult<bool> {
				hr_to_winresult_bool((self.imediafilter_vt().Stop)(self.ppvt))
			}
		}
	};
}

impl_IUnknown!(IMediaFilter, IMediaFilterVT);
impl_IPersist!(IMediaFilter, IMediaFilterVT);
impl_IMediaFilter!(IMediaFilter, IMediaFilterVT);
