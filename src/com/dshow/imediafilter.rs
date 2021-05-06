#![allow(non_snake_case)]

use crate::aliases::WinResult;
use crate::com::{IPersistVT, IUnknownVT};
use crate::com::dshow::vt::IMediaFilterVT;
use crate::com::PPComVT;
use crate::privs::{hr_to_winresult, hr_to_winresult_bool, ref_as_pvoid};
use crate::structs::CLSID;

macro_rules! IMediaFilter_impl {
	(
		$(#[$doc:meta])*
		$name:ident, $vt:ident
	) => {
		IPersist_impl! {
			$(#[$doc])*
			$name, $vt
		}

		impl $name {
			/// [`IMediaFilter::Pause`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-imediafilter-pause)
			/// method.
			pub fn Pause(&self) -> WinResult<bool> {
				let ppvt = unsafe { self.ppvt::<IMediaFilterVT>() };
				hr_to_winresult_bool(unsafe { ((**ppvt).Pause)(ppvt) })
			}

			/// [`IMediaFilter::Run`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-imediafilter-run)
			/// method.
			pub fn Run(&self, tStart: i64) -> WinResult<bool> {
				let ppvt = unsafe { self.ppvt::<IMediaFilterVT>() };
				hr_to_winresult_bool(unsafe { ((**ppvt).Run)(ppvt, tStart) })
			}

			/// [`IMediaFilter::Stop`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-imediafilter-stop)
			/// method.
			pub fn Stop(&self) -> WinResult<bool> {
				let ppvt = unsafe { self.ppvt::<IMediaFilterVT>() };
				hr_to_winresult_bool(unsafe { ((**ppvt).Stop)(ppvt) })
			}
		}
	};
}

IMediaFilter_impl! {
	/// [`IMediaFilter`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nn-strmif-imediafilter)
	/// COM interface.
	///
	/// Virtual table: [`IMediaFilterVT`](crate::dshow::vt::IMediaFilterVT).
	///
	/// Inherits from:
	/// * [`IPersist`](crate::IPersist);
	/// * [`IUnknown`](crate::IUnknown).
	///
	/// Automatically calls
	/// [`IUnknown::Release`](https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
	IMediaFilter, IMediaFilterVT
}
