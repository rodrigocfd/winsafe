#![allow(non_camel_case_types, non_snake_case)]

use crate::co;
use crate::decl::*;
use crate::ole::privs::*;
use crate::prelude::*;
use crate::taskschd::vts::*;

com_interface! { ITaskFolder: "8cfac062-a080-4c15-9a88-aa7c2af80dfc";
	/// [`ITaskFolder`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nn-taskschd-itaskfolder)
	/// COM interface.
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
/// ```no_run
/// use winsafe::prelude::*;
/// ```
pub trait taskschd_ITaskFolder: oleaut_IDispatch {
	/// [`ITaskFolder::DeleteTask`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-itaskfolder-deletetask)
	/// method.
	fn DeleteTask(&self, name: &str) -> HrResult<()> {
		ok_to_hrresult(unsafe {
			(vt::<ITaskFolderVT>(self).DeleteTask)(
				self.ptr(),
				BSTR::SysAllocString(name)?.as_ptr(),
				0,
			)
		})
	}

	fn_com_bstr_get! { get_Name: ITaskFolderVT;
		/// [`ITaskFolder::get_Name`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-itaskfolder-get_name)
		/// method.
	}

	fn_com_bstr_get! { get_Path: ITaskFolderVT;
		/// [`ITaskFolder::get_Path`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-itaskfolder-get_path)
		/// method.
	}

	/// [`ITaskFolder::RegisterTaskDefinition`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-itaskfolder-registertaskdefinition)
	/// method.
	fn RegisterTaskDefinition(
		&self,
		path: Option<&str>,
		definition: &impl taskschd_ITaskDefinition,
		flags: co::TASK_CREATION,
		user_id: Option<&str>,
		password: Option<&str>,
		logon_type: co::TASK_LOGON,
		sddl: Option<VARIANT>,
	) -> HrResult<IRegisteredTask> {
		let mut queried = unsafe { IRegisteredTask::null() };
		ok_to_hrresult(unsafe {
			(vt::<ITaskFolderVT>(self).RegisterTaskDefinition)(
				self.ptr(),
				BSTR::SysAllocString(path.unwrap_or_default())?.as_ptr(),
				definition.ptr(),
				flags.raw() as _,
				Variant::from_opt_str(user_id).to_raw()?,
				Variant::from_opt_str(password).to_raw()?,
				logon_type.raw(),
				sddl.unwrap_or_default(),
				queried.as_mut(),
			)
		})
		.map(|_| queried)
	}
}
