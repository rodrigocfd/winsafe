#![allow(non_camel_case_types, non_snake_case)]

use std::mem::ManuallyDrop;
use std::sync::atomic::AtomicU32;

use crate::co;
use crate::decl::*;
use crate::kernel::ffi_types::*;
use crate::ole::{privs::*, vts::*};
use crate::prelude::*;
use crate::shell::vts::*;

com_interface_userdef! { IFileDialogEvents, IFileDialogEventsImpl: "973510db-7d7f-452b-8975-74a85828d354";
	/// [`IFileDialogEvents`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nn-shobjidl_core-ifiledialogevents)
	/// COM interface.
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
	///     move |fd: &w::IFileDialog, si: &w::IShellItem| -> w::AnyResult<()> {
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
	fn_com_interface_userdef_event! { OnFileOk: Fn(&IFileDialog) -> AnyResult<()>;
		/// [`IFileDialogEvents::OnFileOk`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifiledialogevents-onfileok)
		/// method.
	}

	fn_com_interface_userdef_event! { OnFolderChange: Fn(&IFileDialog) -> AnyResult<()>;
		/// [`IFileDialogEvents::OnFolderChange`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifiledialogevents-onfolderchange)
		/// method.
	}

	fn_com_interface_userdef_event! { OnFolderChanging: Fn(&IFileDialog, &IShellItem) -> AnyResult<()>;
		/// [`IFileDialogEvents::OnFolderChanging`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifiledialogevents-onfolderchanging)
		/// method.
	}

	fn_com_interface_userdef_event! { OnOverwrite: Fn(&IFileDialog, &IShellItem, &mut co::FDEOR) -> AnyResult<()>;
		/// [`IFileDialogEvents::OnOverwrite`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifiledialogevents-onoverwrite)
		/// method.
	}

	fn_com_interface_userdef_event! { OnSelectionChange: Fn(&IFileDialog) -> AnyResult<()>;
		/// [`IFileDialogEvents::OnSelectionChange`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifiledialogevents-onselectionchange)
		/// method.
	}

	fn_com_interface_userdef_event! { OnShareViolation: Fn(&IFileDialog, &IShellItem, &mut co::FDESVR) -> AnyResult<()>;
		/// [`IFileDialogEvents::OnShareViolation`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifiledialogevents-onshareviolation)
		/// method.
	}

	fn_com_interface_userdef_event! { OnTypeChange: Fn(&IFileDialog) -> AnyResult<()>;
		/// [`IFileDialogEvents::OnTypeChange`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifiledialogevents-ontypechange)
		/// method.
	}
}

#[repr(C)]
struct IFileDialogEventsImpl {
	vt: IFileDialogEventsVT,
	counter: AtomicU32,
	OnFileOk: Option<Box<dyn Fn(&IFileDialog) -> AnyResult<()>>>,
	OnFolderChanging: Option<Box<dyn Fn(&IFileDialog, &IShellItem) -> AnyResult<()>>>,
	OnFolderChange: Option<Box<dyn Fn(&IFileDialog) -> AnyResult<()>>>,
	OnSelectionChange: Option<Box<dyn Fn(&IFileDialog) -> AnyResult<()>>>,
	OnShareViolation:
		Option<Box<dyn Fn(&IFileDialog, &IShellItem, &mut co::FDESVR) -> AnyResult<()>>>,
	OnTypeChange: Option<Box<dyn Fn(&IFileDialog) -> AnyResult<()>>>,
	OnOverwrite: Option<Box<dyn Fn(&IFileDialog, &IShellItem, &mut co::FDEOR) -> AnyResult<()>>>,
}

impl IFileDialogEventsImpl {
	#[must_use]
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

	fn_com_interface_userdef_iunknown_impls!(Self);

	fn OnFileOk(p: COMPTR, pfd: COMPTR) -> HRES {
		let box_impl = box_impl_of::<Self>(p);
		hrresult_to_hres(match &box_impl.OnFileOk {
			Some(func) => {
				let fd = ManuallyDrop::new(unsafe { IFileDialog::from_ptr(pfd) });
				anyresult_to_hresult(func(&fd))
			},
			None => Ok(()),
		})
	}

	fn OnFolderChanging(p: COMPTR, pfd: COMPTR, psiFolder: COMPTR) -> HRES {
		let box_impl = box_impl_of::<Self>(p);
		hrresult_to_hres(match &box_impl.OnFolderChanging {
			Some(func) => {
				let fd = ManuallyDrop::new(unsafe { IFileDialog::from_ptr(pfd) });
				let si = ManuallyDrop::new(unsafe { IShellItem::from_ptr(psiFolder) });
				anyresult_to_hresult(func(&fd, &si))
			},
			None => Ok(()),
		})
	}

	fn OnFolderChange(p: COMPTR, pfd: COMPTR) -> HRES {
		let box_impl = box_impl_of::<Self>(p);
		hrresult_to_hres(match &box_impl.OnFolderChange {
			Some(func) => {
				let fd = ManuallyDrop::new(unsafe { IFileDialog::from_ptr(pfd) });
				anyresult_to_hresult(func(&fd))
			},
			None => Ok(()),
		})
	}

	fn OnSelectionChange(p: COMPTR, pfd: COMPTR) -> HRES {
		let box_impl = box_impl_of::<Self>(p);
		hrresult_to_hres(match &box_impl.OnSelectionChange {
			Some(func) => {
				let fd = ManuallyDrop::new(unsafe { IFileDialog::from_ptr(pfd) });
				anyresult_to_hresult(func(&fd))
			},
			None => Ok(()),
		})
	}

	fn OnShareViolation(p: COMPTR, pfd: COMPTR, psi: COMPTR, pResponse: *mut u32) -> HRES {
		let box_impl = box_impl_of::<Self>(p);
		hrresult_to_hres(match &box_impl.OnShareViolation {
			Some(func) => {
				let fd = ManuallyDrop::new(unsafe { IFileDialog::from_ptr(pfd) });
				let si = ManuallyDrop::new(unsafe { IShellItem::from_ptr(psi) });
				let presp = unsafe { &mut *(pResponse as *mut co::FDESVR) };
				anyresult_to_hresult(func(&fd, &si, presp))
			},
			None => Ok(()),
		})
	}

	fn OnTypeChange(p: COMPTR, pfd: COMPTR) -> HRES {
		let box_impl = box_impl_of::<Self>(p);
		hrresult_to_hres(match &box_impl.OnTypeChange {
			Some(func) => {
				let fd = ManuallyDrop::new(unsafe { IFileDialog::from_ptr(pfd) });
				anyresult_to_hresult(func(&fd))
			},
			None => Ok(()),
		})
	}

	fn OnOverwrite(p: COMPTR, pfd: COMPTR, psi: COMPTR, pResponse: *mut u32) -> HRES {
		let box_impl = box_impl_of::<Self>(p);
		hrresult_to_hres(match &box_impl.OnOverwrite {
			Some(func) => {
				let fd = ManuallyDrop::new(unsafe { IFileDialog::from_ptr(pfd) });
				let si = ManuallyDrop::new(unsafe { IShellItem::from_ptr(psi) });
				let presp = unsafe { &mut *(pResponse as *mut co::FDEOR) };
				anyresult_to_hresult(func(&fd, &si, presp))
			},
			None => Ok(()),
		})
	}
}
