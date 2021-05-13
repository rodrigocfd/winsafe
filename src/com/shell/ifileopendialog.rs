#![allow(non_snake_case)]

macro_rules! pub_struct_IFileOpenDialog {
	(
		$(#[$doc:meta])*
		$name:ident, $vt:ty
	) => {
		use crate::com::shell::IShellItemArray;
		use crate::com::shell::vt::{IFileOpenDialogVT, IShellItemArrayVT};

		pub_struct_IFileDialog! {
			$(#[$doc])*
			$name, $vt
		}

		impl $name {
			fn ifileopendialog_vt(&self) -> &IFileOpenDialogVT {
				unsafe { &**(self.ppvt as PPComVT<_>) }
			}

			/// [`IFileOpenDialog::GetResults`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifileopendialog-getresults)
			/// method.
			pub fn GetResults(&self) -> WinResult<IShellItemArray> {
				let mut ppvQueried: PPComVT<IShellItemArrayVT> = std::ptr::null_mut();
				hr_to_winresult(
					(self.ifileopendialog_vt().GetResults)(
						self.ppvt,
						&mut ppvQueried as *mut _ as _,
					),
				).map(|_| IShellItemArray::from(ppvQueried))
			}

			/// [`IFileOpenDialog::GetSelectedItems`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifileopendialog-getselecteditems)
			/// method.
			pub fn GetSelectedItems(&self) -> WinResult<IShellItemArray> {
				let mut ppvQueried: PPComVT<IShellItemArrayVT> = std::ptr::null_mut();
				hr_to_winresult(
					(self.ifileopendialog_vt().GetSelectedItems)(
						self.ppvt,
						&mut ppvQueried as *mut _ as _,
					),
				).map(|_| IShellItemArray::from(ppvQueried))
			}
		}
	};
}

pub_struct_IFileOpenDialog! {
	/// [`IFileOpenDialog`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nn-shobjidl_core-ifileopendialog)
	/// COM interface over
	/// [`IFileOpenDialogVT`](crate::shell::vt::IFileOpenDialogVT). Inherits
	/// from [`IFileDialog`](crate::shell::IFileDialog),
	/// [`IModalWindow`](crate::shell::IModalWindow),
	/// [`IUnknown`](crate::IUnknown).
	///
	/// Automatically calls
	/// [`Release`](https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
	///
	/// # Examples
	///
	/// ```rust,ignore
	/// use winsafe::{co, CoCreateInstance, shell};
	///
	/// let obj: shell::IFileOpenDialog = CoCreateInstance(
	///     &shell::clsid::FileOpenDialog,
	///     None,
	///     co::CLSCTX::INPROC_SERVER,
	/// ).unwrap();
	/// ```
	IFileOpenDialog, crate::com::shell::vt::IFileOpenDialogVT
}
