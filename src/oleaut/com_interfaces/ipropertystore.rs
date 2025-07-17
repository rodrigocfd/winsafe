#![allow(non_camel_case_types, non_snake_case)]

use crate::co;
use crate::decl::*;
use crate::kernel::privs::*;
use crate::ole::privs::*;
use crate::oleaut::{iterators::*, vts::*};
use crate::prelude::*;

com_interface! { IPropertyStore: "886d8eeb-8cf2-4446-8d02-cdba1dbdcf99";
	/// [`IPropertyStore`](https://learn.microsoft.com/en-us/windows/win32/api/propsys/nn-propsys-ipropertystore)
	/// COM interface.
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
/// ```no_run
/// use winsafe::prelude::*;
/// ```
pub trait oleaut_IPropertyStore: ole_IUnknown {
	/// Returns an iterator over the [`co::PKEY`](crate::co::PKEY) elements by
	/// calling
	/// [`IPropertyStore::GetCount`](crate::prelude::oleaut_IPropertyStore::GetCount)
	/// and
	/// [`IPropertyStore::GetAt`](crate::prelude::oleaut_IPropertyStore::GetAt)
	/// consecutively.
	///
	/// # Examples
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*};
	///
	/// let pstore: w::IPropertyStore; // initialized somewhere
	/// # let pstore = unsafe { w::IPropertyStore::null() };
	///
	/// for pkey in pstore.iter()? {
	///     let pkey = pkey?;
	///     // ...
	/// }
	/// # w::HrResult::Ok(())
	/// ```
	#[must_use]
	fn iter(&self) -> HrResult<impl Iterator<Item = HrResult<co::PKEY>> + '_> {
		Ok(IpropertystoreIter::new(self)?)
	}

	fn_com_noparm! { Commit: IPropertyStoreVT;
		/// [`IPropertyStore::Commit`](https://learn.microsoft.com/en-us/windows/win32/api/propsys/nf-propsys-ipropertystore-commit)
		/// method.
	}

	/// [`IPropertyStore::GetAt`](https://learn.microsoft.com/en-us/windows/win32/api/propsys/nf-propsys-ipropertystore-getat)
	/// method.
	#[must_use]
	fn GetAt(&self, index: u32) -> HrResult<co::PKEY> {
		let mut pkey = co::PKEY::default();
		HrRet(unsafe { (vt::<IPropertyStoreVT>(self).GetAt)(self.ptr(), index, pvoid(&mut pkey)) })
			.to_hrresult()
			.map(|_| pkey)
	}

	/// [`IPropertyStore::GetCount`](https://learn.microsoft.com/en-us/windows/win32/api/propsys/nf-propsys-ipropertystore-getcount)
	/// method.
	#[must_use]
	fn GetCount(&self) -> HrResult<u32> {
		let mut count = 0u32;
		HrRet(unsafe { (vt::<IPropertyStoreVT>(self).GetCount)(self.ptr(), &mut count) })
			.to_hrresult()
			.map(|_| count)
	}

	/// [`IPropertyStore::GetValue`](https://learn.microsoft.com/en-us/windows/win32/api/propsys/nf-propsys-ipropertystore-getvalue)
	/// method.
	#[must_use]
	fn GetValue(&self, key: &co::PKEY) -> HrResult<PropVariant> {
		let mut var = PROPVARIANT::default();
		match unsafe {
			co::HRESULT::from_raw((vt::<IPropertyStoreVT>(self).GetValue)(
				self.ptr(),
				pcvoid(key),
				pvoid(&mut var),
			))
		} {
			co::HRESULT::S_OK | co::HRESULT::INPLACE_S_TRUNCATED => {
				Ok(PropVariant::from_raw(&var)?)
			},
			hr => Err(hr),
		}
	}

	/// [`IPropertyStore::SetValue`](https://learn.microsoft.com/en-us/windows/win32/api/propsys/nf-propsys-ipropertystore-setvalue)
	/// method.
	fn SetValue(&self, key: &co::PKEY, value: &PropVariant) -> HrResult<()> {
		HrRet(unsafe {
			(vt::<IPropertyStoreVT>(self).SetValue)(
				self.ptr(),
				pcvoid(key),
				pcvoid(&value.to_raw()?),
			)
		})
		.to_hrresult()
	}
}
