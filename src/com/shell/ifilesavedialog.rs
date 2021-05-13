#![allow(non_snake_case)]

macro_rules! pub_struct_IFileSaveDialog {
	(
		$(#[$doc:meta])*
		$name:ident, $vt:ty
	) => {
		use crate::com::shell::vt::IFileSaveDialogVT;

		pub_struct_IFileDialog! {
			$(#[$doc])*
			$name, $vt
		}

		impl $name {
			fn ifilesavedialog_vt(&self) -> &IFileSaveDialogVT {
				unsafe { &**(self.ppvt as PPComVT<_>) }
			}

			/// [`IFileSaveDialog::SetSaveAsItem`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifilesavedialog-setsaveasitem)
			/// method.
			pub fn SetSaveAsItem(&self, psi: IShellItem) -> WinResult<()> {
				hr_to_winresult(
					(self.ifilesavedialog_vt().SetSaveAsItem)(self.ppvt, psi.ppvt),
				)
			}
		}
	};
}

pub_struct_IFileSaveDialog! {
	/// [`IFileSaveDialog`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nn-shobjidl_core-ifilesavedialog)
	/// COM interface over
	/// [`IFileSaveDialogVT`](crate::shell::vt::IFileSaveDialogVT). Inherits
	/// from [`IFileDialog`](crate::shell::IFileDialog),
	/// [`IModalWindow`](crate::shell::IModalWindow),
	/// [`IUnknown`](crate::IUnknown).
	///
	/// Automatically calls
	/// [`IUnknown::Release`](https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
	///
	/// # Examples
	///
	/// ```rust,ignore
	/// use winsafe::{co, CoCreateInstance, shell};
	///
	/// let obj: shell::IFileSaveDialog = CoCreateInstance(
	///     &shell::clsid::FileSaveDialog,
	///     None,
	///     co::CLSCTX::INPROC_SERVER,
	/// ).unwrap();
	/// ```
	IFileSaveDialog, crate::com::shell::vt::IFileSaveDialogVT
}
