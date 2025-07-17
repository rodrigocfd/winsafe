#![allow(non_camel_case_types, non_snake_case)]

use crate::advapi::ffi;
use crate::co;
use crate::decl::*;
use crate::guard::*;
use crate::kernel::privs::*;

handle! { HSC;
	/// Handle to a
	/// [Service Control Manager](https://learn.microsoft.com/en-us/windows/win32/api/winsvc/nf-winsvc-openscmanagerw).
	/// Originally `SC_HANDLE`.
}

impl HSC {
	/// [`CreateService`](https://learn.microsoft.com/en-us/windows/win32/api/winsvc/nf-winsvc-createservicew)
	/// function.
	#[must_use]
	pub fn CreateService(
		&self,
		service_name: &str,
		display_name: Option<&str>,
		desired_access: co::SERVICE,
		service_type: co::SERVICE_TYPE,
		start_type: co::SERVICE_START,
		error_control: co::SERVICE_ERROR,
		binary_path_name: Option<&str>,
		load_order_group: Option<&str>,
		tag_id: Option<&mut u32>,
		dependencies: &[impl AsRef<str>],
		service_start_name: Option<&str>,
		password: Option<&str>,
	) -> SysResult<CloseServiceHandleSvcGuard> {
		let binary_path_name_quoted = binary_path_name
			.map(|s| if s.starts_with('"') { s.to_owned() } else { format!("\"{}\"", s) });

		unsafe {
			PtrRet(ffi::CreateServiceW(
				self.ptr(),
				WString::from_str(service_name).as_ptr(),
				WString::from_opt_str(display_name).as_ptr(),
				desired_access.raw(),
				service_type.raw(),
				start_type.raw(),
				error_control.raw(),
				WString::from_opt_str(binary_path_name_quoted).as_ptr(),
				WString::from_opt_str(load_order_group).as_ptr(),
				tag_id.map_or(std::ptr::null_mut(), |n| n),
				WString::from_str_vec(dependencies).as_ptr(),
				WString::from_opt_str(service_start_name).as_ptr(),
				WString::from_opt_str(password).as_ptr(),
			))
			.to_sysresult_handle()
			.map(|h| CloseServiceHandleSvcGuard::new(h))
		}
	}

	/// [`OpenSCManager`](https://learn.microsoft.com/en-us/windows/win32/api/winsvc/nf-winsvc-openscmanagerw)
	/// function.
	#[must_use]
	pub fn OpenSCManager(
		machine_name: Option<&str>,
		desired_access: co::SC_MANAGER,
	) -> SysResult<CloseServiceHandleGuard> {
		unsafe {
			PtrRet(ffi::OpenSCManagerW(
				WString::from_opt_str(machine_name).as_ptr(),
				std::ptr::null(),
				desired_access.raw(),
			))
			.to_sysresult_handle()
			.map(|h| CloseServiceHandleGuard::new(h))
		}
	}

	/// [`OpenService`](https://learn.microsoft.com/en-us/windows/win32/api/winsvc/nf-winsvc-openservicew)
	/// function.
	#[must_use]
	pub fn OpenService(
		&self,
		service_name: &str,
		desired_access: co::SERVICE,
	) -> SysResult<CloseServiceHandleSvcGuard> {
		unsafe {
			PtrRet(ffi::OpenServiceW(
				self.ptr(),
				WString::from_str(service_name).as_ptr(),
				desired_access.raw(),
			))
			.to_sysresult_handle()
			.map(|h| CloseServiceHandleSvcGuard::new(h))
		}
	}
}
