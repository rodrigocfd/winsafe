#![allow(non_snake_case)]

use crate::aliases::WinResult;
use crate::com::funcs::CoTaskMemFree;
use crate::com::iunknown::{ComPtr, IUnknownT, IUnknownVT};
use crate::ffi::{HRESULT, PCVOID, PSTR, PVOID};
use crate::privs::hr_to_winresult;
use crate::various::WString;

/// [`IPin`](crate::dshow::IPin) virtual table.
#[repr(C)]
pub struct IPinVT {
	pub IUnknownVT: IUnknownVT,
	pub Connect: fn(ComPtr, ComPtr, ComPtr, PCVOID) -> HRESULT,
	pub ReceiveConnection: fn(ComPtr, ComPtr, PCVOID) -> HRESULT,
	pub Disconnect: fn(ComPtr) -> HRESULT,
	pub ConnectedTo: fn(ComPtr, *mut ComPtr) -> HRESULT,
	pub ConnectionMediaType: fn(ComPtr, PVOID) -> HRESULT,
	pub QueryPinInfo: fn(ComPtr, PVOID) -> HRESULT,
	pub QueryDirection: fn(ComPtr, PVOID) -> HRESULT,
	pub QueryId: fn(ComPtr, *mut PSTR) -> HRESULT,
	pub QueryAccept: fn(ComPtr, PCVOID) -> HRESULT,
	pub EnumMediaTypes: fn(ComPtr, *mut ComPtr) -> HRESULT,
	pub QueryInternalConnections: fn(ComPtr, *mut ComPtr, *mut u32) -> HRESULT,
	pub EndOfStream: fn(ComPtr) -> HRESULT,
	pub BeginFlush: fn(ComPtr) -> HRESULT,
	pub EndFlush: fn(ComPtr) -> HRESULT,
	pub NewSegment: fn(ComPtr, i64, i64, f64) -> HRESULT,
}

/// [`IPin`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nn-strmif-ipin)
/// COM interface over [`IPinVT`](crate::dshow::vt::IPinVT).
///
/// Automatically calls
/// [`IUnknown::Release`](https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
/// when the object goes out of scope.
pub struct IPin(ComPtr);

impl_iunknown!(IPin, 0x56a86891, 0x0ad4, 0x11ce, 0xb03a, 0x0020af0ba770);
impl IPinT for IPin {}

/// Exposes the [`IPin`](crate::dshow::IPin) methods.
pub trait IPinT: IUnknownT {
	/// [`IPin::BeginFlush`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-ipin-beginflush)
	/// method.
	fn BeginFlush(&self) -> WinResult<()> {
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IPinVT);
			hr_to_winresult((vt.BeginFlush)(self.ptr()))
		}
	}

	/// [`IPin::ConnectedTo`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-ipin-connectedto)
	/// method.
	fn ConnectedTo(&self) -> WinResult<IPin> {
		let mut ppv_queried = ComPtr::null();
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IPinVT);
			hr_to_winresult(
				(vt.ConnectedTo)(self.ptr(), &mut ppv_queried),
			)
		}.map(|_| IPin::from(ppv_queried))
	}

	/// [`IPin::Disconnect`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-ipin-disconnect)
	/// method.
	fn Disconnect(&self) -> WinResult<()> {
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IPinVT);
			hr_to_winresult((vt.Disconnect)(self.ptr()))
		}
	}

	/// [`IPin::EndFlush`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-ipin-endflush)
	/// method.
	fn EndFlush(&self) -> WinResult<()> {
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IPinVT);
			hr_to_winresult((vt.EndFlush)(self.ptr()))
		}
	}

	/// [`IPin::EndOfStream`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-ipin-endofstream)
	/// method.
	fn EndOfStream(&self) -> WinResult<()> {
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IPinVT);
			hr_to_winresult((vt.EndOfStream)(self.ptr()))
		}
	}

	/// [`IPin::QueryId`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-ipin-queryid)
	/// method.
	fn QueryId(&self) -> WinResult<String> {
		let mut pstr: *mut u16 = std::ptr::null_mut();
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IPinVT);
			hr_to_winresult((vt.QueryId)(self.ptr(), &mut pstr))
		}.map(|_| {
			let name = WString::from_wchars_nullt(pstr);
			CoTaskMemFree(pstr);
			name.to_string()
		})
	}
}
