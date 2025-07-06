#![allow(non_camel_case_types, non_snake_case)]

use std::mem::ManuallyDrop;
use std::sync::atomic::AtomicU32;

use crate::co;
use crate::decl::*;
use crate::kernel::ffi_types::*;
use crate::ole::{privs::*, vts::*};
use crate::prelude::*;
use crate::shell::vts::*;

com_interface_userdef! { IOperationsProgressDialog: IOperationsProgressDialogImpl, "0c9fb851-e5c9-43eb-a370-f0677b13874c";
	/// [`IOperationsProgressDialog`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nn-shobjidl_core-ioperationsprogressdialog)
	/// COM interface.
	///
	/// Automatically calls
	/// [`IUnknown::Release`](https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
}

impl IOperationsProgressDialog {
	fn_com_userdef_event! { StartProgressDialog: Fn(&HWND, co::PROGDLG) -> AnyResult<()>;
		/// [`IOperationsProgressDialog::StartProgressDialog`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ioperationsprogressdialog-startprogressdialog)
		/// method.
	}

	fn_com_userdef_event! { StopProgressDialog: Fn() -> AnyResult<()>;
		/// [`IOperationsProgressDialog::StopProgressDialog`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ioperationsprogressdialog-stopprogressdialog)
		/// method.
	}

	fn_com_userdef_event! { SetOperation: Fn(co::SPACTION) -> AnyResult<()>;
		/// [`IOperationsProgressDialog::SetOperation`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ioperationsprogressdialog-setoperation)
		/// method.
	}

	fn_com_userdef_event! { SetMode: Fn(co::PDM) -> AnyResult<()>;
		/// [`IOperationsProgressDialog::SetMode`]()
		/// method.
	}

	fn_com_userdef_event! { UpdateProgress: Fn(u64, u64, u64, u64, u64, u64) -> AnyResult<()>;
		/// [`IOperationsProgressDialog::UpdateProgress`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ioperationsprogressdialog-updateprogress)
		/// method.
	}

	fn_com_userdef_event! { UpdateLocations: Fn(&IShellItem, &IShellItem, &IShellItem) -> AnyResult<()>;
		/// [`IOperationsProgressDialog::UpdateLocations`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ioperationsprogressdialog-updatelocations)
		/// method.
	}

	fn_com_userdef_event! { ResetTimer: Fn() -> AnyResult<()>;
		/// [`IOperationsProgressDialog::ResetTimer`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ioperationsprogressdialog-resettimer)
		/// method.
	}

	fn_com_userdef_event! { PauseTimer: Fn() -> AnyResult<()>;
		/// [`IOperationsProgressDialog::PauseTimer`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ioperationsprogressdialog-pausetimer)
		/// method.
	}

	fn_com_userdef_event! { ResumeTimer: Fn() -> AnyResult<()>;
		/// [`IOperationsProgressDialog::ResumeTimer`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ioperationsprogressdialog-resumetimer)
		/// method.
	}

	fn_com_userdef_event! { GetMilliseconds: Fn(&mut u64, &mut u64) -> AnyResult<()>;
		/// [`IOperationsProgressDialog::GetMilliseconds`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ioperationsprogressdialog-getmilliseconds)
		/// method.
	}

	fn_com_userdef_event! { GetOperationStatus: Fn(&mut co::PDOPS) -> AnyResult<()>;
		/// [`IOperationsProgressDialog::GetOperationStatus`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ioperationsprogressdialog-getoperationstatus)
		/// method.
	}
}

#[repr(C)]
struct IOperationsProgressDialogImpl {
	vt: IOperationsProgressDialogVT,
	counter: AtomicU32,
	StartProgressDialog: Option<Box<dyn Fn(&HWND, co::PROGDLG) -> AnyResult<()>>>,
	StopProgressDialog: Option<Box<dyn Fn() -> AnyResult<()>>>,
	SetOperation: Option<Box<dyn Fn(co::SPACTION) -> AnyResult<()>>>,
	SetMode: Option<Box<dyn Fn(co::PDM) -> AnyResult<()>>>,
	UpdateProgress: Option<Box<dyn Fn(u64, u64, u64, u64, u64, u64) -> AnyResult<()>>>,
	UpdateLocations: Option<Box<dyn Fn(&IShellItem, &IShellItem, &IShellItem) -> AnyResult<()>>>,
	ResetTimer: Option<Box<dyn Fn() -> AnyResult<()>>>,
	PauseTimer: Option<Box<dyn Fn() -> AnyResult<()>>>,
	ResumeTimer: Option<Box<dyn Fn() -> AnyResult<()>>>,
	GetMilliseconds: Option<Box<dyn Fn(&mut u64, &mut u64) -> AnyResult<()>>>,
	GetOperationStatus: Option<Box<dyn Fn(&mut co::PDOPS) -> AnyResult<()>>>,
}

impl IOperationsProgressDialogImpl {
	#[must_use]
	const fn new() -> Self {
		Self {
			vt: IOperationsProgressDialogVT {
				IUnknownVT: IUnknownVT {
					QueryInterface: Self::QueryInterface,
					AddRef: Self::AddRef,
					Release: Self::Release,
				},
				StartProgressDialog: Self::StartProgressDialog,
				StopProgressDialog: Self::StopProgressDialog,
				SetOperation: Self::SetOperation,
				SetMode: Self::SetMode,
				UpdateProgress: Self::UpdateProgress,
				UpdateLocations: Self::UpdateLocations,
				ResetTimer: Self::ResetTimer,
				PauseTimer: Self::PauseTimer,
				ResumeTimer: Self::ResumeTimer,
				GetMilliseconds: Self::GetMilliseconds,
				GetOperationStatus: Self::GetOperationStatus,
			},
			counter: AtomicU32::new(1),
			StartProgressDialog: None,
			StopProgressDialog: None,
			SetOperation: None,
			SetMode: None,
			UpdateProgress: None,
			UpdateLocations: None,
			ResetTimer: None,
			PauseTimer: None,
			ResumeTimer: None,
			GetMilliseconds: None,
			GetOperationStatus: None,
		}
	}

	fn_com_userdef_iunknown_impls!(Self);

	fn StartProgressDialog(p: COMPTR, hwndOwner: HANDLE, flags: u32) -> HRES {
		let box_impl = box_impl_of::<Self>(p);
		hrresult_to_hres(match &box_impl.StartProgressDialog {
			Some(func) => unsafe {
				anyresult_to_hresult(func(&HWND::from_ptr(hwndOwner), co::PROGDLG::from_raw(flags)))
			},
			None => Ok(()),
		})
	}

	fn_com_userdef_impl_noparm!(StopProgressDialog);

	fn SetOperation(p: COMPTR, action: u32) -> HRES {
		let box_impl = box_impl_of::<Self>(p);
		hrresult_to_hres(match &box_impl.SetOperation {
			Some(func) => anyresult_to_hresult(func(unsafe { co::SPACTION::from_raw(action) })),
			None => Ok(()),
		})
	}

	fn SetMode(p: COMPTR, mode: u32) -> HRES {
		let box_impl = box_impl_of::<Self>(p);
		hrresult_to_hres(match &box_impl.SetMode {
			Some(func) => anyresult_to_hresult(func(unsafe { co::PDM::from_raw(mode) })),
			None => Ok(()),
		})
	}

	fn UpdateProgress(
		p: COMPTR,
		ullPointsCurrent: u64,
		ullPointsTotal: u64,
		ullSizeCurrent: u64,
		ullSizeTotal: u64,
		ullItemsCurrent: u64,
		ullItemsTotal: u64,
	) -> HRES {
		let box_impl = box_impl_of::<Self>(p);
		hrresult_to_hres(match &box_impl.UpdateProgress {
			Some(func) => anyresult_to_hresult(func(
				ullPointsCurrent,
				ullPointsTotal,
				ullSizeCurrent,
				ullSizeTotal,
				ullItemsCurrent,
				ullItemsTotal,
			)),
			None => Ok(()),
		})
	}

	fn UpdateLocations(p: COMPTR, psiSource: COMPTR, psiTarget: COMPTR, psiItem: COMPTR) -> HRES {
		let box_impl = box_impl_of::<Self>(p);
		hrresult_to_hres(match &box_impl.UpdateLocations {
			Some(func) => unsafe {
				anyresult_to_hresult(func(
					&ManuallyDrop::new(IShellItem::from_ptr(psiSource)),
					&ManuallyDrop::new(IShellItem::from_ptr(psiTarget)),
					&ManuallyDrop::new(IShellItem::from_ptr(psiItem)),
				))
			},
			None => Ok(()),
		})
	}

	fn_com_userdef_impl_noparm!(ResetTimer);

	fn_com_userdef_impl_noparm!(PauseTimer);

	fn_com_userdef_impl_noparm!(ResumeTimer);

	fn GetMilliseconds(p: COMPTR, pullElapsed: *mut u64, pullRemaining: *mut u64) -> HRES {
		let box_impl = box_impl_of::<Self>(p);
		hrresult_to_hres(match &box_impl.GetMilliseconds {
			Some(func) => {
				anyresult_to_hresult(unsafe { func(&mut *pullElapsed, &mut *pullRemaining) })
			},
			None => Ok(()),
		})
	}

	fn GetOperationStatus(p: COMPTR, popstatus: *mut u32) -> HRES {
		let box_impl = box_impl_of::<Self>(p);
		hrresult_to_hres(match &box_impl.GetOperationStatus {
			Some(func) => {
				anyresult_to_hresult(func(unsafe { &mut *(popstatus as *mut co::PDOPS) }))
			},
			None => Ok(()),
		})
	}
}
