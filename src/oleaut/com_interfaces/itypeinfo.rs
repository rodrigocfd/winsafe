#![allow(non_camel_case_types, non_snake_case)]

use crate::co;
use crate::decl::*;
use crate::kernel::privs::*;
use crate::ole::privs::*;
use crate::oleaut::vts::*;
use crate::prelude::*;

com_interface! { ITypeInfo: "00020401-0000-0000-c000-000000000046";
	/// [`ITypeInfo`](https://learn.microsoft.com/en-us/windows/win32/api/oaidl/nn-oaidl-itypeinfo)
	/// COM interface.
	///
	/// Automatically calls
	/// [`Release`](https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
}

impl oleaut_ITypeInfo for ITypeInfo {}

/// This trait is enabled with the `oleaut` feature, and provides methods for
/// [`ITypeInfo`](crate::ITypeInfo).
///
/// Prefer importing this trait through the prelude:
///
/// ```no_run
/// use winsafe::prelude::*;
/// ```
pub trait oleaut_ITypeInfo: ole_IUnknown {
	/// [`ITypeInfo::AddressOfMember`](https://learn.microsoft.com/en-us/windows/win32/api/oaidl/nf-oaidl-itypeinfo-addressofmember)
	/// method.
	#[must_use]
	fn AddressOfMember(
		&self,
		member_id: i32,
		inv_kind: co::INVOKEKIND,
	) -> HrResult<*mut std::ffi::c_void> {
		let mut addr: *mut std::ffi::c_void = std::ptr::null_mut();
		ok_to_hrresult(unsafe {
			(vt::<ITypeInfoVT>(self).AddressOfMember)(
				self.ptr(),
				member_id,
				inv_kind.raw(),
				&mut addr,
			)
		})
		.map(|_| addr)
	}

	/// [`ITypeInfo::CreateInstance`](https://learn.microsoft.com/en-us/windows/win32/api/oaidl/nf-oaidl-itypeinfo-createinstance)
	/// method.
	#[must_use]
	fn CreateInstance<T>(&self, iunk_outer: Option<&impl ole_IUnknown>) -> HrResult<T>
	where
		T: ole_IUnknown,
	{
		let mut queried = unsafe { T::null() };
		ok_to_hrresult(unsafe {
			(vt::<ITypeInfoVT>(self).CreateInstance)(
				self.ptr(),
				iunk_outer.map_or(std::ptr::null_mut(), |uo| uo.ptr()),
				pcvoid(&T::IID),
				queried.as_mut(),
			)
		})
		.map(|_| queried)
	}

	/// [`ITypeInfo::GetDllEntry`](https://learn.microsoft.com/en-us/windows/win32/api/oaidl/nf-oaidl-itypeinfo-getdllentry)
	/// method.
	///
	/// Returns:
	/// * DLL name;
	/// * entry point name;
	/// * ordinal.
	#[must_use]
	fn GetDllEntry(
		&self,
		member_id: i32,
		inv_kind: co::INVOKEKIND,
	) -> HrResult<(String, String, u16)> {
		let (mut dll_name, mut name) = (BSTR::default(), BSTR::default());
		let mut ordinal = 0u16;

		ok_to_hrresult(unsafe {
			(vt::<ITypeInfoVT>(self).GetDllEntry)(
				self.ptr(),
				member_id,
				inv_kind.raw(),
				dll_name.as_mut_ptr(),
				name.as_mut_ptr(),
				&mut ordinal,
			)
		})
		.map(|_| {
			let dll_name_str = dll_name.to_string();
			let name_str = if name.as_ptr().is_null() { String::new() } else { name.to_string() };
			(dll_name_str, name_str, ordinal)
		})
	}

	/// [`ITypeInfo::GetDocumentation`](https://learn.microsoft.com/en-us/windows/win32/api/oaidl/nf-oaidl-itypeinfo-getdocumentation)
	/// method.
	///
	/// Returns:
	/// * item name;
	/// * documentation;
	/// * help localization context;
	/// * fully qualified name of the file containing the DLL used for help file.
	#[must_use]
	fn GetDocumentation(&self, member_id: i32) -> HrResult<(String, String, u32, String)> {
		let mut name = BSTR::default();
		let mut doc = BSTR::default();
		let mut context = 0u32;
		let mut help_file = BSTR::default();

		ok_to_hrresult(unsafe {
			(vt::<ITypeInfoVT>(self).GetDocumentation)(
				self.ptr(),
				member_id,
				name.as_mut_ptr(),
				doc.as_mut_ptr(),
				&mut context,
				help_file.as_mut_ptr(),
			)
		})
		.map(|_| (name.to_string(), doc.to_string(), context, help_file.to_string()))
	}

	/// [`ITypeInfo::GetIDsOfNames`](https://learn.microsoft.com/en-us/windows/win32/api/oaidl/nf-oaidl-itypeinfo-getidsofnames)
	/// method.
	#[must_use]
	fn GetIDsOfNames(&self, names: &[impl AsRef<str>]) -> HrResult<Vec<i32>> {
		let (_wstrs, pwstrs) = create_wstr_ptr_vecs(names);
		let mut ids = vec![0i32; names.len()];

		ok_to_hrresult(unsafe {
			(vt::<ITypeInfoVT>(self).GetIDsOfNames)(
				self.ptr(),
				vec_ptr(&pwstrs),
				names.len() as _,
				ids.as_mut_ptr() as _,
			)
		})
		.map(|_| ids)
	}
}
