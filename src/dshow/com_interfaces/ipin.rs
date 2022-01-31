#![allow(non_snake_case)]

use crate::ffi_types::{HRES, PCVOID, PSTR, PVOID};
use crate::kernel::decl::WString;
use crate::ole::decl::{ComPtr, CoTaskMemFree, HrResult};
use crate::ole::privs::ok_to_hrresult;
use crate::prelude::OleIUnknown;
use crate::vt::IUnknownVT;

/// [`IPin`](crate::IPin) virtual table.
#[cfg_attr(docsrs, doc(cfg(feature = "dshow")))]
#[repr(C)]
pub struct IPinVT {
	pub IUnknownVT: IUnknownVT,
	pub Connect: fn(ComPtr, ComPtr, ComPtr, PCVOID) -> HRES,
	pub ReceiveConnection: fn(ComPtr, ComPtr, PCVOID) -> HRES,
	pub Disconnect: fn(ComPtr) -> HRES,
	pub ConnectedTo: fn(ComPtr, *mut ComPtr) -> HRES,
	pub ConnectionMediaType: fn(ComPtr, PVOID) -> HRES,
	pub QueryPinInfo: fn(ComPtr, PVOID) -> HRES,
	pub QueryDirection: fn(ComPtr, PVOID) -> HRES,
	pub QueryId: fn(ComPtr, *mut PSTR) -> HRES,
	pub QueryAccept: fn(ComPtr, PCVOID) -> HRES,
	pub EnumMediaTypes: fn(ComPtr, *mut ComPtr) -> HRES,
	pub QueryInternalConnections: fn(ComPtr, *mut ComPtr, *mut u32) -> HRES,
	pub EndOfStream: fn(ComPtr) -> HRES,
	pub BeginFlush: fn(ComPtr) -> HRES,
	pub EndFlush: fn(ComPtr) -> HRES,
	pub NewSegment: fn(ComPtr, i64, i64, f64) -> HRES,
}

/// [`IPin`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nn-strmif-ipin)
/// COM interface over [`IPinVT`](crate::vt::IPinVT).
///
/// Automatically calls
/// [`IUnknown::Release`](https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
/// when the object goes out of scope.
#[cfg_attr(docsrs, doc(cfg(feature = "dshow")))]
pub struct IPin(ComPtr);

impl_iunknown!(IPin, "56a86891-0ad4-11ce-b03a-0020af0ba770");
impl DshowIPin for IPin {}

/// [`IPin`](crate::IPin) methods methods from `dshow` feature.
#[cfg_attr(docsrs, doc(cfg(feature = "dshow")))]
pub trait DshowIPin: OleIUnknown {
	/// [`IPin::BeginFlush`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-ipin-beginflush)
	/// method.
	fn BeginFlush(&self) -> HrResult<()> {
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IPinVT);
			ok_to_hrresult((vt.BeginFlush)(self.ptr()))
		}
	}

	/// [`IPin::ConnectedTo`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-ipin-connectedto)
	/// method.
	fn ConnectedTo(&self) -> HrResult<IPin> {
		let mut ppv_queried = ComPtr::null();
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IPinVT);
			ok_to_hrresult(
				(vt.ConnectedTo)(self.ptr(), &mut ppv_queried),
			)
		}.map(|_| IPin::from(ppv_queried))
	}

	/// [`IPin::Disconnect`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-ipin-disconnect)
	/// method.
	fn Disconnect(&self) -> HrResult<()> {
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IPinVT);
			ok_to_hrresult((vt.Disconnect)(self.ptr()))
		}
	}

	/// [`IPin::EndFlush`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-ipin-endflush)
	/// method.
	fn EndFlush(&self) -> HrResult<()> {
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IPinVT);
			ok_to_hrresult((vt.EndFlush)(self.ptr()))
		}
	}

	/// [`IPin::EndOfStream`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-ipin-endofstream)
	/// method.
	fn EndOfStream(&self) -> HrResult<()> {
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IPinVT);
			ok_to_hrresult((vt.EndOfStream)(self.ptr()))
		}
	}

	/// [`IPin::QueryId`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-ipin-queryid)
	/// method.
	fn QueryId(&self) -> HrResult<String> {
		let mut pstr: *mut u16 = std::ptr::null_mut();
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IPinVT);
			ok_to_hrresult((vt.QueryId)(self.ptr(), &mut pstr))
		}.map(|_| {
			let name = WString::from_wchars_nullt(pstr);
			CoTaskMemFree(pstr);
			name.to_string()
		})
	}
}
