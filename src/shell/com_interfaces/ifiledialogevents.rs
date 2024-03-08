#![allow(non_camel_case_types, non_snake_case)]

use std::mem::ManuallyDrop;
use std::sync::atomic::{AtomicU32, Ordering};

use crate::co;
use crate::decl::*;
use crate::kernel::ffi_types::*;
use crate::ole::privs::*;
use crate::prelude::*;
use crate::vt::*;

/// [`IFileDialogEvents`](crate::IFileDialogEvents) virtual table.
#[repr(C)]
pub struct IFileDialogEventsVT {
	pub IUnknownVT: IUnknownVT,
	pub OnFileOk: fn(COMPTR, COMPTR) -> HRES,
	pub OnFolderChanging: fn(COMPTR, COMPTR, COMPTR) -> HRES,
	pub OnFolderChange: fn(COMPTR, COMPTR) -> HRES,
	pub OnSelectionChange: fn(COMPTR, COMPTR) -> HRES,
	pub OnShareViolation: fn(COMPTR, COMPTR, COMPTR, *mut u32) -> HRES,
	pub OnTypeChange: fn(COMPTR, COMPTR) -> HRES,
	pub OnOverwrite: fn(COMPTR, COMPTR, COMPTR, *mut u32) -> HRES,
}

//------------------------------------------------------------------------------

#[repr(C)]
struct IFileDialogEventsImpl {
	vt: IFileDialogEventsVT,
	counter: AtomicU32,
	OnFileOk: Option<Box<dyn Fn(&IFileDialog) -> HrResult<()>>>,
	OnFolderChanging: Option<Box<dyn Fn(&IFileDialog, &IShellItem) -> HrResult<()>>>,
	OnFolderChange: Option<Box<dyn Fn(&IFileDialog) -> HrResult<()>>>,
	OnSelectionChange: Option<Box<dyn Fn(&IFileDialog) -> HrResult<()>>>,
	OnShareViolation: Option<Box<dyn Fn(&IFileDialog, &IShellItem) -> HrResult<co::FDESVR>>>,
	OnTypeChange: Option<Box<dyn Fn(&IFileDialog) -> HrResult<()>>>,
	OnOverwrite: Option<Box<dyn Fn(&IFileDialog, &IShellItem) -> HrResult<co::FDEOR>>>,
}

impl IFileDialogEventsImpl {
	const fn new() -> Self {
		Self {
			vt: IFileDialogEventsVT {
				IUnknownVT: IUnknownVT {
					QueryInterface: Self::QueryInterface,
					AddRef: Self::AddRef,
					Release: Self::Release,
				},
				OnFileOk: Self::OnFileOk,
				OnFolderChanging: Self::OnFolderChanging,
				OnFolderChange: Self::OnFolderChange,
				OnSelectionChange: Self::OnSelectionChange,
				OnShareViolation: Self::OnShareViolation,
				OnTypeChange: Self::OnTypeChange,
				OnOverwrite: Self::OnOverwrite,
			},
			counter: AtomicU32::new(1),
			OnFileOk: None,
			OnFolderChanging: None,
			OnFolderChange: None,
			OnSelectionChange: None,
			OnShareViolation: None,
			OnTypeChange: None,
			OnOverwrite: None,
		}
	}

	fn QueryInterface(_p: COMPTR, _riid: PCVOID, ppv: *mut COMPTR) -> HRES {
		unsafe { *ppv = std::ptr::null_mut(); }
		co::HRESULT::E_NOTIMPL.raw()
	}

	fn AddRef(p: COMPTR) -> u32 {
		let box_impl = box_impl::<Self>(p);
		let cc = box_impl.counter.fetch_add(1, Ordering::Relaxed) + 1;
		cc
	}

	fn Release(p: COMPTR) -> u32 {
		let mut box_impl = box_impl::<Self>(p);
		let count = box_impl.counter.fetch_sub(1, Ordering::Relaxed) - 1;
		if count == 0 {
			unsafe { ManuallyDrop::drop(&mut box_impl); }
		}
		count
	}

	fn OnFileOk(p: COMPTR, pfd: COMPTR) -> HRES {
		let box_impl = box_impl::<Self>(p);
		hrresult_to_hres(
			&match &box_impl.OnFileOk {
				Some(func) => {
					let fd = ManuallyDrop::new(unsafe { IFileDialog::from_ptr(pfd) });
					func(&fd)
				},
				None => Ok(()),
			},
		)
	}

	fn OnFolderChanging(p: COMPTR, pfd: COMPTR, psiFolder: COMPTR) -> HRES {
		let box_impl = box_impl::<Self>(p);
		hrresult_to_hres(
			&match &box_impl.OnFolderChanging {
				Some(func) => {
					let fd = ManuallyDrop::new(unsafe { IFileDialog::from_ptr(pfd) });
					let si = ManuallyDrop::new(unsafe { IShellItem::from_ptr(psiFolder) });
					func(&fd, &si)
				},
				None => Ok(()),
			},
		)
	}

	fn OnFolderChange(p: COMPTR, pfd: COMPTR) -> HRES {
		let box_impl = box_impl::<Self>(p);
		hrresult_to_hres(
			&match &box_impl.OnFolderChange {
				Some(func) => {
					let fd = ManuallyDrop::new(unsafe { IFileDialog::from_ptr(pfd) });
					func(&fd)
				},
				None => Ok(()),
			},
		)
	}

	fn OnSelectionChange(p: COMPTR, pfd: COMPTR) -> HRES {
		let box_impl = box_impl::<Self>(p);
		hrresult_to_hres(
			&match &box_impl.OnSelectionChange {
				Some(func) => {
					let fd = ManuallyDrop::new(unsafe { IFileDialog::from_ptr(pfd) });
					func(&fd)
				},
				None => Ok(()),
			},
		)
	}

	fn OnShareViolation(
		p: COMPTR,
		pfd: COMPTR,
		psi: COMPTR,
		pResponse: *mut u32,
	) -> HRES
	{
		let box_impl = box_impl::<Self>(p);
		let ret = match &box_impl.OnShareViolation {
			Some(func) => {
				let fd = ManuallyDrop::new(unsafe { IFileDialog::from_ptr(pfd) });
				let si = ManuallyDrop::new(unsafe { IShellItem::from_ptr(psi) });
				func(&fd, &si)
			},
			None => Ok(co::FDESVR::DEFAULT),
		};
		match ret {
			Ok(ret) => {
				unsafe { *pResponse = ret.raw(); }
				co::HRESULT::S_OK.raw()
			},
			Err(e) => e.raw(),
		}
	}

	fn OnTypeChange(p: COMPTR, pfd: COMPTR) -> HRES {
		let box_impl = box_impl::<Self>(p);
		hrresult_to_hres(
			&match &box_impl.OnTypeChange {
				Some(func) => {
					let fd = ManuallyDrop::new(unsafe { IFileDialog::from_ptr(pfd) });
					func(&fd)
				},
				None => Ok(()),
			},
		)
	}

	fn OnOverwrite(
		p: COMPTR,
		pfd: COMPTR,
		psi: COMPTR,
		pResponse: *mut u32,
	) -> HRES
	{
		let box_impl = box_impl::<Self>(p);
		let ret = match &box_impl.OnOverwrite {
			Some(func) => {
				let fd = ManuallyDrop::new(unsafe { IFileDialog::from_ptr(pfd) });
				let si = ManuallyDrop::new(unsafe { IShellItem::from_ptr(psi) });
				func(&fd, &si)
			},
			None => Ok(co::FDEOR::DEFAULT),
		};
		match ret {
			Ok(ret) => {
				unsafe { *pResponse = ret.raw(); }
				co::HRESULT::S_OK.raw()
			},
			Err(e) => e.raw(),
		}
	}
}

//------------------------------------------------------------------------------

com_interface_custom! { IFileDialogEvents, IFileDialogEventsImpl: "973510db-7d7f-452b-8975-74a85828d354";
	/// [`IFileDialogEvents`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nn-shobjidl_core-ifiledialogevents)
	/// COM interface over [`IFileDialogEventsVT`](crate::vt::IFileDialogEventsVT).
	///
	/// Automatically calls
	/// [`IUnknown::Release`](https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
	///
	/// # Examples
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*, co};
	///
	/// let hparent: w::HWND; // initialized somewhere
	/// # let hparent = w::HWND::NULL;
	///
	/// let file_open = w::CoCreateInstance::<w::IFileOpenDialog>(
	///     &co::CLSID::FileOpenDialog,
	///     None,
	///     co::CLSCTX::INPROC_SERVER,
	/// )?;
	///
	/// let file_dialog_events = w::IFileDialogEvents::new_impl();
	///
	/// file_dialog_events.OnFolderChanging(
	///     move |fd: &w::IFileDialog, si: &w::IShellItem| -> w::HrResult<()> {
	///         println!("New folder: {}",
	///             si.GetDisplayName(co::SIGDN::FILESYSPATH)?);
	///         Ok(())
	///     },
	/// );
	///
	/// file_open.Advise(&file_dialog_events)?;
	/// # w::HrResult::Ok(())
	/// ```
}

impl IFileDialogEvents {
	/// Creates a custom COM implementation, to which you can add closures to
	/// handle events.
	#[must_use]
	pub fn new_impl() -> Self {
		let box_impl = Box::new(IFileDialogEventsImpl::new());
		Self(Box::into_raw(box_impl))
	}

	fn_com_closure! { OnFileOk: Fn(&IFileDialog) -> HrResult<()>;
		/// [`IFileDialogEvents::OnFileOk`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifiledialogevents-onfileok)
		/// method.
	}

	fn_com_closure! { OnFolderChange: Fn(&IFileDialog) -> HrResult<()>;
		/// [`IFileDialogEvents::OnFolderChange`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifiledialogevents-onfolderchange)
		/// method.
	}

	fn_com_closure! { OnFolderChanging: Fn(&IFileDialog, &IShellItem) -> HrResult<()>;
		/// [`IFileDialogEvents::OnFolderChanging`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifiledialogevents-onfolderchanging)
		/// method.
	}

	fn_com_closure! { OnOverwrite: Fn(&IFileDialog, &IShellItem) -> HrResult<co::FDEOR>;
		/// [`IFileDialogEvents::OnOverwrite`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifiledialogevents-onoverwrite)
		/// method.
	}

	fn_com_closure! { OnSelectionChange: Fn(&IFileDialog) -> HrResult<()>;
		/// [`IFileDialogEvents::OnSelectionChange`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifiledialogevents-onselectionchange)
		/// method.
	}

	fn_com_closure! { OnShareViolation: Fn(&IFileDialog, &IShellItem) -> HrResult<co::FDESVR>;
		/// [`IFileDialogEvents::OnShareViolation`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifiledialogevents-onshareviolation)
		/// method.
	}

	fn_com_closure! { OnTypeChange: Fn(&IFileDialog) -> HrResult<()>;
		/// [`IFileDialogEvents::OnTypeChange`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifiledialogevents-ontypechange)
		/// method.
	}
}
