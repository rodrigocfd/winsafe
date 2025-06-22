#![allow(non_camel_case_types, non_snake_case)]

use std::mem::ManuallyDrop;
use std::sync::atomic::AtomicU32;

use crate::co;
use crate::decl::*;
use crate::kernel::ffi_types::*;
use crate::ole::{privs::*, vts::*};
use crate::prelude::*;

com_interface_userdef! { IDropTarget: IDropTargetImpl, "00000122-0000-0000-c000-000000000046";
	/// [`IDropTarget`](https://learn.microsoft.com/en-us/windows/win32/api/oleidl/nn-oleidl-idroptarget)
	/// COM interface.
	///
	/// Automatically calls
	/// [`Release`](https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
	///
	/// # Examples
	///
	/// Retrieving dropped files:
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*, co};
	///
	/// let drop_target = w::IDropTarget::new_impl();
	///
	/// drop_target.Drop(
	///     |d: &w::IDataObject, key_st: co::MK, pt: w::POINT, fx: &mut co::DROPEFFECT|
	///         -> w::AnyResult<()>
	///     {
	///         let mut fmt = w::FORMATETC::default();
	///         fmt.cfFormat = co::CF::HDROP;
	///         fmt.dwAspect = co::DVASPECT::CONTENT;
	///         fmt.tymed = co::TYMED::HGLOBAL;
	///
	///         let medium = unsafe { d.GetData(&fmt)? };
	///         let hglobal = unsafe { medium.ptr_hglobal().unwrap() };
	///         let ptr_lock = hglobal.GlobalLock()?;
	///         let hdrop = unsafe { w::HDROP::from_ptr(ptr_lock.as_ptr() as _) };
	///         let dropped_paths = hdrop.DragQueryFile()?
	///             .collect::<w::SysResult<Vec<_>>>()?;
	///
	///         for f in dropped_paths {
	///             println!("> {f}");
	///         }
	///
	///         Ok(())
	///     },
	/// );
	/// ```
}

impl IDropTarget {
	fn_com_userdef_event! { DragEnter: Fn(&IDataObject, co::MK, POINT, &mut co::DROPEFFECT) -> AnyResult<()>;
		/// [`IDropTarget::DragEnter`](https://learn.microsoft.com/en-us/windows/win32/api/oleidl/nf-oleidl-idroptarget-dragenter)
		/// method.
		///
		/// # Examples
		///
		/// ```no_run
		/// use winsafe::{self as w, prelude::*, co};
		///
		/// let drop_target = w::IDropTarget::new_impl();
		///
		/// drop_target.DragEnter(
		///     |d: &w::IDataObject, key_st: co::MK, pt: w::POINT, fx: &mut co::DROPEFFECT|
		///         -> w::AnyResult<()>
		///     {
		///         *fx &= co::DROPEFFECT::COPY;
		///         Ok(())
		///     },
		/// );
		/// ```
	}

	fn_com_userdef_event! { DragLeave: Fn() -> AnyResult<()>;
		/// [`IDropTarget::DragLeave`](https://learn.microsoft.com/en-us/windows/win32/api/oleidl/nf-oleidl-idroptarget-dragleave)
		/// method.
	}

	fn_com_userdef_event! { DragOver: Fn(co::MK, POINT, &mut co::DROPEFFECT) -> AnyResult<()>;
		/// [`IDropTarget::DragOver`](https://learn.microsoft.com/en-us/windows/win32/api/oleidl/nf-oleidl-idroptarget-dragover)
		/// method.
		///
		/// # Examples
		///
		/// ```no_run
		/// use winsafe::{self as w, prelude::*, co};
		///
		/// let drop_target = w::IDropTarget::new_impl();
		///
		/// drop_target.DragOver(
		///     |key_st: co::MK, pt: w::POINT, fx: &mut co::DROPEFFECT|
		///         -> w::AnyResult<()>
		///     {
		///         *fx &= co::DROPEFFECT::COPY;
		///         Ok(())
		///     },
		/// );
		/// ```
	}

	fn_com_userdef_event! { Drop: Fn(&IDataObject, co::MK, POINT, &mut co::DROPEFFECT) -> AnyResult<()>;
		/// [`IDropTarget::Drop`](https://learn.microsoft.com/en-us/windows/win32/api/oleidl/nf-oleidl-idroptarget-drop)
		/// method.
	}
}

#[repr(C)]
struct IDropTargetImpl {
	vt: IDropTargetVT,
	counter: AtomicU32,
	DragEnter:
		Option<Box<dyn Fn(&IDataObject, co::MK, POINT, &mut co::DROPEFFECT) -> AnyResult<()>>>,
	DragOver: Option<Box<dyn Fn(co::MK, POINT, &mut co::DROPEFFECT) -> AnyResult<()>>>,
	DragLeave: Option<Box<dyn Fn() -> AnyResult<()>>>,
	Drop: Option<Box<dyn Fn(&IDataObject, co::MK, POINT, &mut co::DROPEFFECT) -> AnyResult<()>>>,
}

impl IDropTargetImpl {
	#[must_use]
	const fn new() -> Self {
		Self {
			vt: IDropTargetVT {
				IUnknownVT: IUnknownVT {
					QueryInterface: Self::QueryInterface,
					AddRef: Self::AddRef,
					Release: Self::Release,
				},
				DragEnter: Self::DragEnter,
				DragOver: Self::DragOver,
				DragLeave: Self::DragLeave,
				Drop: Self::Drop,
			},
			counter: AtomicU32::new(1),
			DragEnter: None,
			DragOver: None,
			DragLeave: None,
			Drop: None,
		}
	}

	fn_com_userdef_iunknown_impls!(Self);

	fn DragEnter(
		p: COMPTR,
		pDataObj: COMPTR,
		grfKeyState: u32,
		pt: u64,
		pdwEffect: *mut u32,
	) -> HRES {
		let box_impl = box_impl_of::<Self>(p);
		hrresult_to_hres(match &box_impl.DragEnter {
			Some(func) => {
				let dob = ManuallyDrop::new(unsafe { IDataObject::from_ptr(pDataObj) });
				let mk = unsafe { co::MK::from_raw(grfKeyState as _) };
				let pt = POINT::with(LODWORD(pt) as _, HIDWORD(pt) as _);
				let pfx = unsafe { &mut *(pdwEffect as *mut co::DROPEFFECT) };
				anyresult_to_hresult(func(&dob, mk, pt, pfx))
			},
			None => Ok(()),
		})
	}

	fn DragOver(p: COMPTR, grfKeyState: u32, pt: u64, pdwEffect: *mut u32) -> HRES {
		let box_impl = box_impl_of::<Self>(p);
		hrresult_to_hres(match &box_impl.DragOver {
			Some(func) => {
				let mk = unsafe { co::MK::from_raw(grfKeyState as _) };
				let pt = POINT::with(LODWORD(pt) as _, HIDWORD(pt) as _);
				let pfx = unsafe { &mut *(pdwEffect as *mut co::DROPEFFECT) };
				anyresult_to_hresult(func(mk, pt, pfx))
			},
			None => Ok(()),
		})
	}

	fn_com_userdef_impl_noparm!(DragLeave);

	fn Drop(p: COMPTR, pDataObj: COMPTR, grfKeyState: u32, pt: u64, pdwEffect: *mut u32) -> HRES {
		let box_impl = box_impl_of::<Self>(p);
		hrresult_to_hres(match &box_impl.Drop {
			Some(func) => {
				let dob = ManuallyDrop::new(unsafe { IDataObject::from_ptr(pDataObj) });
				let mk = unsafe { co::MK::from_raw(grfKeyState as _) };
				let pt = POINT::with(LODWORD(pt) as _, HIDWORD(pt) as _);
				let pfx = unsafe { &mut *(pdwEffect as *mut co::DROPEFFECT) };
				anyresult_to_hresult(func(&dob, mk, pt, pfx))
			},
			None => Ok(()),
		})
	}
}
