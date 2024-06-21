#![allow(non_camel_case_types, non_snake_case)]

use crate::co;
use crate::decl::*;
use crate::guard::*;
use crate::kernel::ffi_types::*;
use crate::mf::vts::*;
use crate::ole::privs::*;
use crate::prelude::*;

com_interface! { IMFAttributes: "2cd2d921-c447-44a7-a13c-4adabfc247e3";
	/// [`IMFAttributes`](https://learn.microsoft.com/en-us/windows/win32/api/mfobjects/nn-mfobjects-imfattributes)
	/// COM interface.
	///
	/// Automatically calls
	/// [`Release`](https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
}

impl mf_IMFAttributes for IMFAttributes {}

/// This trait is enabled with the `mf` feature, and provides methods for
/// [`IMFAttributes`](crate::IMFAttributes).
///
/// Prefer importing this trait through the prelude:
///
/// ```no_run
/// use winsafe::prelude::*;
/// ```
pub trait mf_IMFAttributes: ole_IUnknown {
	/// [`IMFAttributes::Compare`](https://learn.microsoft.com/en-us/windows/win32/api/mfobjects/nf-mfobjects-imfattributes-compare)
	/// method.
	#[must_use]
	fn Compare(&self,
		theirs: &impl mf_IMFAttributes,
		match_type: co::MF_ATTRIBUTES_MATCH,
	) -> HrResult<bool>
	{
		let mut res: BOOL = 0;
		ok_to_hrresult(
			unsafe {
				(vt::<IMFAttributesVT>(self).Compare)(
					self.ptr(),
					theirs.ptr(),
					match_type.raw(),
					&mut res,
				)
			},
		).map(|_| res != 0)
	}

	/// [`IMFAttributes::CompareItem`](https://learn.microsoft.com/en-us/windows/win32/api/mfobjects/nf-mfobjects-imfattributes-compareitem)
	/// method.
	#[must_use]
	fn CompareItem(&self,
		guid_key: &GUID,
		value: &PROPVARIANT,
	) -> HrResult<bool>
	{
		let mut res: BOOL = 0;
		ok_to_hrresult(
			unsafe {
				(vt::<IMFAttributesVT>(self).CompareItem)(
					self.ptr(),
					guid_key as *const _ as _,
					value as *const _ as _,
					&mut res,
				)
			},
		).map(|_| res != 0)
	}

	/// [`IMFAttributes::CopyAllItems`](https://learn.microsoft.com/en-us/windows/win32/api/mfobjects/nf-mfobjects-imfattributes-copyallitems)
	/// method.
	fn CopyAllItems(&self, dest: &impl mf_IMFAttributes) -> HrResult<()> {
		ok_to_hrresult(
			unsafe {
				(vt::<IMFAttributesVT>(self).CopyAllItems)(self.ptr(), dest.ptr())
			},
		)
	}

	fn_com_noparm! { DeleteAllItems: IMFAttributesVT;
		/// [`IMFAttributes::DeleteAllItems`](https://learn.microsoft.com/en-us/windows/win32/api/mfobjects/nf-mfobjects-imfattributes-deleteallitems)
		/// method.
	}

	/// [`IMFAttributes::DeleteItem`](https://learn.microsoft.com/en-us/windows/win32/api/mfobjects/nf-mfobjects-imfattributes-deleteitem)
	/// method.
	fn DeleteItem(&self, guid_key: &GUID) -> HrResult<()> {
		ok_to_hrresult(
			unsafe {
				(vt::<IMFAttributesVT>(self).DeleteItem)(
					self.ptr(),
					guid_key as *const _ as _,
				)
			},
		)
	}

	/// [`IMFAttributes::GetAllocatedBlob`](https://learn.microsoft.com/en-us/windows/win32/api/mfobjects/nf-mfobjects-imfattributes-getallocatedblob)
	/// method.
	#[must_use]
	fn GetAllocatedBlob(&self, guid_key: &GUID) -> HrResult<Vec<u8>> {
		let mut pbuf = std::ptr::null_mut::<u8>();
		let mut sz = u32::default();

		ok_to_hrresult(
			unsafe {
				(vt::<IMFAttributesVT>(self).GetAllocatedBlob)(
					self.ptr(),
					guid_key as *const _ as _,
					&mut pbuf,
					&mut sz,
				)
			},
		).map(|_| {
			let raw = unsafe { CoTaskMemFreeGuard::new(pbuf as *mut _, sz as _) };
			raw.as_slice().to_vec()
		})
	}

	/// [`IMFAttributes::GetAllocatedString`](https://learn.microsoft.com/en-us/windows/win32/api/mfobjects/nf-mfobjects-imfattributes-getallocatedstring)
	/// method.
	#[must_use]
	fn GetAllocatedString(&self, guid_key: &GUID) -> HrResult<String> {
		let mut pbuf = std::ptr::null_mut::<u16>();
		let mut nchars = u32::default();

		ok_to_hrresult(
			unsafe {
				(vt::<IMFAttributesVT>(self).GetAllocatedString)(
					self.ptr(),
					guid_key as *const _ as _,
					&mut pbuf,
					&mut nchars,
				)
			},
		).map(|_| {
			let str = unsafe { WString::from_wchars_nullt(pbuf) };
			let _ = unsafe { CoTaskMemFreeGuard::new(pbuf as _, 0) };
			str.to_string()
		})
	}

	/// [`IMFAttributes::GetBlob`](https://learn.microsoft.com/en-us/windows/win32/api/mfobjects/nf-mfobjects-imfattributes-getblob)
	/// method.
	///
	/// Calls
	/// [`IMFAttributes::GetBlobSize`](crate::prelude::mf_IMFAttributes::GetBlobSize)
	/// to alloc the buffer.
	#[must_use]
	fn GetBlob(&self, guid_key: &GUID) -> HrResult<Vec<u8>> {
		let sz = self.GetBlobSize(guid_key)?;
		let mut buf = vec![0u8; sz as _];
		ok_to_hrresult(
			unsafe {
				(vt::<IMFAttributesVT>(self).GetBlob)(
					self.ptr(),
					guid_key as *const _ as _,
					buf.as_mut_ptr(),
					sz,
					std::ptr::null_mut(),
				)
			},
		).map(|_| buf)
	}

	/// [`IMFAttributes::GetBlobSize`](https://learn.microsoft.com/en-us/windows/win32/api/mfobjects/nf-mfobjects-imfattributes-getblobsize)
	/// method.
	#[must_use]
	fn GetBlobSize(&self, guid_key: &GUID) -> HrResult<u32> {
		let mut sz = u32::default();
		ok_to_hrresult(
			unsafe {
				(vt::<IMFAttributesVT>(self).GetBlobSize)(
					self.ptr(),
					guid_key as *const _ as _,
					&mut sz,
				)
			},
		).map(|_| sz)
	}

	/// [`IMFAttributes::GetCount`](https://learn.microsoft.com/en-us/windows/win32/api/mfobjects/nf-mfobjects-imfattributes-getcount)
	/// method.
	#[must_use]
	fn GetCount(&self) -> HrResult<u32> {
		let mut count = u32::default();
		ok_to_hrresult(
			unsafe {
				(vt::<IMFAttributesVT>(self).GetCount)(self.ptr(), &mut count)
			},
		).map(|_| count)
	}

	/// [`IMFAttributes::GetUINT32`](https://learn.microsoft.com/en-us/windows/win32/api/mfobjects/nf-mfobjects-imfattributes-getuint32)
	/// method.
	fn GetUINT32(&self, guid_key: &GUID) -> HrResult<u32> {
		let mut value = u32::default();
		ok_to_hrresult(
			unsafe {
				(vt::<IMFAttributesVT>(self).GetUINT32)(
					self.ptr(),
					guid_key as *const _ as _,
					&mut value,
				)
			},
		).map(|_| value)
	}

	/// [`IMFAttributes::GetUINT64`](https://learn.microsoft.com/en-us/windows/win32/api/mfobjects/nf-mfobjects-imfattributes-getuint64)
	/// method.
	fn GetUINT64(&self, guid_key: &GUID) -> HrResult<u64> {
		let mut value = u64::default();
		ok_to_hrresult(
			unsafe {
				(vt::<IMFAttributesVT>(self).GetUINT64)(
					self.ptr(),
					guid_key as *const _ as _,
					&mut value,
				)
			},
		).map(|_| value)
	}

	/// [`IMFAttributes::SetUINT32`](https://learn.microsoft.com/en-us/windows/win32/api/mfobjects/nf-mfobjects-imfattributes-setuint32)
	/// method.
	fn SetUINT32(&self, guid_key: &GUID, value: u32) -> HrResult<()> {
		ok_to_hrresult(
			unsafe {
				(vt::<IMFAttributesVT>(self).SetUINT32)(
					self.ptr(),
					guid_key as *const _ as _,
					value,
				)
			},
		)
	}

	/// [`IMFAttributes::SetUINT64`](https://learn.microsoft.com/en-us/windows/win32/api/mfobjects/nf-mfobjects-imfattributes-setuint64)
	/// method.
	fn SetUINT64(&self, guid_key: &GUID, value: u64) -> HrResult<()> {
		ok_to_hrresult(
			unsafe {
				(vt::<IMFAttributesVT>(self).SetUINT64)(
					self.ptr(),
					guid_key as *const _ as _,
					value,
				)
			},
		)
	}
}
