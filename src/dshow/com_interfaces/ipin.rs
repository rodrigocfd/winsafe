#![allow(non_camel_case_types, non_snake_case)]

use crate::co;
use crate::decl::*;
use crate::dshow::vts::*;
use crate::kernel::privs::*;
use crate::ole::privs::*;
use crate::prelude::*;

com_interface! { IPin: "56a86891-0ad4-11ce-b03a-0020af0ba770";
	/// [`IPin`](https://learn.microsoft.com/en-us/windows/win32/api/strmif/nn-strmif-ipin)
	/// COM interface.
	///
	/// Automatically calls
	/// [`IUnknown::Release`](https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
}

impl dshow_IPin for IPin {}

/// This trait is enabled with the `dshow` feature, and provides methods for
/// [`IPin`](crate::IPin).
pub trait dshow_IPin: ole_IUnknown {
	fn_com_noparm! { BeginFlush: IPinVT;
		/// [`IPin::BeginFlush`](https://learn.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-ipin-beginflush)
		/// method.
	}

	/// [`IPin::Connect`](https://learn.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-ipin-connect)
	/// method.
	fn Connect(&self, receive_pin: &impl dshow_IPin, mt: Option<&AM_MEDIA_TYPE>) -> HrResult<()> {
		HrRet(unsafe {
			(vt::<IPinVT>(self).Connect)(self.ptr(), receive_pin.ptr(), pcvoid_or_null(mt))
		})
		.to_hrresult()
	}

	fn_com_interface_get! { ConnectedTo: IPinVT => IPin;
		/// [`IPin::ConnectedTo`](https://learn.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-ipin-connectedto)
		/// method.
	}

	/// [`IPin::ConnectionMediaType`](https://learn.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-ipin-connectionmediatype)
	/// method.
	fn ConnectionMediaType(&self, amt: &mut AM_MEDIA_TYPE) -> HrResult<()> {
		HrRet(unsafe { (vt::<IPinVT>(self).ConnectionMediaType)(self.ptr(), pvoid(amt)) })
			.to_hrresult()
	}

	fn_com_noparm! { Disconnect: IPinVT;
		/// [`IPin::Disconnect`](https://learn.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-ipin-disconnect)
		/// method.
	}

	fn_com_noparm! { EndFlush: IPinVT;
		/// [`IPin::EndFlush`](https://learn.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-ipin-endflush)
		/// method.
	}

	fn_com_noparm! { EndOfStream: IPinVT;
		/// [`IPin::EndOfStream`](https://learn.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-ipin-endofstream)
		/// method.
	}

	fn_com_interface_get! { EnumMediaTypes: IPinVT => IEnumMediaTypes;
		/// [`IPin::EnumMediaTypes`](https://learn.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-ipin-enummediatypes)
		/// method.
	}

	/// [`IPin::NewSegment`](https://learn.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-ipin-newsegment)
	/// method.
	fn NewSegment(&self, start: i64, stop: i64, rate: f64) -> HrResult<()> {
		HrRet(unsafe { (vt::<IPinVT>(self).NewSegment)(self.ptr(), start, stop, rate) })
			.to_hrresult()
	}

	/// [`IPin::QueryAccept`](https://learn.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-ipin-queryaccept)
	/// method.
	#[must_use]
	fn QueryAccept(&self, amt: &AM_MEDIA_TYPE) -> HrResult<bool> {
		HrRet(unsafe { (vt::<IPinVT>(self).QueryAccept)(self.ptr(), pcvoid(amt)) })
			.to_bool_hrresult()
	}

	/// [`IPin::QueryDirection`](https://learn.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-ipin-querydirection)
	/// method.
	#[must_use]
	fn QueryDirection(&self) -> HrResult<co::PIN_DIRECTION> {
		let mut pin_dir = co::PIN_DIRECTION::INPUT;
		HrRet(unsafe { (vt::<IPinVT>(self).QueryDirection)(self.ptr(), pvoid(&mut pin_dir)) })
			.to_hrresult()
			.map(|_| pin_dir)
	}

	/// [`IPin::QueryId`](https://learn.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-ipin-queryid)
	/// method.
	#[must_use]
	fn QueryId(&self) -> HrResult<String> {
		let mut pstr = std::ptr::null_mut::<u16>();
		HrRet(unsafe { (vt::<IPinVT>(self).QueryId)(self.ptr(), &mut pstr) })
			.to_hrresult()
			.map(|_| htaskmem_ptr_to_str(pstr))
	}

	/// [`IPin::QueryInternalConnections`](https://learn.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-ipin-queryinternalconnections)
	/// method.
	#[must_use]
	fn QueryInternalConnections(&self) -> HrResult<Vec<IPin>> {
		let mut count = 0u32;
		HrRet(unsafe {
			(vt::<IPinVT>(self).QueryInternalConnections)(
				self.ptr(),
				std::ptr::null_mut(),
				&mut count,
			)
		})
		.to_hrresult()?;

		let mut queried = vec![unsafe { IPin::null() }; count as _];
		HrRet(unsafe {
			(vt::<IPinVT>(self).QueryInternalConnections)(
				self.ptr(),
				queried.as_mut_ptr() as _,
				&mut count,
			)
		})
		.to_hrresult()
		.map(|_| queried)
	}

	/// [`IPin::QueryPinInfo`](https://learn.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-ipin-querypininfo)
	/// method.
	fn QueryPinInfo(&self, info: &mut PIN_INFO) -> HrResult<()> {
		HrRet(unsafe { (vt::<IPinVT>(self).QueryPinInfo)(self.ptr(), pvoid(info)) }).to_hrresult()
	}

	/// [`IPin::ReceiveConnection`](https://learn.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-ipin-receiveconnection)
	/// method.
	fn ReceiveConnection(&self, connector: &impl dshow_IPin, mt: &AM_MEDIA_TYPE) -> HrResult<()> {
		HrRet(unsafe {
			(vt::<IPinVT>(self).ReceiveConnection)(self.ptr(), connector.ptr(), pcvoid(mt))
		})
		.to_hrresult()
	}
}
