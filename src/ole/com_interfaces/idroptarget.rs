#![allow(non_camel_case_types, non_snake_case)]

use std::mem::ManuallyDrop;
use std::sync::atomic::AtomicU32;

use crate::co;
use crate::decl::*;
use crate::kernel::ffi_types::*;
use crate::ole::{privs::*, vts::*};
use crate::prelude::*;

com_interface_userdef! { IDropTarget, IDropTargetImpl: "00000122-0000-0000-c000-000000000046";
	/// [`IDropTarget`](https://learn.microsoft.com/en-us/windows/win32/api/oleidl/nn-oleidl-idroptarget)
	/// COM interface.
	///
	/// Automatically calls
	/// [`Release`](https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
	///
	/// # Examples
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*, co};
	///
	/// let drop_target = w::IDropTarget::new_impl();
	///
	/// drop_target.Drop(
	///     |d: &w::IDataObject, mk: co::MK, pt: w::POINT, de: &mut co::DROPEFFECT|
	///         -> w::HrResult<()>
	///     {
	///         println!("X: {}, Y: {}", pt.x, pt.y);
	///         *de &= co::DROPEFFECT::COPY;
	///         Ok(())
	///     },
	/// );
	/// ```
}

impl IDropTarget {
	fn_com_userdef_closure! { DragEnter: Fn(&IDataObject, co::MK, POINT, &mut co::DROPEFFECT) -> HrResult<()>;
		/// [`IDropTarget::DragEnter`](https://learn.microsoft.com/en-us/windows/win32/api/oleidl/nf-oleidl-idroptarget-dragenter)
		/// method.
	}

	fn_com_userdef_closure! { DragLeave: Fn() -> HrResult<()>;
		/// [`IDropTarget::DragLeave`](https://learn.microsoft.com/en-us/windows/win32/api/oleidl/nf-oleidl-idroptarget-dragleave)
		/// method.
	}

	fn_com_userdef_closure! { DragOver: Fn(co::MK, POINT, &mut co::DROPEFFECT) -> HrResult<()>;
		/// [`IDropTarget::DragOver`](https://learn.microsoft.com/en-us/windows/win32/api/oleidl/nf-oleidl-idroptarget-dragover)
		/// method.
	}

	fn_com_userdef_closure! { Drop: Fn(&IDataObject, co::MK, POINT, &mut co::DROPEFFECT) -> HrResult<()>;
		/// [`IDropTarget::Drop`](https://learn.microsoft.com/en-us/windows/win32/api/oleidl/nf-oleidl-idroptarget-drop)
		/// method.
	}
}

#[repr(C)]
pub struct IDropTargetImpl {
	vt: IDropTargetVT,
	counter: AtomicU32,
	DragEnter: Option<Box<dyn Fn(&IDataObject, co::MK, POINT, &mut co::DROPEFFECT) -> HrResult<()>>>,
	DragOver: Option<Box<dyn Fn(co::MK, POINT, &mut co::DROPEFFECT) -> HrResult<()>>>,
	DragLeave: Option<Box<dyn Fn() -> HrResult<()>>>,
	Drop: Option<Box<dyn Fn(&IDataObject, co::MK, POINT, &mut co::DROPEFFECT) -> HrResult<()>>>,
}

impl IDropTargetImpl {
	fn new() -> Self {
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

 	com_interface_userdef_iunknown_methods!(Self);

	fn DragEnter(
		p: COMPTR,
		pDataObj: COMPTR,
		grfKeyState: u32,
		pt: u64,
		pdwEffect: *mut u32,
	) -> HRES
	{
		let box_impl = box_impl_of::<Self>(p);
		hrresult_to_hres(
			match &box_impl.DragEnter {
				Some(func) => {
					let dob = ManuallyDrop::new(unsafe { IDataObject::from_ptr(pDataObj) });
					let mk = unsafe { co::MK::from_raw(grfKeyState as _) };
					let pt = POINT::new(LODWORD(pt) as _, HIDWORD(pt) as _);
					let pfx = unsafe { &mut *(pdwEffect as *mut co::DROPEFFECT) };
					func(&dob, mk, pt, pfx)
				},
				None => Ok(()),
			},
		)
	}

	fn DragOver(
		p: COMPTR,
		grfKeyState: u32,
		pt: u64,
		pdwEffect: *mut u32,
	) -> HRES
	{
		let box_impl = box_impl_of::<Self>(p);
		hrresult_to_hres(
			match &box_impl.DragOver {
				Some(func) => {
					let mk = unsafe { co::MK::from_raw(grfKeyState as _) };
					let pt = POINT::new(LODWORD(pt) as _, HIDWORD(pt) as _);
					let pfx = unsafe { &mut *(pdwEffect as *mut co::DROPEFFECT) };
					func(mk, pt, pfx)
				},
				None => Ok(()),
			},
		)
	}

	fn DragLeave(p: COMPTR) -> HRES {
		let box_impl = box_impl_of::<Self>(p);
		hrresult_to_hres(
			match &box_impl.DragLeave {
				Some(func) => func(),
				None => Ok(()),
			},
		)
	}

	fn Drop(
		p: COMPTR,
		pDataObj: COMPTR,
		grfKeyState: u32,
		pt: u64,
		pdwEffect: *mut u32,
	) -> HRES
	{
		let box_impl = box_impl_of::<Self>(p);
		hrresult_to_hres(
			match &box_impl.Drop {
				Some(func) => {
					let dob = ManuallyDrop::new(unsafe { IDataObject::from_ptr(pDataObj) });
					let mk = unsafe { co::MK::from_raw(grfKeyState as _) };
					let pt = POINT::new(LODWORD(pt) as _, HIDWORD(pt) as _);
					let pfx = unsafe { &mut *(pdwEffect as *mut co::DROPEFFECT) };
					func(&dob, mk, pt, pfx)
				},
				None => Ok(()),
			},
		)
	}
}
