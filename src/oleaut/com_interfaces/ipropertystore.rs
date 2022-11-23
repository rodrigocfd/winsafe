#![allow(non_camel_case_types, non_snake_case)]

use std::marker::PhantomData;
use std::mem::ManuallyDrop;

use crate::co;
use crate::kernel::ffi_types::{HRES, PCVOID, PVOID};
use crate::ole::decl::{ComPtr, HrResult};
use crate::ole::privs::ok_to_hrresult;
use crate::oleaut::decl::{PROPERTYKEY, PROPVARIANT};
use crate::prelude::ole_IUnknown;
use crate::vt::IUnknownVT;

/// [`IPropertyStore`](crate::IPropertyStore) virtual table.
#[cfg_attr(docsrs, doc(cfg(feature = "oleaut")))]
#[repr(C)]
pub struct IPropertyStoreVT {
	pub IUnknownVT: IUnknownVT,
	pub GetCount: fn(ComPtr, *mut u32) -> HRES,
	pub GetAt: fn(ComPtr, u32, PVOID) -> HRES,
	pub GetValue: fn(ComPtr, PCVOID, PVOID) -> HRES,
	pub SetValue: fn(ComPtr, PCVOID, PCVOID) -> HRES,
	pub Commit: fn(ComPtr) -> HRES,
}

com_interface! { IPropertyStore: "oleaut";
	"886d8eeb-8cf2-4446-8d02-cdba1dbdcf99";
	/// [`IPropertyStore`](https://learn.microsoft.com/en-us/windows/win32/api/propsys/nn-propsys-ipropertystore)
	/// COM interface over [`IPropertyStoreVT`](crate::vt::IPropertyStoreVT).
	///
	/// Automatically calls
	/// [`IUnknown::Release`](https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
	///
	/// Usually, this interface is taken via
	/// [`IShellItem::BindToHandler`](crate::prelude::shell_IShellItem::BindToHandler).
}

impl oleaut_IPropertyStore for IPropertyStore {}

/// This trait is enabled with the `oleaut` feature, and provides methods for
/// [`IPropertyStore`](crate::IPropertyStore).
///
/// Prefer importing this trait through the prelude:
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// ```
#[cfg_attr(docsrs, doc(cfg(feature = "oleaut")))]
pub trait oleaut_IPropertyStore: ole_IUnknown {
	/// Returns an iterator over the [`PROPERTYKEY`](crate::PROPERTYKEY)
	/// elements by calling
	/// [`IPropertyStore::GetCount`](crate::prelude::oleaut_IPropertyStore::GetCount)
	/// and
	/// [`IPropertyStore::GetAt`](crate::prelude::oleaut_IPropertyStore::GetAt)
	/// consecutively.
	///
	/// # Examples
	///
	/// ```rust,no_run
	/// use winsafe::prelude::*;
	/// use winsafe::IPropertyStore;
	///
	/// let pstore: IPropertyStore; // initialized somewhere
	/// # let pstore = IPropertyStore::from(unsafe { winsafe::ComPtr::null() });
	///
	/// for ppk in pstore.iter()? {
	///     let ppk = ppk?;
	///     // ...
	/// }
	/// # Ok::<_, winsafe::co::HRESULT>(())
	/// ```
	#[must_use]
	fn iter(&self) -> HrResult<Box<dyn Iterator<Item = HrResult<PROPERTYKEY>> + '_>> {
		Ok(Box::new(PropertyStoreIter::new(unsafe { self.ptr() })?))
	}

	/// [`IPropertyStore::Commit`](https://learn.microsoft.com/en-us/windows/win32/api/propsys/nf-propsys-ipropertystore-commit)
	/// method.
	fn Commit(&self) -> HrResult<()> {
		unsafe {
			let vt = self.vt_ref::<IPropertyStoreVT>();
			ok_to_hrresult((vt.Commit)(self.ptr()))
		}
	}

	/// [`IPropertyStore::GetAt`](https://learn.microsoft.com/en-us/windows/win32/api/propsys/nf-propsys-ipropertystore-getat)
	/// method.
	#[must_use]
	fn GetAt(&self, index: u32) -> HrResult<PROPERTYKEY> {
		let mut ppk = PROPERTYKEY::default();
		unsafe {
			let vt = self.vt_ref::<IPropertyStoreVT>();
			ok_to_hrresult(
				(vt.GetAt)(self.ptr(), index, &mut ppk as *const _ as _),
			)
		}.map(|_| ppk)
	}

	/// [`IPropertyStore::GetCount`](https://learn.microsoft.com/en-us/windows/win32/api/propsys/nf-propsys-ipropertystore-getcount)
	/// method.
	#[must_use]
	fn GetCount(&self) -> HrResult<u32> {
		let mut count = u32::default();
		unsafe {
			let vt = self.vt_ref::<IPropertyStoreVT>();
			ok_to_hrresult((vt.GetCount)(self.ptr(), &mut count))
		}.map(|_| count)
	}

	/// [`IPropertyStore::GetValue`](https://learn.microsoft.com/en-us/windows/win32/api/propsys/nf-propsys-ipropertystore-getvalue)
	/// method.
	#[must_use]
	fn GetValue(&self, key: &PROPERTYKEY) -> HrResult<PROPVARIANT> {
		let mut var = PROPVARIANT::default();
		unsafe {
			let vt = self.vt_ref::<IPropertyStoreVT>();
			match co::HRESULT(
				(vt.GetValue)(
					self.ptr(),
					key as *const _ as _,
					&mut var as *mut _ as _,
				),
			) {
				co::HRESULT::S_OK
					| co::HRESULT::INPLACE_S_TRUNCATED => Ok(var),
				hr => Err(hr),
			}
		}
	}
}

//------------------------------------------------------------------------------

struct PropertyStoreIter<'a> {
	array: ManuallyDrop<IPropertyStore>,
	count: u32,
	current: u32,
	_owner: PhantomData<&'a ()>,
}

impl<'a> Iterator for PropertyStoreIter<'a> {
	type Item = HrResult<PROPERTYKEY>;

	fn next(&mut self) -> Option<Self::Item> {
		if self.current == self.count {
			return None;
		}

		match self.array.GetAt(self.current) {
			Err(e) => {
				self.current = self.count; // no further iterations will be made
				Some(Err(e))
			},
			Ok(ppk) => {
				self.current += 1;
				Some(Ok(ppk))
			},
		}
	}
}

impl<'a> PropertyStoreIter<'a> {
	fn new(com_ptr: ComPtr) -> HrResult<Self> {
		let array = ManuallyDrop::new(IPropertyStore(com_ptr));
		let count = array.GetCount()?;

		Ok(Self {
			array,
			count,
			current: 0,
			_owner: PhantomData,
		})
	}
}
