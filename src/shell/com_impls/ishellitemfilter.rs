#![allow(non_camel_case_types, non_snake_case)]

use std::mem::ManuallyDrop;
use std::sync::atomic::AtomicU32;

use crate::co;
use crate::decl::*;
use crate::kernel::ffi_types::*;
use crate::ole::{privs::*, vts::*};
use crate::prelude::*;
use crate::shell::vts::*;

com_interface_userdef! { IShellItemFilter: IShellItemFilterImpl, "2659b475-eeb8-48b7-8f07-b378810f48cf";
	/// [`IShellItemFilter`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nn-shobjidl_core-ishellitemfilter)
	/// COM interface.
	///
	/// Automatically calls
	/// [`IUnknown::Release`](https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
}

impl IShellItemFilter {
	fn_com_userdef_event! { IncludeItem: Fn(&IShellItem) -> AnyResult<()>;
		/// [`IShellItemFilter::IncludeItem`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ishellitemfilter-includeitem)
		/// method.
	}

	fn_com_userdef_event! { GetEnumFlagsForItem: Fn(&IShellItem, &mut co::SHCONTF) -> AnyResult<()>;
		/// [`IShellItemFilter::GetEnumFlagsForItem`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ishellitemfilter-getenumflagsforitem)
		/// method.
	}
}

#[repr(C)]
struct IShellItemFilterImpl {
	vt: IShellItemFilterVT,
	counter: AtomicU32,
	IncludeItem: Option<Box<dyn Fn(&IShellItem) -> AnyResult<()>>>,
	GetEnumFlagsForItem: Option<Box<dyn Fn(&IShellItem, &mut co::SHCONTF) -> AnyResult<()>>>,
}

impl IShellItemFilterImpl {
	#[must_use]
	const fn new() -> Self {
		Self {
			vt: IShellItemFilterVT {
				IUnknownVT: IUnknownVT {
					QueryInterface: Self::QueryInterface,
					AddRef: Self::AddRef,
					Release: Self::Release,
				},
				IncludeItem: Self::IncludeItem,
				GetEnumFlagsForItem: Self::GetEnumFlagsForItem,
			},
			counter: AtomicU32::new(1),
			IncludeItem: None,
			GetEnumFlagsForItem: None,
		}
	}

	fn_com_userdef_iunknown_impls!(Self);

	fn IncludeItem(p: COMPTR, psi: COMPTR) -> HRES {
		let box_impl = box_impl_of::<Self>(p);
		hrresult_to_hres(match &box_impl.IncludeItem {
			Some(func) => {
				anyresult_to_hresult(func(&ManuallyDrop::new(unsafe { IShellItem::from_ptr(psi) })))
			},
			None => Ok(()),
		})
	}

	fn GetEnumFlagsForItem(p: COMPTR, psi: COMPTR, pgrfFlags: *mut u32) -> HRES {
		let box_impl = box_impl_of::<Self>(p);
		hrresult_to_hres(match &box_impl.GetEnumFlagsForItem {
			Some(func) => unsafe {
				anyresult_to_hresult(func(
					&ManuallyDrop::new(IShellItem::from_ptr(psi)),
					&mut *(pgrfFlags as *mut co::SHCONTF),
				))
			},
			None => Ok(()),
		})
	}
}
