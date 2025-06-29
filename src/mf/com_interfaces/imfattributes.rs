#![allow(non_camel_case_types, non_snake_case)]

use crate::co;
use crate::decl::*;
use crate::guard::*;
use crate::kernel::privs::*;
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
	fn Compare(
		&self,
		theirs: &impl mf_IMFAttributes,
		match_type: co::MF_ATTRIBUTES_MATCH,
	) -> HrResult<bool> {
		let mut res = 0;
		ok_to_hrresult(unsafe {
			(vt::<IMFAttributesVT>(self).Compare)(
				self.ptr(),
				theirs.ptr(),
				match_type.raw(),
				&mut res,
			)
		})
		.map(|_| res != 0)
	}

	/// [`IMFAttributes::CompareItem`](https://learn.microsoft.com/en-us/windows/win32/api/mfobjects/nf-mfobjects-imfattributes-compareitem)
	/// method.
	#[must_use]
	fn CompareItem(&self, guid_key: &GUID, value: &PropVariant) -> HrResult<bool> {
		let mut res = 0;
		ok_to_hrresult(unsafe {
			(vt::<IMFAttributesVT>(self).CompareItem)(
				self.ptr(),
				pcvoid(guid_key),
				pcvoid(&value.to_raw()?),
				&mut res,
			)
		})
		.map(|_| res != 0)
	}

	/// [`IMFAttributes::CopyAllItems`](https://learn.microsoft.com/en-us/windows/win32/api/mfobjects/nf-mfobjects-imfattributes-copyallitems)
	/// method.
	fn CopyAllItems(&self, dest: &impl mf_IMFAttributes) -> HrResult<()> {
		ok_to_hrresult(unsafe {
			(vt::<IMFAttributesVT>(self).CopyAllItems)(self.ptr(), dest.ptr())
		})
	}

	fn_com_noparm! { DeleteAllItems: IMFAttributesVT;
		/// [`IMFAttributes::DeleteAllItems`](https://learn.microsoft.com/en-us/windows/win32/api/mfobjects/nf-mfobjects-imfattributes-deleteallitems)
		/// method.
	}

	/// [`IMFAttributes::DeleteItem`](https://learn.microsoft.com/en-us/windows/win32/api/mfobjects/nf-mfobjects-imfattributes-deleteitem)
	/// method.
	fn DeleteItem(&self, guid_key: &GUID) -> HrResult<()> {
		ok_to_hrresult(unsafe {
			(vt::<IMFAttributesVT>(self).DeleteItem)(self.ptr(), pcvoid(guid_key))
		})
	}

	/// [`IMFAttributes::GetAllocatedBlob`](https://learn.microsoft.com/en-us/windows/win32/api/mfobjects/nf-mfobjects-imfattributes-getallocatedblob)
	/// method.
	///
	/// Note that this method allocates the buffer twice, whereas
	/// [`IMFAttributes::GetBlob`](crate::prelude::mf_IMFAttributes::GetBlob)
	/// allocates only once, thus being more efficient.
	#[must_use]
	fn GetAllocatedBlob(&self, guid_key: &GUID) -> HrResult<Vec<u8>> {
		let mut pbuf = std::ptr::null_mut::<u8>();
		let mut sz = 0u32;

		ok_to_hrresult(unsafe {
			(vt::<IMFAttributesVT>(self).GetAllocatedBlob)(
				self.ptr(),
				pcvoid(guid_key),
				&mut pbuf,
				&mut sz,
			)
		})
		.map(|_| {
			let raw = unsafe { CoTaskMemFreeGuard::new(pbuf as *mut _, sz as _) };
			raw.as_slice().to_vec()
		})
	}

	/// [`IMFAttributes::GetAllocatedString`](https://learn.microsoft.com/en-us/windows/win32/api/mfobjects/nf-mfobjects-imfattributes-getallocatedstring)
	/// method.
	///
	/// Note that this method allocates the buffer twice, whereas
	/// [`IMFAttributes::GetString`](crate::prelude::mf_IMFAttributes::GetString)
	/// allocates only once, thus being more efficient.
	#[must_use]
	fn GetAllocatedString(&self, guid_key: &GUID) -> HrResult<String> {
		let mut pstr = std::ptr::null_mut::<u16>();
		let mut nchars = 0u32;

		ok_to_hrresult(unsafe {
			(vt::<IMFAttributesVT>(self).GetAllocatedString)(
				self.ptr(),
				pcvoid(guid_key),
				&mut pstr,
				&mut nchars,
			)
		})
		.map(|_| htaskmem_ptr_to_str(pstr))
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

		ok_to_hrresult(unsafe {
			(vt::<IMFAttributesVT>(self).GetBlob)(
				self.ptr(),
				pcvoid(guid_key),
				buf.as_mut_ptr(),
				sz,
				std::ptr::null_mut(),
			)
		})
		.map(|_| buf)
	}

	/// [`IMFAttributes::GetBlobSize`](https://learn.microsoft.com/en-us/windows/win32/api/mfobjects/nf-mfobjects-imfattributes-getblobsize)
	/// method.
	#[must_use]
	fn GetBlobSize(&self, guid_key: &GUID) -> HrResult<u32> {
		let mut sz = 0u32;
		ok_to_hrresult(unsafe {
			(vt::<IMFAttributesVT>(self).GetBlobSize)(self.ptr(), pcvoid(guid_key), &mut sz)
		})
		.map(|_| sz)
	}

	/// [`IMFAttributes::GetCount`](https://learn.microsoft.com/en-us/windows/win32/api/mfobjects/nf-mfobjects-imfattributes-getcount)
	/// method.
	#[must_use]
	fn GetCount(&self) -> HrResult<u32> {
		let mut count = 0u32;
		ok_to_hrresult(unsafe { (vt::<IMFAttributesVT>(self).GetCount)(self.ptr(), &mut count) })
			.map(|_| count)
	}

	/// [`IMFAttributes::GetDouble`](https://learn.microsoft.com/en-us/windows/win32/api/mfobjects/nf-mfobjects-imfattributes-getdouble)
	/// method.
	#[must_use]
	fn GetDouble(&self, guid_key: &GUID) -> HrResult<f64> {
		let mut value = f64::default();
		ok_to_hrresult(unsafe {
			(vt::<IMFAttributesVT>(self).GetDouble)(self.ptr(), pcvoid(guid_key), &mut value)
		})
		.map(|_| value)
	}

	/// [`IMFAttributes::GetGUID`](https://learn.microsoft.com/en-us/windows/win32/api/mfobjects/nf-mfobjects-imfattributes-getguid)
	/// method.
	#[must_use]
	fn GetGUID(&self, guid_key: &GUID) -> HrResult<GUID> {
		let mut value = GUID::default();
		ok_to_hrresult(unsafe {
			(vt::<IMFAttributesVT>(self).GetGUID)(self.ptr(), pcvoid(guid_key), pvoid(&mut value))
		})
		.map(|_| value)
	}

	/// [`IMFAttributes::GetItem`](https://learn.microsoft.com/en-us/windows/win32/api/mfobjects/nf-mfobjects-imfattributes-getitem)
	/// method.
	#[must_use]
	fn GetItem(&self, guid_key: &GUID) -> HrResult<PropVariant> {
		let mut value = PROPVARIANT::default();
		ok_to_hrresult(unsafe {
			(vt::<IMFAttributesVT>(self).GetItem)(self.ptr(), pcvoid(guid_key), pvoid(&mut value))
		})?;
		PropVariant::from_raw(&value)
	}

	/// [`IMFAttributes::GetItemByIndex`](https://learn.microsoft.com/en-us/windows/win32/api/mfobjects/nf-mfobjects-imfattributes-getitembyindex)
	/// method.
	#[must_use]
	fn GetItemByIndex(&self, index: u32) -> HrResult<(GUID, PropVariant)> {
		let mut guid = GUID::default();
		let mut value = PROPVARIANT::default();

		ok_to_hrresult(unsafe {
			(vt::<IMFAttributesVT>(self).GetItemByIndex)(
				self.ptr(),
				index,
				pvoid(&mut guid),
				pvoid(&mut value),
			)
		})?;
		Ok((guid, PropVariant::from_raw(&value)?))
	}

	/// [`IMFAttributes::GetItemType`](https://learn.microsoft.com/en-us/windows/win32/api/mfobjects/nf-mfobjects-imfattributes-getitemtype)
	/// method.
	#[must_use]
	fn GetItemType(&self, guid_key: &GUID) -> HrResult<co::MF_ATTRIBUTE> {
		let mut ty = co::MF_ATTRIBUTE::default();
		ok_to_hrresult(unsafe {
			(vt::<IMFAttributesVT>(self).GetItemType)(self.ptr(), pcvoid(guid_key), ty.as_mut())
		})
		.map(|_| ty)
	}

	/// [`IMFAttributes::GetString`](https://learn.microsoft.com/en-us/windows/win32/api/mfobjects/nf-mfobjects-imfattributes-getstring)
	/// method.
	///
	/// Calls
	/// [`IMFAttributes::GetStringLength`](crate::prelude::mf_IMFAttributes::GetStringLength)
	/// to alloc the buffer.
	#[must_use]
	fn GetString(&self, guid_key: &GUID) -> HrResult<String> {
		let len = self.GetStringLength(guid_key)? + 1;
		let mut buf = WString::new_alloc_buf(len as _);

		ok_to_hrresult(unsafe {
			(vt::<IMFAttributesVT>(self).GetString)(
				self.ptr(),
				pcvoid(guid_key),
				buf.as_mut_ptr(),
				len,
				std::ptr::null_mut(),
			)
		})
		.map(|_| buf.to_string())
	}

	/// [`IMFAttributes::GetStringLength`](https://learn.microsoft.com/en-us/windows/win32/api/mfobjects/nf-mfobjects-imfattributes-getstringlength)
	/// method.
	#[must_use]
	fn GetStringLength(&self, guid_key: &GUID) -> HrResult<u32> {
		let mut len = 0u32;
		ok_to_hrresult(unsafe {
			(vt::<IMFAttributesVT>(self).GetStringLength)(self.ptr(), pcvoid(guid_key), &mut len)
		})
		.map(|_| len)
	}

	/// [`IMFAttributes::GetUINT32`](https://learn.microsoft.com/en-us/windows/win32/api/mfobjects/nf-mfobjects-imfattributes-getuint32)
	/// method.
	fn GetUINT32(&self, guid_key: &GUID) -> HrResult<u32> {
		let mut value = 0u32;
		ok_to_hrresult(unsafe {
			(vt::<IMFAttributesVT>(self).GetUINT32)(self.ptr(), pcvoid(guid_key), &mut value)
		})
		.map(|_| value)
	}

	/// [`IMFAttributes::GetUINT64`](https://learn.microsoft.com/en-us/windows/win32/api/mfobjects/nf-mfobjects-imfattributes-getuint64)
	/// method.
	fn GetUINT64(&self, guid_key: &GUID) -> HrResult<u64> {
		let mut value = 0u64;
		ok_to_hrresult(unsafe {
			(vt::<IMFAttributesVT>(self).GetUINT64)(self.ptr(), pcvoid(guid_key), &mut value)
		})
		.map(|_| value)
	}

	/// [`IMFAttributes::GetUnknown`](https://learn.microsoft.com/en-us/windows/win32/api/mfobjects/nf-mfobjects-imfattributes-getunknown)
	/// method.
	#[must_use]
	fn GetUnknown<T>(&self, guid_key: &GUID) -> HrResult<T>
	where
		T: ole_IUnknown,
	{
		let mut queried = unsafe { T::null() };
		ok_to_hrresult(unsafe {
			(vt::<IMFAttributesVT>(self).GetUnknown)(
				self.ptr(),
				pcvoid(guid_key),
				pcvoid(&T::IID),
				queried.as_mut(),
			)
		})
		.map(|_| queried)
	}

	fn_com_noparm! { LockStore: IMFAttributesVT;
		/// [`IMFAttributes::LockStore`](https://learn.microsoft.com/en-us/windows/win32/api/mfobjects/nf-mfobjects-imfattributes-lockstore)
		/// method.
	}

	/// [`IMFAttributes::SetBlob`](https://learn.microsoft.com/en-us/windows/win32/api/mfobjects/nf-mfobjects-imfattributes-setblob)
	/// method.
	fn SetBlob(&self, guid_key: &GUID, buf: &[u8]) -> HrResult<()> {
		ok_to_hrresult(unsafe {
			(vt::<IMFAttributesVT>(self).SetBlob)(
				self.ptr(),
				pcvoid(guid_key),
				vec_ptr(buf),
				buf.len() as _,
			)
		})
	}

	/// [`IMFAttributes::SetDouble`](https://learn.microsoft.com/en-us/windows/win32/api/mfobjects/nf-mfobjects-imfattributes-setdouble)
	/// method.
	fn SetDouble(&self, guid_key: &GUID, value: f64) -> HrResult<()> {
		ok_to_hrresult(unsafe {
			(vt::<IMFAttributesVT>(self).SetDouble)(self.ptr(), pcvoid(guid_key), value)
		})
	}

	/// [`IMFAttributes::SetGUID`](https://learn.microsoft.com/en-us/windows/win32/api/mfobjects/nf-mfobjects-imfattributes-setguid)
	/// method.
	fn SetGUID(&self, guid_key: &GUID, value: &GUID) -> HrResult<()> {
		ok_to_hrresult(unsafe {
			(vt::<IMFAttributesVT>(self).SetGUID)(self.ptr(), pcvoid(guid_key), pcvoid(value))
		})
	}

	/// [`IMFAttributes::SetItem`](https://learn.microsoft.com/en-us/windows/win32/api/mfobjects/nf-mfobjects-imfattributes-setitem)
	/// method.
	fn SetItem(&self, guid_key: &GUID, value: &PropVariant) -> HrResult<()> {
		ok_to_hrresult(unsafe {
			(vt::<IMFAttributesVT>(self).SetItem)(
				self.ptr(),
				pcvoid(guid_key),
				pcvoid(&value.to_raw()?),
			)
		})
	}

	/// [`IMFAttributes::SetString`](https://learn.microsoft.com/en-us/windows/win32/api/mfobjects/nf-mfobjects-imfattributes-setstring)
	/// method.
	fn SetString(&self, guid_key: &GUID, value: &str) -> HrResult<()> {
		ok_to_hrresult(unsafe {
			(vt::<IMFAttributesVT>(self).SetString)(
				self.ptr(),
				pcvoid(guid_key),
				WString::from_str(value).as_ptr(),
			)
		})
	}

	/// [`IMFAttributes::SetUINT32`](https://learn.microsoft.com/en-us/windows/win32/api/mfobjects/nf-mfobjects-imfattributes-setuint32)
	/// method.
	fn SetUINT32(&self, guid_key: &GUID, value: u32) -> HrResult<()> {
		ok_to_hrresult(unsafe {
			(vt::<IMFAttributesVT>(self).SetUINT32)(self.ptr(), pcvoid(guid_key), value)
		})
	}

	/// [`IMFAttributes::SetUINT64`](https://learn.microsoft.com/en-us/windows/win32/api/mfobjects/nf-mfobjects-imfattributes-setuint64)
	/// method.
	fn SetUINT64(&self, guid_key: &GUID, value: u64) -> HrResult<()> {
		ok_to_hrresult(unsafe {
			(vt::<IMFAttributesVT>(self).SetUINT64)(self.ptr(), pcvoid(guid_key), value)
		})
	}

	/// [`IMFAttributes::SetUnknown`](https://learn.microsoft.com/en-us/windows/win32/api/mfobjects/nf-mfobjects-imfattributes-setunknown)
	/// method.
	fn SetUnknown(&self, guid_key: &GUID, value: &impl ole_IUnknown) -> HrResult<()> {
		ok_to_hrresult(unsafe {
			(vt::<IMFAttributesVT>(self).SetUnknown)(self.ptr(), pcvoid(guid_key), value.ptr())
		})
	}

	fn_com_noparm! { UnlockStore: IMFAttributesVT;
		/// [`IMFAttributes::UnlockStore`](https://learn.microsoft.com/en-us/windows/win32/api/mfobjects/nf-mfobjects-imfattributes-unlockstore)
		/// method.
	}
}
