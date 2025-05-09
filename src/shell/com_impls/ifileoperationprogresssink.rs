#![allow(non_camel_case_types, non_snake_case)]

use std::mem::ManuallyDrop;
use std::sync::atomic::AtomicU32;

use crate::co;
use crate::decl::*;
use crate::kernel::ffi_types::*;
use crate::ole::{privs::*, vts::*};
use crate::prelude::*;
use crate::shell::vts::*;

com_interface_userdef! { IFileOperationProgressSink, IFileOperationProgressSinkImpl: "04b0f1a7-9490-44bc-96e1-4296a31252e2";
	/// [`IFileOperationProgressSink`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nn-shobjidl_core-ifileoperationprogresssink)
	/// COM interface.
	///
	/// Automatically calls
	/// [`IUnknown::Release`](https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
}

impl IFileOperationProgressSink {
	fn_com_interface_userdef_event! { StartOperations: Fn() -> AnyResult<()>;
		/// [`IFileOperationProgressSink::StartOperations`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifileoperationprogresssink-startoperations)
		/// method.
	}

	fn_com_interface_userdef_event! { FinishOperations: Fn(co::HRESULT) -> AnyResult<()>;
		/// [`IFileOperationProgressSink::FinishOperations`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifileoperationprogresssink-finishoperations)
		/// method.
	}

	fn_com_interface_userdef_event! { PreRenameItem: Fn(co::TSF, &IShellItem, &str) -> AnyResult<()>;
		/// [`IFileOperationProgressSink::PreRenameItem`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifileoperationprogresssink-prerenameitem)
		/// method.
	}

	fn_com_interface_userdef_event! { PostRenameItem: Fn(co::TSF, &IShellItem, &str, co::HRESULT, &IShellItem) -> AnyResult<()>;
		/// [`IFileOperationProgressSink::PostRenameItem`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifileoperationprogresssink-postrenameitem)
		/// method.
	}

	fn_com_interface_userdef_event! { PreMoveItem: Fn(co::TSF, &IShellItem, &IShellItem, &str) -> AnyResult<()>;
		/// [`IFileOperationProgressSink::PreMoveItem`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifileoperationprogresssink-premoveitem)
		/// method.
	}

	fn_com_interface_userdef_event! { PostMoveItem: Fn(co::TSF, &IShellItem, &IShellItem, &str, co::HRESULT, &IShellItem) -> AnyResult<()>;
		/// [`IFileOperationProgressSink::PostMoveItem`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifileoperationprogresssink-postmoveitem)
		/// method.
	}

	fn_com_interface_userdef_event! { PreCopyItem: Fn(co::TSF, &IShellItem, &IShellItem, &str) -> AnyResult<()>;
		/// [`IFileOperationProgressSink::PreCopyItem`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifileoperationprogresssink-precopyitem)
		/// method.
	}

	fn_com_interface_userdef_event! { PostCopyItem: Fn(co::TSF, &IShellItem, &IShellItem, &str, co::HRESULT, &IShellItem) -> AnyResult<()>;
		/// [`IFileOperationProgressSink::PostCopyItem`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifileoperationprogresssink-postcopyitem)
		/// method.
	}

	fn_com_interface_userdef_event! { PreDeleteItem: Fn(co::TSF, &IShellItem) -> AnyResult<()>;
		/// [`IFileOperationProgressSink::PreDeleteItem`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifileoperationprogresssink-predeleteitem)
		/// method.
	}

	fn_com_interface_userdef_event! { PostDeleteItem: Fn(co::TSF, &IShellItem, co::HRESULT, &IShellItem) -> AnyResult<()>;
		/// [`IFileOperationProgressSink::PostDeleteItem`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifileoperationprogresssink-postdeleteitem)
		/// method.
	}

	fn_com_interface_userdef_event! { PreNewItem: Fn(co::TSF, &IShellItem, &str) -> AnyResult<()>;
		/// [`IFileOperationProgressSink::PreNewItem`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifileoperationprogresssink-prenewitem)
		/// method.
	}

	fn_com_interface_userdef_event! { PostNewItem: Fn(co::TSF, &IShellItem, &str, &str, co::FILE_ATTRIBUTE, co::HRESULT, &IShellItem) -> AnyResult<()>;
		/// [`IFileOperationProgressSink::PostNewItem`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifileoperationprogresssink-postnewitem)
		/// method.
	}

	fn_com_interface_userdef_event! { UpdateProgress: Fn(u32, u32) -> AnyResult<()>;
		/// [`IFileOperationProgressSink::UpdateProgress`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifileoperationprogresssink-updateprogress)
		/// method.
	}

	fn_com_interface_userdef_event! { ResetTimer: Fn() -> AnyResult<()>;
		/// [`IFileOperationProgressSink::ResetTimer`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifileoperationprogresssink-resettimer)
		/// method.
	}

	fn_com_interface_userdef_event! { PauseTimer: Fn() -> AnyResult<()>;
		/// [`IFileOperationProgressSink::PauseTimer`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifileoperationprogresssink-pausetimer)
		/// method.
	}

	fn_com_interface_userdef_event! { ResumeTimer: Fn() -> AnyResult<()>;
		/// [`IFileOperationProgressSink::ResumeTimer`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifileoperationprogresssink-resumetimer)
		/// method.
	}
}

#[repr(C)]
struct IFileOperationProgressSinkImpl {
	vt: IFileOperationProgressSinkVT,
	counter: AtomicU32,
	StartOperations: Option<Box<dyn Fn() -> AnyResult<()>>>,
	FinishOperations: Option<Box<dyn Fn(co::HRESULT) -> AnyResult<()>>>,
	PreRenameItem: Option<Box<dyn Fn(co::TSF, &IShellItem, &str) -> AnyResult<()>>>,
	PostRenameItem:
		Option<Box<dyn Fn(co::TSF, &IShellItem, &str, co::HRESULT, &IShellItem) -> AnyResult<()>>>,
	PreMoveItem: Option<Box<dyn Fn(co::TSF, &IShellItem, &IShellItem, &str) -> AnyResult<()>>>,
	PostMoveItem: Option<
		Box<
			dyn Fn(
				co::TSF,
				&IShellItem,
				&IShellItem,
				&str,
				co::HRESULT,
				&IShellItem,
			) -> AnyResult<()>,
		>,
	>,
	PreCopyItem: Option<Box<dyn Fn(co::TSF, &IShellItem, &IShellItem, &str) -> AnyResult<()>>>,
	PostCopyItem: Option<
		Box<
			dyn Fn(
				co::TSF,
				&IShellItem,
				&IShellItem,
				&str,
				co::HRESULT,
				&IShellItem,
			) -> AnyResult<()>,
		>,
	>,
	PreDeleteItem: Option<Box<dyn Fn(co::TSF, &IShellItem) -> AnyResult<()>>>,
	PostDeleteItem:
		Option<Box<dyn Fn(co::TSF, &IShellItem, co::HRESULT, &IShellItem) -> AnyResult<()>>>,
	PreNewItem: Option<Box<dyn Fn(co::TSF, &IShellItem, &str) -> AnyResult<()>>>,
	PostNewItem: Option<
		Box<
			dyn Fn(
				co::TSF,
				&IShellItem,
				&str,
				&str,
				co::FILE_ATTRIBUTE,
				co::HRESULT,
				&IShellItem,
			) -> AnyResult<()>,
		>,
	>,
	UpdateProgress: Option<Box<dyn Fn(u32, u32) -> AnyResult<()>>>,
	ResetTimer: Option<Box<dyn Fn() -> AnyResult<()>>>,
	PauseTimer: Option<Box<dyn Fn() -> AnyResult<()>>>,
	ResumeTimer: Option<Box<dyn Fn() -> AnyResult<()>>>,
}

impl IFileOperationProgressSinkImpl {
	#[must_use]
	const fn new() -> Self {
		Self {
			vt: IFileOperationProgressSinkVT {
				IUnknownVT: IUnknownVT {
					QueryInterface: Self::QueryInterface,
					AddRef: Self::AddRef,
					Release: Self::Release,
				},
				StartOperations: Self::StartOperations,
				FinishOperations: Self::FinishOperations,
				PreRenameItem: Self::PreRenameItem,
				PostRenameItem: Self::PostRenameItem,
				PreMoveItem: Self::PreMoveItem,
				PostMoveItem: Self::PostMoveItem,
				PreCopyItem: Self::PreCopyItem,
				PostCopyItem: Self::PostCopyItem,
				PreDeleteItem: Self::PreDeleteItem,
				PostDeleteItem: Self::PostDeleteItem,
				PreNewItem: Self::PreNewItem,
				PostNewItem: Self::PostNewItem,
				UpdateProgress: Self::UpdateProgress,
				ResetTimer: Self::ResetTimer,
				PauseTimer: Self::PauseTimer,
				ResumeTimer: Self::ResumeTimer,
			},
			counter: AtomicU32::new(1),
			StartOperations: None,
			FinishOperations: None,
			PreRenameItem: None,
			PostRenameItem: None,
			PreMoveItem: None,
			PostMoveItem: None,
			PreCopyItem: None,
			PostCopyItem: None,
			PreDeleteItem: None,
			PostDeleteItem: None,
			PreNewItem: None,
			PostNewItem: None,
			UpdateProgress: None,
			ResetTimer: None,
			PauseTimer: None,
			ResumeTimer: None,
		}
	}

	fn_com_interface_userdef_iunknown_impls!(Self);

	fn_com_interface_userdef_impl_noparm!(StartOperations);

	fn FinishOperations(p: COMPTR, hrResult: HRES) -> HRES {
		let box_impl = box_impl_of::<Self>(p);
		hrresult_to_hres(match &box_impl.FinishOperations {
			Some(func) => {
				let hr = unsafe { co::HRESULT::from_raw(hrResult) };
				anyresult_to_hresult(func(hr))
			},
			None => Ok(()),
		})
	}

	fn PreRenameItem(p: COMPTR, dwFlags: u32, psiItem: COMPTR, pszNewName: PCSTR) -> HRES {
		let box_impl = box_impl_of::<Self>(p);
		hrresult_to_hres(match &box_impl.PreRenameItem {
			Some(func) => {
				let flags = unsafe { co::TSF::from_raw(dwFlags) };
				let item = ManuallyDrop::new(unsafe { IShellItem::from_ptr(psiItem) });
				let new_name = unsafe { WString::from_wchars_nullt(pszNewName) }.to_string();
				anyresult_to_hresult(func(flags, &item, &new_name))
			},
			None => Ok(()),
		})
	}

	fn PostRenameItem(
		p: COMPTR,
		dwFlags: u32,
		psiItem: COMPTR,
		pszNewName: PCSTR,
		hrRename: HRES,
		psiNewlyCreated: COMPTR,
	) -> HRES {
		let box_impl = box_impl_of::<Self>(p);
		hrresult_to_hres(match &box_impl.PostRenameItem {
			Some(func) => {
				let flags = unsafe { co::TSF::from_raw(dwFlags) };
				let item = ManuallyDrop::new(unsafe { IShellItem::from_ptr(psiItem) });
				let new_name = unsafe { WString::from_wchars_nullt(pszNewName) }.to_string();
				let hr = unsafe { co::HRESULT::from_raw(hrRename) };
				let new_item = ManuallyDrop::new(unsafe { IShellItem::from_ptr(psiNewlyCreated) });
				anyresult_to_hresult(func(flags, &item, &new_name, hr, &new_item))
			},
			None => Ok(()),
		})
	}

	fn PreMoveItem(
		p: COMPTR,
		dwFlags: u32,
		psiItem: COMPTR,
		psiDestinationFolder: COMPTR,
		pszNewName: PCSTR,
	) -> HRES {
		let box_impl = box_impl_of::<Self>(p);
		hrresult_to_hres(match &box_impl.PreMoveItem {
			Some(func) => {
				let flags = unsafe { co::TSF::from_raw(dwFlags) };
				let item = ManuallyDrop::new(unsafe { IShellItem::from_ptr(psiItem) });
				let dest_folder =
					ManuallyDrop::new(unsafe { IShellItem::from_ptr(psiDestinationFolder) });
				let new_name = unsafe { WString::from_wchars_nullt(pszNewName) }.to_string();
				anyresult_to_hresult(func(flags, &item, &dest_folder, &new_name))
			},
			None => Ok(()),
		})
	}

	fn PostMoveItem(
		p: COMPTR,
		dwFlags: u32,
		psiItem: COMPTR,
		psiDestinationFolder: COMPTR,
		pszNewName: PCSTR,
		hrMove: HRES,
		psiNewlyCreated: COMPTR,
	) -> HRES {
		let box_impl = box_impl_of::<Self>(p);
		hrresult_to_hres(match &box_impl.PostMoveItem {
			Some(func) => {
				let flags = unsafe { co::TSF::from_raw(dwFlags) };
				let item = ManuallyDrop::new(unsafe { IShellItem::from_ptr(psiItem) });
				let dest_folder =
					ManuallyDrop::new(unsafe { IShellItem::from_ptr(psiDestinationFolder) });
				let new_name = unsafe { WString::from_wchars_nullt(pszNewName) }.to_string();
				let hr = unsafe { co::HRESULT::from_raw(hrMove) };
				let new_item = ManuallyDrop::new(unsafe { IShellItem::from_ptr(psiNewlyCreated) });
				anyresult_to_hresult(func(flags, &item, &dest_folder, &new_name, hr, &new_item))
			},
			None => Ok(()),
		})
	}

	fn PreCopyItem(
		p: COMPTR,
		dwFlags: u32,
		psiItem: COMPTR,
		psiDestinationFolder: COMPTR,
		pszNewName: PCSTR,
	) -> HRES {
		let box_impl = box_impl_of::<Self>(p);
		hrresult_to_hres(match &box_impl.PreCopyItem {
			Some(func) => {
				let flags = unsafe { co::TSF::from_raw(dwFlags) };
				let item = ManuallyDrop::new(unsafe { IShellItem::from_ptr(psiItem) });
				let dest_folder =
					ManuallyDrop::new(unsafe { IShellItem::from_ptr(psiDestinationFolder) });
				let new_name = unsafe { WString::from_wchars_nullt(pszNewName) }.to_string();
				anyresult_to_hresult(func(flags, &item, &dest_folder, &new_name))
			},
			None => Ok(()),
		})
	}

	fn PostCopyItem(
		p: COMPTR,
		dwFlags: u32,
		psiItem: COMPTR,
		psiDestinationFolder: COMPTR,
		pszNewName: PCSTR,
		hrCopy: HRES,
		psiNewlyCreated: COMPTR,
	) -> HRES {
		let box_impl = box_impl_of::<Self>(p);
		hrresult_to_hres(match &box_impl.PostCopyItem {
			Some(func) => {
				let flags = unsafe { co::TSF::from_raw(dwFlags) };
				let item = ManuallyDrop::new(unsafe { IShellItem::from_ptr(psiItem) });
				let dest_folder =
					ManuallyDrop::new(unsafe { IShellItem::from_ptr(psiDestinationFolder) });
				let new_name = unsafe { WString::from_wchars_nullt(pszNewName) }.to_string();
				let hr = unsafe { co::HRESULT::from_raw(hrCopy) };
				let new_item = ManuallyDrop::new(unsafe { IShellItem::from_ptr(psiNewlyCreated) });
				anyresult_to_hresult(func(flags, &item, &dest_folder, &new_name, hr, &new_item))
			},
			None => Ok(()),
		})
	}

	fn PreDeleteItem(p: COMPTR, dwFlags: u32, psiItem: COMPTR) -> HRES {
		let box_impl = box_impl_of::<Self>(p);
		hrresult_to_hres(match &box_impl.PreDeleteItem {
			Some(func) => {
				let flags = unsafe { co::TSF::from_raw(dwFlags) };
				let item = ManuallyDrop::new(unsafe { IShellItem::from_ptr(psiItem) });
				anyresult_to_hresult(func(flags, &item))
			},
			None => Ok(()),
		})
	}

	fn PostDeleteItem(
		p: COMPTR,
		dwFlags: u32,
		psiItem: COMPTR,
		hrDelete: HRES,
		psiNewlyCreated: COMPTR,
	) -> HRES {
		let box_impl = box_impl_of::<Self>(p);
		hrresult_to_hres(match &box_impl.PostDeleteItem {
			Some(func) => {
				let flags = unsafe { co::TSF::from_raw(dwFlags) };
				let item = ManuallyDrop::new(unsafe { IShellItem::from_ptr(psiItem) });
				let hr = unsafe { co::HRESULT::from_raw(hrDelete) };
				let new_item = ManuallyDrop::new(unsafe { IShellItem::from_ptr(psiNewlyCreated) });
				anyresult_to_hresult(func(flags, &item, hr, &new_item))
			},
			None => Ok(()),
		})
	}

	fn PreNewItem(
		p: COMPTR,
		dwFlags: u32,
		psiDestinationFolder: COMPTR,
		pszNewName: PCSTR,
	) -> HRES {
		let box_impl = box_impl_of::<Self>(p);
		hrresult_to_hres(match &box_impl.PreNewItem {
			Some(func) => {
				let flags = unsafe { co::TSF::from_raw(dwFlags) };
				let dest_folder =
					ManuallyDrop::new(unsafe { IShellItem::from_ptr(psiDestinationFolder) });
				let new_name = unsafe { WString::from_wchars_nullt(pszNewName) }.to_string();
				anyresult_to_hresult(func(flags, &dest_folder, &new_name))
			},
			None => Ok(()),
		})
	}

	fn PostNewItem(
		p: COMPTR,
		dwFlags: u32,
		psiDestinationFolder: COMPTR,
		pszNewName: PCSTR,
		pszTemplateName: PCSTR,
		dwFileAttributes: u32,
		hrNew: HRES,
		psiNewItem: COMPTR,
	) -> HRES {
		let box_impl = box_impl_of::<Self>(p);
		hrresult_to_hres(match &box_impl.PostNewItem {
			Some(func) => {
				let flags = unsafe { co::TSF::from_raw(dwFlags) };
				let dest_folder =
					ManuallyDrop::new(unsafe { IShellItem::from_ptr(psiDestinationFolder) });
				let new_name = unsafe { WString::from_wchars_nullt(pszNewName) }.to_string();
				let template_name =
					unsafe { WString::from_wchars_nullt(pszTemplateName) }.to_string();
				let file_attr = unsafe { co::FILE_ATTRIBUTE::from_raw(dwFileAttributes) };
				let hr = unsafe { co::HRESULT::from_raw(hrNew) };
				let new_item = ManuallyDrop::new(unsafe { IShellItem::from_ptr(psiNewItem) });
				anyresult_to_hresult(func(
					flags,
					&dest_folder,
					&new_name,
					&template_name,
					file_attr,
					hr,
					&new_item,
				))
			},
			None => Ok(()),
		})
	}

	fn UpdateProgress(p: COMPTR, iWorkTotal: u32, iWorkSoFar: u32) -> HRES {
		let box_impl = box_impl_of::<Self>(p);
		hrresult_to_hres(match &box_impl.UpdateProgress {
			Some(func) => anyresult_to_hresult(func(iWorkTotal, iWorkSoFar)),
			None => Ok(()),
		})
	}

	fn_com_interface_userdef_impl_noparm!(ResetTimer);

	fn_com_interface_userdef_impl_noparm!(PauseTimer);

	fn_com_interface_userdef_impl_noparm!(ResumeTimer);
}
