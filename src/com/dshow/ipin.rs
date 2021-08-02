#![allow(non_snake_case)]

use crate::com::iunknown::IUnknownVT;
use crate::com::traits::{ComInterface, PPVT};
use crate::ffi::{HRESULT, PCVOID, PSTR, PVOID};
use crate::structs::IID;

/// [`IPin`](crate::dshow::IPin) virtual table.
pub struct IPinVT {
	pub IUnknownVT: IUnknownVT,
	pub Connect: fn(PPVT, PPVT, PPVT, PCVOID) -> HRESULT,
	pub ReceiveConnection: fn(PPVT, PPVT, PCVOID) -> HRESULT,
	pub Disconnect: fn(PPVT) -> HRESULT,
	pub ConnectedTo: fn(PPVT, *mut PPVT) -> HRESULT,
	pub ConnectionMediaType: fn(PPVT, PVOID) -> HRESULT,
	pub QueryPinInfo: fn(PPVT, PVOID) -> HRESULT,
	pub QueryDirection: fn(PPVT, PVOID) -> HRESULT,
	pub QueryId: fn(PPVT, *mut PSTR) -> HRESULT,
	pub QueryAccept: fn(PPVT, PCVOID) -> HRESULT,
	pub EnumMediaTypes: fn(PPVT, *mut PPVT) -> HRESULT,
	pub QueryInternalConnections: fn(PPVT, *mut PPVT, *mut u32) -> HRESULT,
	pub EndOfStream: fn(PPVT) -> HRESULT,
	pub BeginFlush: fn(PPVT) -> HRESULT,
	pub EndFlush: fn(PPVT) -> HRESULT,
	pub NewSegment: fn(PPVT, i64, i64, f64) -> HRESULT,
}

/// [`IPin`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nn-strmif-ipin)
/// COM interface over [`IPinVT`](crate::dshow::vt::IPinVT). Inherits from
/// [`IUnknown`](crate::IUnknown).
///
/// Automatically calls
/// [`IUnknown::Release`](https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
/// when the object goes out of scope.
pub struct IPin {
	pub(crate) ppvt: PPVT,
}

impl ComInterface for IPin {
	const IID: IID = IID::new(0x56a86891, 0x0ad4, 0x11ce, 0xb03a, 0x0020af0ba770);
}

macro_rules! impl_IPin {
	($name:ty, $vt:ty) => {
		use crate::com::funcs::CoTaskMemFree;
		use crate::various::WString;

		impl $name {
			fn ipin_vt(&self) -> &IPinVT {
				unsafe { &**(self.ppvt as *mut *mut _) }
			}

			/// [`IPin::BeginFlush`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-ipin-beginflush)
			/// method.
			pub fn BeginFlush(&self) -> WinResult<()> {
				hr_to_winresult((self.ipin_vt().BeginFlush)(self.ppvt))
			}

			/// [`IPin::ConnectedTo`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-ipin-connectedto)
			/// method.
			pub fn ConnectedTo(&self) -> WinResult<IPin> {
				let mut ppvQueried: PPVT = std::ptr::null_mut();
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
