#![allow(non_camel_case_types, non_snake_case)]

use crate::{co, comctl_ole};
use crate::comctl::decl::IdTdiconStr;
use crate::kernel::decl::{HINSTANCE, WString};
use crate::ole::decl::HrResult;
use crate::ole::privs::ok_to_hrresult;
use crate::prelude::Handle;
use crate::user::decl::HWND;

impl comctl_ole_Hwnd for HWND {}

/// This trait is enabled with `comctl` and `ole` features, and provides methods
/// for [`HWND`](crate::HWND).
///
/// Prefer importing this trait through the prelude:
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// ```
#[cfg_attr(docsrs, doc(cfg(all(feature = "comctl", feature = "ole"))))]
pub trait comctl_ole_Hwnd: Handle {
	/// [`InitializeFlatSB`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-initializeflatsb)
	/// method.
	fn InitializeFlatSB(self) -> HrResult<()> {
		ok_to_hrresult(
			unsafe { comctl_ole::ffi::InitializeFlatSB(self.as_ptr()) },
		)
	}

	/// [`TaskDialog`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-taskdialog)
	/// method.
	///
	/// Unless you need something specific, consider using the
	/// [`task_dlg`](crate::task_dlg) high-level abstractions.
	///
	/// If you need more customization, see the
	/// [`TaskDialogIndirect`](crate::TaskDialogIndirect) function.
	///
	/// # Examples
	///
	/// An information message with just an OK button:
	///
	/// ```rust,no_run
	/// use winsafe::prelude::*;
	/// use winsafe::{co, HWND, IdTdiconStr};
	///
	/// let hwnd: HWND; // initialized somewhere
	/// # let hwnd = HWND::NULL;
	///
	/// hwnd.TaskDialog(
	///     None,
	///     Some("My app name"),
	///     Some("Operation successful"),
	///     Some("The operation completed successfully."),
	///     co::TDCBF::OK,
	///     IdTdiconStr::Tdicon(co::TD_ICON::INFORMATION),
	/// )?;
	/// # Ok::<_, co::HRESULT>(())
	/// ```
	///
	/// Prompt the user to click OK or Cancel upon a question:
	///
	/// ```rust,no_run
	/// use winsafe::prelude::*;
	/// use winsafe::{co, HWND, IdTdiconStr};
	///
	/// let hwnd: HWND; // initialized somewhere
	/// # let hwnd = HWND::NULL;
	///
	/// let answer = hwnd.TaskDialog(
	///     None,
	///     Some("My app name"),
	///     Some("File modified"),
	///     Some("The file has been modified.\nProceed closing the application?"),
	///     co::TDCBF::OK | co::TDCBF::CANCEL,
	///     IdTdiconStr::Tdicon(co::TD_ICON::WARNING),
	/// )?;
	///
	/// if answer == co::DLGID::OK {
	///     println!("User clicked OK.");
	/// }
	/// # Ok::<_, co::HRESULT>(())
	/// ```
	fn TaskDialog(self,
		hinstance: Option<HINSTANCE>,
		window_title: Option<&str>,
		main_instruction: Option<&str>,
		content: Option<&str>,
		common_buttons: co::TDCBF,
		icon: IdTdiconStr) -> HrResult<co::DLGID>
	{
		// https://weblogs.asp.net/kennykerr/Windows-Vista-for-Developers-_1320_-Part-2-_1320_-Task-Dialogs-in-Depth
		let mut pn_button = i32::default();
		ok_to_hrresult(
			unsafe {
				comctl_ole::ffi::TaskDialog(
					self.as_ptr(),
					hinstance.map_or(std::ptr::null_mut(), |h| h.0),
					WString::from_opt_str(window_title).as_ptr(),
					WString::from_opt_str(main_instruction).as_ptr(),
					WString::from_opt_str(content).as_ptr(),
					common_buttons.0,
					icon.as_ptr(),
					&mut pn_button,
				)
			},
		).map(|_| co::DLGID(pn_button as _))
	}

	/// [`UninitializeFlatSB`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-uninitializeflatsb)
	/// method.
	fn UninitializeFlatSB(self) -> HrResult<()> {
		ok_to_hrresult(
			unsafe { comctl_ole::ffi::UninitializeFlatSB(self.as_ptr()) },
		)
	}
}
