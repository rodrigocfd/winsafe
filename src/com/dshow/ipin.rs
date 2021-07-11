#![allow(non_snake_case)]

use crate::com::iunknown::IUnknownVT;
use crate::com::traits::{ComInterface, PPComVT};
use crate::ffi::{HRESULT, PCVOID, PSTR, PVOID};
use crate::structs::IID;

type PP = PPComVT<IUnknownVT>;

/// [`IPin`](crate::dshow::IPin) virtual table.
pub struct IPinVT {
	pub IUnknownVT: IUnknownVT,
	pub Connect: fn(PP, PP, PP, PCVOID) -> HRESULT,
	pub ReceiveConnection: fn(PP, PP, PCVOID) -> HRESULT,
	pub Disconnect: fn(PP) -> HRESULT,
	pub ConnectedTo: fn(PP, *mut PP) -> HRESULT,
	pub ConnectionMediaType: fn(PP, PVOID) -> HRESULT,
	pub QueryPinInfo: fn(PP, PVOID) -> HRESULT,
	pub QueryDirection: fn(PP, PVOID) -> HRESULT,
	pub QueryId: fn(PP, *mut PSTR) -> HRESULT,
	pub QueryAccept: fn(PP, PCVOID) -> HRESULT,
	pub EnumMediaTypes: fn(PP, *mut PP) -> HRESULT,
	pub QueryInternalConnections: fn(PP, *mut PP, *mut u32) -> HRESULT,
	pub EndOfStream: fn(PP) -> HRESULT,
	pub BeginFlush: fn(PP) -> HRESULT,
	pub EndFlush: fn(PP) -> HRESULT,
	pub NewSegment: fn(PP, i64, i64, f64) -> HRESULT,
}

/// [`IPin`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nn-strmif-ipin)
/// COM interface over [`IPinVT`](crate::dshow::vt::IPinVT). Inherits from
/// [`IUnknown`](crate::IUnknown).
///
/// Automatically calls
/// [`IUnknown::Release`](https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
/// when the object goes out of scope.
pub struct IPin {
	pub(crate) ppvt: PPComVT<IUnknownVT>,
}

impl_send_sync_fromppvt!(IPin);

impl ComInterface for IPin {
	const IID: IID = IID::new(0x56a86891, 0x0ad4, 0x11ce, 0xb03a, 0x0020af0ba770);
}

macro_rules! impl_IPin {
	($name:ty, $vt:ty) => {
		use crate::com::funcs::CoTaskMemFree;
		use crate::various::WString;

		impl $name {
			fn ipin_vt(&self) -> &IPinVT {
				unsafe { &**(self.ppvt as PPComVT<_>) }
			}

			/// [`IPin::BeginFlush`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-ipin-beginflush)
			/// method.
			pub fn BeginFlush(&self) -> WinResult<()> {
				hr_to_winresult((self.ipin_vt().BeginFlush)(self.ppvt))
			}

			/// [`IPin::ConnectedTo`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-ipin-connectedto)
			/// method.
			pub fn ConnectedTo(&self) -> WinResult<IPin> {
				let mut ppvQueried: PPComVT<IUnknownVT> = std::ptr::null_mut();
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

impl_IUnknown!(IPin, IPinVT);
impl_IPin!(IPin, IPinVT);
