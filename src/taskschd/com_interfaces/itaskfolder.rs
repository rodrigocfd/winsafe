#![allow(non_camel_case_types, non_snake_case)]

use crate::kernel::decl::WString;
use crate::kernel::ffi_types::{HRES, PCSTR, PSTR};
use crate::ole::decl::{ComPtr, HrResult};
use crate::ole::privs::ok_to_hrresult;
use crate::oleaut::decl::{BSTR, VARIANT};
use crate::prelude::oleaut_IDispatch;
use crate::vt::IDispatchVT;

/// [`ITaskFolder`](crate::ITaskFolder) virtual table.
#[repr(C)]
pub struct ITaskFolderVT {
	pub IDispatchVT: IDispatchVT,
	pub get_Name: fn(ComPtr, *mut PSTR) -> HRES,
	pub get_Path: fn(ComPtr, *mut PSTR) -> HRES,
	pub GetFolder: fn(ComPtr, PCSTR, *mut ComPtr) -> HRES,
	pub GetFolders: fn(ComPtr, i32, *mut ComPtr) -> HRES,
	pub CreateFolder: fn(ComPtr, PCSTR, VARIANT, *mut ComPtr) -> HRES,
	pub DeleteFolder: fn(ComPtr, PCSTR, i32) -> HRES,
	pub GetTask: fn(ComPtr, PCSTR, *mut ComPtr) -> HRES,
	pub GetTasks: fn(ComPtr, i32, *mut ComPtr) -> HRES,
	pub DeleteTask: fn(ComPtr, PCSTR, i32) -> HRES,
	pub RegisterTask: fn(ComPtr, PCSTR, PCSTR, i32, VARIANT, VARIANT, u32, VARIANT, *mut ComPtr) -> HRES,
	pub RegisterTaskDefinition: fn(ComPtr, PCSTR, ComPtr, i32, VARIANT, VARIANT, u32, VARIANT, *mut ComPtr) -> HRES,
	pub GetSecurityDescriptor: fn(ComPtr, i32, *mut PSTR) -> HRES,
	pub SetSecurityDescriptor: fn(ComPtr, PCSTR, i32) -> HRES,
}

com_interface! { ITaskFolder: "8cfac062-a080-4c15-9a88-aa7c2af80dfc";
	/// [`ITaskFolder`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nn-taskschd-itaskfolder)
	/// COM interface over [`ITaskFolderVT`](crate::vt::ITaskFolderVT).
	/// 
	/// Automatically calls
	/// [`Release`](https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
}

impl oleaut_IDispatch for ITaskFolder {}
impl taskschd_ITaskFolder for ITaskFolder {}

/// This trait is enabled with the `taskschd` feature, and provides methods for
/// [`ITaskFolder`](crate::ITaskFolder).
///
/// Prefer importing this trait through the prelude:
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// ```
pub trait taskschd_ITaskFolder: oleaut_IDispatch {
	/// [`ITaskFolder::DeleteTask`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-itaskfolder-deletetask)
	/// method.
	fn DeleteTask(&self, name: &str) -> HrResult<()> {
		unsafe {
			let vt = self.vt_ref::<ITaskFolderVT>();
			ok_to_hrresult(
				(vt.DeleteTask)(
					self.ptr(),
					WString::from_str(name).as_ptr(),
					0,
				),
			)
		}
	}

	/// [`ITaskFolder::get_Name`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-itaskfolder-get_name)
	/// method.
	#[must_use]
	fn get_Name(&self) -> HrResult<String> {
		let mut pstr = std::ptr::null_mut::<u16>();
		unsafe {
			let vt = self.vt_ref::<ITaskFolderVT>();
			ok_to_hrresult((vt.get_Name)(self.ptr(), &mut pstr))
		}.map(|_| {
			let bstr = unsafe { BSTR::from_ptr(pstr) };
			bstr.to_string()
		})
	}

	/// [`ITaskFolder::get_Path`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-itaskfolder-get_path)
	/// method.
	#[must_use]
	fn get_Path(&self) -> HrResult<String> {
		let mut pstr = std::ptr::null_mut::<u16>();
		unsafe {
			let vt = self.vt_ref::<ITaskFolderVT>();
			ok_to_hrresult((vt.get_Path)(self.ptr(), &mut pstr))
		}.map(|_| {
			let bstr = unsafe { BSTR::from_ptr(pstr) };
			bstr.to_string()
		})
	}
}
