#![allow(non_camel_case_types, non_snake_case)]

use crate::co;
use crate::comctl::ffi;
use crate::decl::*;
use crate::kernel::privs::*;
use crate::ole::privs::*;
use crate::prelude::*;

impl HWND {
	/// [`DefSubclassProc`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-defsubclassproc)
	/// function.
	///
	/// The return type is variable, being defined by the `RetType` associated
	/// type of the [`MsgSend`](crate::prelude::MsgSend) trait. That means each
	/// message can define its own return type.
	///
	/// # Safety
	///
	/// Messages manipulate pointers, copies and window states. Improper use may
	/// lead to undefined behavior.
	pub unsafe fn DefSubclassProc<M>(&self, msg: M) -> M::RetType
	where
		M: MsgSend,
	{
		let mut msg = msg;
		let wm_any = msg.as_generic_wm();
		unsafe {
			msg.isize_to_ret(ffi::DefSubclassProc(
				self.ptr(),
				wm_any.msg_id.raw(),
				wm_any.wparam,
				wm_any.lparam,
			))
		}
	}

	/// [`InitializeFlatSB`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-initializeflatsb)
	/// function.
	pub fn InitializeFlatSB(&self) -> HrResult<()> {
		ok_to_hrresult(unsafe { ffi::InitializeFlatSB(self.ptr()) })
	}

	/// [`RemoveWindowSubclass`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-removewindowsubclass)
	/// function.
	pub fn RemoveWindowSubclass(
		&self,
		subclass_func: SUBCLASSPROC,
		subclass_id: usize,
	) -> SysResult<()> {
		bool_to_sysresult(unsafe {
			ffi::RemoveWindowSubclass(self.ptr(), subclass_func as _, subclass_id)
		})
	}

	/// [`SetWindowSubclass`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-setwindowsubclass)
	/// function.
	///
	/// # Safety
	///
	/// You must provide a subclass procedure.
	pub unsafe fn SetWindowSubclass(
		&self,
		subclass_proc: SUBCLASSPROC,
		subclass_id: usize,
		ref_data: usize,
	) -> SysResult<()> {
		bool_to_sysresult(unsafe {
			ffi::SetWindowSubclass(self.ptr(), subclass_proc as _, subclass_id, ref_data)
		})
	}

	/// [`TaskDialog`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-taskdialog)
	/// function.
	///
	/// If you need more customization, see the
	/// [`TaskDialogIndirect`](crate::TaskDialogIndirect) function.
	///
	/// # Examples
	///
	/// An information message with just an OK button:
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*, co};
	///
	/// let hwnd: w::HWND; // initialized somewhere
	/// # let hwnd = w::HWND::NULL;
	///
	/// hwnd.TaskDialog(
	///     Some("Operation successful"),
	///     None,
	///     Some("The operation completed successfully."),
	///     co::TDCBF::OK,
	///     w::IconRes::Info,
	/// )?;
	/// # w::HrResult::Ok(())
	/// ```
	///
	/// Prompt the user to click OK or Cancel upon a question:
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*, co};
	///
	/// let hwnd: w::HWND; // initialized somewhere
	/// # let hwnd = w::HWND::NULL;
	///
	/// let answer = hwnd.TaskDialog(
	///     Some("My app name"),
	///     Some("File modified"),
	///     Some("The file has been modified.\nProceed closing the application?"),
	///     co::TDCBF::OK | co::TDCBF::CANCEL,
	///     w::IconRes::Warn,
	/// )?;
	///
	/// if answer == co::DLGID::OK {
	///     println!("User clicked OK.");
	/// }
	/// # w::HrResult::Ok(())
	/// ```
	pub fn TaskDialog(
		&self,
		window_title: Option<&str>,
		main_instruction: Option<&str>,
		content: Option<&str>,
		common_buttons: co::TDCBF,
		icon: IconRes,
	) -> HrResult<co::DLGID> {
		// https://weblogs.asp.net/kennykerr/Windows-Vista-for-Developers-_1320_-Part-2-_1320_-Task-Dialogs-in-Depth
		let mut pn_button = i32::default();
		let (hinst, raw_ico) = icon.as_ptr();

		ok_to_hrresult(unsafe {
			ffi::TaskDialog(
				self.ptr(),
				hinst.ptr(),
				WString::from_opt_str(window_title).as_ptr(),
				WString::from_opt_str(main_instruction).as_ptr(),
				WString::from_opt_str(content).as_ptr(),
				common_buttons.raw(),
				raw_ico,
				&mut pn_button,
			)
		})
		.map(|_| unsafe { co::DLGID::from_raw(pn_button as _) })
	}

	/// [`UninitializeFlatSB`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-uninitializeflatsb)
	/// function.
	pub fn UninitializeFlatSB(&self) -> HrResult<()> {
		ok_to_hrresult(unsafe { ffi::UninitializeFlatSB(self.ptr()) })
	}
}
