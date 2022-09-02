#![allow(non_camel_case_types, non_snake_case)]

use crate::co;
use crate::dshow::decl::{AM_MEDIA_TYPE, PIN_INFO};
use crate::kernel::decl::WString;
use crate::kernel::ffi_types::{HRES, PCVOID, PSTR, PVOID};
use crate::ole::decl::{ComPtr, CoTaskMemFree, HrResult};
use crate::ole::privs::{ok_to_hrresult, okfalse_to_hrresult};
use crate::prelude::ole_IUnknown;
use crate::vt::IUnknownVT;

/// [`IPin`](crate::IPin) virtual table.
#[cfg_attr(docsrs, doc(cfg(feature = "dshow")))]
#[repr(C)]
pub struct IPinVT {
	pub IUnknownVT: IUnknownVT,
	pub Connect: fn(ComPtr, ComPtr, PCVOID) -> HRES,
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

com_interface! { IPin: "dshow";
	"56a86891-0ad4-11ce-b03a-0020af0ba770";
	/// [`IPin`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nn-strmif-ipin)
	/// COM interface over [`IPinVT`](crate::vt::IPinVT).
	///
	/// Automatically calls
	/// [`IUnknown::Release`](https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
}

impl dshow_IPin for IPin {}

/// This trait is enabled with the `dshow` feature, and provides methods for
/// [`IPin`](crate::IPin).
#[cfg_attr(docsrs, doc(cfg(feature = "dshow")))]
pub trait dshow_IPin: ole_IUnknown {
	/// [`IPin::BeginFlush`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-ipin-beginflush)
	/// method.
	fn BeginFlush(&self) -> HrResult<()> {
		unsafe {
			let vt = self.vt_ref::<IPinVT>();
			ok_to_hrresult((vt.BeginFlush)(self.ptr()))
		}
	}

	/// [`IPin::Connect`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-ipin-connect)
	/// method.
	fn Connect(&self,
		receive_pin: &IPin, mt: Option<&AM_MEDIA_TYPE>) -> HrResult<()>
	{
		unsafe {
			let vt = self.vt_ref::<IPinVT>();
			ok_to_hrresult(
				(vt.Connect)(
					self.ptr(),
					receive_pin.ptr(),
					mt.map_or(std::ptr::null(), |p| p as *const _ as _),
				),
			)
		}
	}

	/// [`IPin::ConnectedTo`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-ipin-connectedto)
	/// method.
	#[must_use]
	fn ConnectedTo(&self) -> HrResult<IPin> {
		unsafe {
			let mut ppv_queried = ComPtr::null();
			let vt = self.vt_ref::<IPinVT>();
			ok_to_hrresult(
				(vt.ConnectedTo)(self.ptr(), &mut ppv_queried),
			).map(|_| IPin::from(ppv_queried))
		}
	}

	/// [`IPin::ConnectionMediaType`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-ipin-connectionmediatype)
	/// method.
	fn ConnectionMediaType(&self, mt: &mut AM_MEDIA_TYPE) -> HrResult<()> {
		unsafe {
			let vt = self.vt_ref::<IPinVT>();
			ok_to_hrresult((vt.ConnectionMediaType)(self.ptr(), mt as *mut _ as _))
		}
	}

	/// [`IPin::Disconnect`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-ipin-disconnect)
	/// method.
	fn Disconnect(&self) -> HrResult<()> {
		unsafe {
			let vt = self.vt_ref::<IPinVT>();
			ok_to_hrresult((vt.Disconnect)(self.ptr()))
		}
	}

	/// [`IPin::EndFlush`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-ipin-endflush)
	/// method.
	fn EndFlush(&self) -> HrResult<()> {
		unsafe {
			let vt = self.vt_ref::<IPinVT>();
			ok_to_hrresult((vt.EndFlush)(self.ptr()))
		}
	}

	/// [`IPin::EndOfStream`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-ipin-endofstream)
	/// method.
	fn EndOfStream(&self) -> HrResult<()> {
		unsafe {
			let vt = self.vt_ref::<IPinVT>();
			ok_to_hrresult((vt.EndOfStream)(self.ptr()))
		}
	}

	/// [`IPin::NewSegment`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-ipin-newsegment)
	/// method.
	fn NewSegment(&self, start: i64, stop: i64, rate: f64) -> HrResult<()> {
		unsafe {
			let vt = self.vt_ref::<IPinVT>();
			ok_to_hrresult((vt.NewSegment)(self.ptr(), start, stop, rate))
		}
	}

	/// [`IPin::QueryAccept`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-ipin-queryaccept)
	/// method.
	#[must_use]
	fn QueryAccept(&self, mt: &AM_MEDIA_TYPE) -> HrResult<bool> {
		unsafe {
			let vt = self.vt_ref::<IPinVT>();
			okfalse_to_hrresult((vt.QueryAccept)(self.ptr(), mt as *const _ as _))
		}
	}

	/// [`IPin::QueryDirection`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-ipin-querydirection)
	/// method.
	#[must_use]
	fn QueryDirection(&self) -> HrResult<co::PIN_DIRECTION> {
		let mut pin_dir = co::PIN_DIRECTION::INPUT;
		unsafe {
			let vt = self.vt_ref::<IPinVT>();
			ok_to_hrresult(
				(vt.QueryDirection)(self.ptr(), &mut pin_dir as *mut _ as _),
			).map(|_| pin_dir)
		}
	}

	/// [`IPin::QueryId`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-ipin-queryid)
	/// method.
	#[must_use]
	fn QueryId(&self) -> HrResult<String> {
		let mut pstr: *mut u16 = std::ptr::null_mut();
		unsafe {
			let vt = self.vt_ref::<IPinVT>();
			ok_to_hrresult((vt.QueryId)(self.ptr(), &mut pstr))
		}.map(|_| {
			let name = WString::from_wchars_nullt(pstr);
			CoTaskMemFree(pstr as _);
			name.to_string()
		})
	}

	/// [`IPin::QueryInternalConnections`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-ipin-queryinternalconnections)
	/// method.
	#[must_use]
	fn QueryInternalConnections(&self) -> HrResult<Vec<IPin>> {
		let mut count = u32::default();
		unsafe {
			let vt = self.vt_ref::<IPinVT>();
			if let Err(e) = ok_to_hrresult(
				(vt.QueryInternalConnections)(
					self.ptr(),
					std::ptr::null_mut(),
					&mut count as *mut _ as _,
				),
			) {
				return Err(e);
			}

			let mut ppv_queried = vec![ComPtr::null(); count as _];
			ok_to_hrresult(
				(vt.QueryInternalConnections)(
					self.ptr(),
					ppv_queried.as_mut_ptr(),
					&mut count as *mut _ as _,
				),
			).map(|_| {
				ppv_queried.into_iter()
					.map(|ppv| IPin::from(ppv))
					.collect::<Vec<_>>()
			})
		}
	}

	/// [`IPin::QueryPinInfo`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-ipin-querypininfo)
	/// method.
	fn QueryPinInfo(&self, info: &mut PIN_INFO) -> HrResult<()> {
		unsafe {
			let vt = self.vt_ref::<IPinVT>();
			ok_to_hrresult((vt.QueryPinInfo)(self.ptr(), info as *mut _ as _))
		}
	}

	/// [`IPin::ReceiveConnection`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-ipin-receiveconnection)
	/// method.
	fn ReceiveConnection(&self,
		connector: &IPin, mt: &AM_MEDIA_TYPE) -> HrResult<()>
	{
		unsafe {
			let vt = self.vt_ref::<IPinVT>();
			ok_to_hrresult(
				(vt.ReceiveConnection)(
					self.ptr(),
					connector.ptr(),
					mt as *const _ as _,
				),
			)
		}
	}
}
