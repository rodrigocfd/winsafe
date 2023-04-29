#![allow(non_camel_case_types, non_snake_case)]

use crate::kernel::ffi_types::{HRES, PCSTR, PSTR};
use crate::ole::decl::{ComPtr, HrResult};
use crate::ole::privs::ok_to_hrresult;
use crate::oleaut::decl::{BSTR, VARIANT};
use crate::prelude::{oleaut_IDispatch, oleaut_Variant};
use crate::taskschd::decl::{ITaskFolder, ITaskDefinition};
use crate::vt::IDispatchVT;

/// [`ITaskService`](crate::ITaskService) virtual table.
#[repr(C)]
pub struct ITaskServiceVT {
	pub IDispatchVT: IDispatchVT,
	pub GetFolder: fn(ComPtr, PCSTR, *mut ComPtr) -> HRES,
	pub GetRunningTasks: fn(ComPtr, i32, *mut ComPtr) -> HRES,
	pub NewTask: fn(ComPtr, u32, *mut ComPtr) -> HRES,
	pub Connect: fn(ComPtr, VARIANT, VARIANT, VARIANT, VARIANT) -> HRES,
	pub get_Connected: fn(ComPtr, *mut i16) -> HRES,
	pub get_TargetServer: fn(ComPtr, *mut PSTR) -> HRES,
	pub get_ConnectedUser: fn(ComPtr, *mut PSTR) -> HRES,
	pub get_ConnectedDomain: fn(ComPtr, *mut PSTR) -> HRES,
	pub get_HighestVersion: fn(ComPtr, *mut u32) -> HRES,
}

com_interface! { ITaskService: "2faba4c7-4da9-4013-9697-20cc3fd40f85";
	/// [`ITaskService`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nn-taskschd-itaskservice)
	/// COM interface over [`ITaskServiceVT`](crate::vt::ITaskServiceVT).
	///
	/// Automatically calls
	/// [`Release`](https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
	///
	/// # Examples
	///
	/// ```rust,no_run
	/// use winsafe::prelude::*;
	/// use winsafe::{co, CoCreateInstance, ITaskService};
	///
	/// let obj = CoCreateInstance::<ITaskService>(
	///     &co::CLSID::TaskScheduler,
	///     None,
	///     co::CLSCTX::INPROC_SERVER,
	/// )?;
	/// # Ok::<_, co::HRESULT>(())
	/// ```
}

impl oleaut_IDispatch for ITaskService {}
impl taskschd_ITaskService for ITaskService {}

/// This trait is enabled with the `taskschd` feature, and provides methods for
/// [`ITaskService`](crate::ITaskService).
///
/// Prefer importing this trait through the prelude:
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// ```
pub trait taskschd_ITaskService: oleaut_IDispatch {
	/// [`ITaskService::Connect`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-itaskservice-connect)
	/// method.
	fn Connect(&self,
		server_name: Option<&str>,
		user: Option<&str>,
		domain: Option<&str>,
		password: Option<&str>,
	) -> HrResult<()>
	{
		unsafe {
			let vt = self.vt_ref::<ITaskServiceVT>();
			ok_to_hrresult(
				(vt.Connect)(
					self.ptr(),
					match server_name {
						Some(server_name) => VARIANT::new_bstr(server_name)?,
						None => VARIANT::default(),
					},
					match user {
						Some(user) => VARIANT::new_bstr(user)?,
						None => VARIANT::default(),
					},
					match domain {
						Some(domain) => VARIANT::new_bstr(domain)?,
						None => VARIANT::default(),
					},
					match password {
						Some(password) => VARIANT::new_bstr(password)?,
						None => VARIANT::default(),
					},
				)
			)
		}
	}

	/// [`ITaskService::get_Connected`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-itaskservice-get_connected)
	/// method.
	#[must_use]
	fn get_Connected(&self) -> HrResult<bool> {
		let mut connected = i16::default();
		unsafe {
			let vt = self.vt_ref::<ITaskServiceVT>();
			ok_to_hrresult((vt.get_Connected)(self.ptr(), &mut connected))
		}.map(|_| connected != 0)
	}

	fn_bstr_get! { get_ConnectedDomain: ITaskServiceVT;
		/// [`ITaskService::get_ConnectedDomain`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-itaskservice-get_connecteddomain)
		/// method.
	}

	fn_bstr_get! { get_ConnectedUser: ITaskServiceVT;
		/// [`ITaskService::get_ConnectedUser`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-itaskservice-get_connecteduser)
		/// method.
	}

	/// [`ITaskService::get_HighestVersion`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-itaskservice-get_highestversion)
	/// method.
	#[must_use]
	fn get_HighestVersion(&self) -> HrResult<u32> {
		let mut ver = u32::default();
		unsafe {
			let vt = self.vt_ref::<ITaskServiceVT>();
			ok_to_hrresult((vt.get_HighestVersion)(self.ptr(), &mut ver))
		}.map(|_| ver)
	}

	fn_bstr_get! { get_TargetServer: ITaskServiceVT;
		/// [`ITaskService::get_TargetServer`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-itaskservice-get_targetserver)
		/// method.
	}

	/// [`ITaskService::GetFolder`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-itaskservice-getfolder)
	/// method.
	#[must_use]
	fn GetFolder(&self, path: &str) -> HrResult<ITaskFolder> {
		unsafe {
			let mut ppv_queried = ComPtr::null();
			let vt = self.vt_ref::<ITaskServiceVT>();
			ok_to_hrresult(
				(vt.GetFolder)(
					self.ptr(),
					BSTR::SysAllocString(path)?.as_ptr(),
					&mut ppv_queried,
				),
			).map(|_| ITaskFolder::from(ppv_queried))
		}
	}

	/// [`ITaskService::NewTask`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-itaskservice-newtask)
	/// method.
	#[must_use]
	fn NewTask(&self) -> HrResult<ITaskDefinition> {
		unsafe {
			let mut ppv_queried = ComPtr::null();
			let vt = self.vt_ref::<ITaskServiceVT>();
			ok_to_hrresult((vt.NewTask)(self.ptr(), 0, &mut ppv_queried))
				.map(|_| ITaskDefinition::from(ppv_queried))
		}
	}
}
