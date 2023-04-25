#![allow(non_camel_case_types, non_snake_case)]

use crate::kernel::ffi_types::{HRES, PCSTR, PSTR};
use crate::ole::decl::{ComPtr, HrResult};
use crate::ole::privs::ok_to_hrresult;
use crate::oleaut::decl::BSTR;
use crate::prelude::oleaut_IDispatch;
use crate::vt::IDispatchVT;

/// [`ITaskDefinition`](crate::ITaskDefinition) virtual table.
#[repr(C)]
pub struct ITaskDefinitionVT {
	pub IDispatchVT: IDispatchVT,
	pub get_RegistrationInfo: fn(ComPtr, *mut ComPtr) -> HRES,
	pub put_RegistrationInfo: fn(ComPtr, ComPtr) -> HRES,
	pub get_Triggers: fn(ComPtr, *mut ComPtr) -> HRES,
	pub put_Triggers: fn(ComPtr, ComPtr) -> HRES,
	pub get_Settings: fn(ComPtr, *mut ComPtr) -> HRES,
	pub put_Settings: fn(ComPtr, ComPtr) -> HRES,
	pub get_Data: fn(ComPtr, *mut PSTR) -> HRES,
	pub put_Data: fn(ComPtr, PCSTR) -> HRES,
	pub get_Principal: fn(ComPtr, *mut ComPtr) -> HRES,
	pub put_Principal: fn(ComPtr, ComPtr) -> HRES,
	pub get_Actions: fn(ComPtr, *mut ComPtr) -> HRES,
	pub put_Actions: fn(ComPtr, ComPtr) -> HRES,
	pub get_XmlText: fn(ComPtr, *mut PSTR) -> HRES,
	pub put_XmlText: fn(ComPtr, PCSTR) -> HRES,
}

com_interface! { ITaskDefinition: "f5bc8fc5-536d-4f77-b852-fbc1356fdeb6";
	/// [`ITaskDefinition`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nn-taskschd-itaskdefinition)
	/// COM interface over [`ITaskDefinitionVT`](crate::vt::ITaskDefinitionVT).
	/// 
	/// Automatically calls
	/// [`Release`](https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
}

impl oleaut_IDispatch for ITaskDefinition {}
impl taskschd_ITaskDefinition for ITaskDefinition {}

/// This trait is enabled with the `taskschd` feature, and provides methods for
/// [`ITaskDefinition`](crate::ITaskDefinition).
///
/// Prefer importing this trait through the prelude:
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// ```
pub trait taskschd_ITaskDefinition: oleaut_IDispatch {
	/// [`ITaskDefinition::get_Data`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-itaskdefinition-get_data)
	/// method.
	#[must_use]
	fn get_Data(&self) -> HrResult<String> {
		let mut pstr = std::ptr::null_mut::<u16>();
		unsafe {
			let vt = self.vt_ref::<ITaskDefinitionVT>();
			ok_to_hrresult((vt.get_Data)(self.ptr(), &mut pstr))
		}.map(|_| {
			let bstr = unsafe { BSTR::from_ptr(pstr) };
			bstr.to_string()
		})
	}

	/// [`ITaskDefinition::get_XmlText`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-itaskdefinition-get_xmltext)
	/// method.
	#[must_use]
	fn get_XmlText(&self) -> HrResult<String> {
		let mut pstr = std::ptr::null_mut::<u16>();
		unsafe {
			let vt = self.vt_ref::<ITaskDefinitionVT>();
			ok_to_hrresult((vt.get_XmlText)(self.ptr(), &mut pstr))
		}.map(|_| {
			let bstr = unsafe { BSTR::from_ptr(pstr) };
			bstr.to_string()
		})
	}

	/// [`ITaskDefinition::put_Data`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-itaskdefinition-put_data)
	/// method.
	fn put_Data(&self, xml: &str) -> HrResult<()> {
		unsafe {
			let vt = self.vt_ref::<ITaskDefinitionVT>();
			ok_to_hrresult(
				(vt.put_Data)(self.ptr(), BSTR::SysAllocString(xml)?.as_ptr()),
			)
		}
	}

	/// [`ITaskDefinition::put_XmlText`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-itaskdefinition-put_xmltext)
	/// method.
	fn put_XmlText(&self, xml: &str) -> HrResult<()> {
		unsafe {
			let vt = self.vt_ref::<ITaskDefinitionVT>();
			ok_to_hrresult(
				(vt.put_XmlText)(self.ptr(), BSTR::SysAllocString(xml)?.as_ptr()),
			)
		}
	}
}
