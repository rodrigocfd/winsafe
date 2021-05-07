#![allow(non_snake_case)]

// use crate::com::{IUnknownVT, PPComVT};
// use crate::com::dshow::vt::IPinVT;
// use crate::com::funcs::CoTaskMemFree;
// use crate::WString;

macro_rules! IPin_impl {
	(
		$(#[$doc:meta])*
		$name:ident, $vt:ty
	) => {
		use crate::com::dshow::vt::IPinVT;
		use crate::com::funcs::CoTaskMemFree;
		use crate::WString;

		IUnknown_impl! {
			$(#[$doc])*
			$name, $vt
		}

		impl $name {
			ppvt_conv!(ipin_vt, IPinVT);

			/// [`IPin::BeginFlush`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-ipin-beginflush)
			/// method.
			pub fn BeginFlush(&self) -> WinResult<()> {
				hr_to_winresult((self.ipin_vt().BeginFlush)(self.ppvt))
			}

			/// [`IPin::ConnectedTo`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-ipin-connectedto)
			/// method.
			pub fn ConnectedTo(&self) -> WinResult<IPin> {
				let mut ppvQueried: PPComVT<IPinVT> = std::ptr::null_mut();
				hr_to_winresult(
					(self.ipin_vt().ConnectedTo)(
						self.ppvt,
						&mut ppvQueried as *mut _ as _,
					),
				).map(|_| IPin::from(ppvQueried))
			}

			/// [`IPin::Disconnect`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-ipin-disconnect)
			/// method.
			pub fn Disconnect(&self) -> WinResult<()> {
				hr_to_winresult((self.ipin_vt().Disconnect)(self.ppvt))
			}

			/// [`IPin::EndFlush`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-ipin-endflush)
			/// method.
			pub fn EndFlush(&self) -> WinResult<()> {
				hr_to_winresult((self.ipin_vt().EndFlush)(self.ppvt))
			}

			/// [`IPin::EndOfStream`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-ipin-endofstream)
			/// method.
			pub fn EndOfStream(&self) -> WinResult<()> {
				hr_to_winresult((self.ipin_vt().EndOfStream)(self.ppvt))
			}

			/// [`IPin::QueryId`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-ipin-queryid)
			/// method.
			pub fn QueryId(&self) -> WinResult<String> {
				let mut pstr: *mut u16 = std::ptr::null_mut();
				hr_to_winresult((self.ipin_vt().QueryId)(self.ppvt, &mut pstr))
					.map(|_| {
						let name = WString::from_wchars_nullt(pstr);
						CoTaskMemFree(pstr);
						name.to_string()
					})
			}
		}
	};
}

IPin_impl! {
	/// [`IPin`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nn-strmif-ipin)
	/// COM interface over [`IPinVT`](crate::dshow::vt::IPinVT). Inherits from
	/// [`IUnknown`](crate::IUnknown).
	///
	/// Automatically calls
	/// [`IUnknown::Release`](https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
	IPin, crate::com::dshow::vt::IPinVT
}
