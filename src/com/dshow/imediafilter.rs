#![allow(non_snake_case)]

macro_rules! pub_struct_IMediaFilter {
	(
		$(#[$doc:meta])*
		$name:ident, $vt:ty
	) => {
		use crate::com::dshow::vt::IMediaFilterVT;
		use crate::privs::hr_to_winresult_bool;

		pub_struct_IPersist! {
			$(#[$doc])*
			$name, $vt
		}

		impl $name {
			fn imediafilter_vt(&self) -> &IMediaFilterVT {
				unsafe { &**(self.ppvt as PPComVT<_>) }
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

pub_struct_IMediaFilter! {
	/// [`IMediaFilter`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nn-strmif-imediafilter)
	/// COM interface over [`IMediaFilterVT`](crate::dshow::vt::IMediaFilterVT).
	/// Inherits from [`IPersist`](crate::IPersist),
	/// [`IUnknown`](crate::IUnknown).
	///
	/// Automatically calls
	/// [`IUnknown::Release`](https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
	IMediaFilter, crate::com::dshow::vt::IMediaFilterVT
}
