#![allow(non_camel_case_types, non_snake_case)]

use crate::decl::*;
use crate::mf::vts::*;
use crate::ole::privs::*;
use crate::prelude::*;

com_interface! { IMFCollection: "5bc8a76b-869a-46a3-9b03-fa218a66aebe";
	/// [`IMFCollection`](https://learn.microsoft.com/en-us/windows/win32/api/mfobjects/nn-mfobjects-imfcollection)
	/// COM interface.
	///
	/// Automatically calls
	/// [`Release`](https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
}

impl mf_IMFCollection for IMFCollection {}

/// This trait is enabled with the `mf` feature, and provides methods for
/// [`IMFCollection`](crate::IMFCollection).
///
/// Prefer importing this trait through the prelude:
///
/// ```no_run
/// use winsafe::prelude::*;
/// ```
pub trait mf_IMFCollection: ole_IUnknown {
	/// [`IMFCollection::AddElement`](https://learn.microsoft.com/en-us/windows/win32/api/mfobjects/nf-mfobjects-imfcollection-addelement)
	/// method.
	fn AddElement(&self, element: &impl ole_IUnknown) -> HrResult<()> {
		ok_to_hrresult(
			unsafe {
				(vt::<IMFCollectionVT>(self).AddElement)(self.ptr(), element.ptr())
			},
		)
	}

	/// [`IMFCollection::GetElement`](https://learn.microsoft.com/en-us/windows/win32/api/mfobjects/nf-mfobjects-imfcollection-getelement)
	/// method.
	#[must_use]
	fn GetElement(&self, index: u32) -> HrResult<Option<IUnknown>> {
		let mut queried = unsafe { IUnknown::null() };
		ok_to_hrresult(
			unsafe {
				(vt::<IMFCollectionVT>(self).GetElement)(
					self.ptr(),
					index,
					queried.as_mut(),
				)
			},
		).map(|_| if queried.ptr().is_null() {
			None
		} else {
			Some(queried)
		})
	}

	/// [`IMFCollection::GetElementCount`](https://learn.microsoft.com/en-us/windows/win32/api/mfobjects/nf-mfobjects-imfcollection-getelementcount)
	/// method.
	#[must_use]
	fn GetElementCount(&self) -> HrResult<u32> {
		let mut count = u32::default();
		ok_to_hrresult(
			unsafe {
				(vt::<IMFCollectionVT>(self).GetElementCount)(
					self.ptr(),
					&mut count,
				)
			},
		).map(|_| count)
	}

	/// [`IMFCollection::InsertElementAt`](https://learn.microsoft.com/en-us/windows/win32/api/mfobjects/nf-mfobjects-imfcollection-insertelementat)
	/// method.
	fn InsertElementAt(&self,
		index: u32,
		element: &impl ole_IUnknown
	) -> HrResult<()>
	{
		ok_to_hrresult(
			unsafe {
				(vt::<IMFCollectionVT>(self).InsertElementAt)(
					self.ptr(),
					index,
					element.ptr(),
				)
			},
		)
	}

	fn_com_noparm! { RemoveAllElements: IMFCollectionVT;
		/// [`IMFCollection::RemoveAllElements`](https://learn.microsoft.com/en-us/windows/win32/api/mfobjects/nf-mfobjects-imfcollection-removeallelements)
		/// method.
	}

	/// [`IMFCollection::RemoveElement`](https://learn.microsoft.com/en-us/windows/win32/api/mfobjects/nf-mfobjects-imfcollection-removeelement)
	/// method.
	fn RemoveElement(&self, index: u32) -> HrResult<Option<IUnknown>> {
		let mut queried = unsafe { IUnknown::null() };
		ok_to_hrresult(
			unsafe {
				(vt::<IMFCollectionVT>(self).RemoveElement)(
					self.ptr(),
					index,
					queried.as_mut(),
				)
			},
		).map(|_| if queried.ptr().is_null() {
			None
		} else {
			Some(queried)
		})
	}
}
