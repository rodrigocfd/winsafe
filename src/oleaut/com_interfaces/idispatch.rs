#![allow(non_camel_case_types, non_snake_case)]

use crate::co;
use crate::decl::*;
use crate::kernel::privs::*;
use crate::ole::privs::*;
use crate::oleaut::vts::*;
use crate::prelude::*;

com_interface! { IDispatch: "00020400-0000-0000-c000-000000000046";
	/// [`IDispatch`](https://learn.microsoft.com/en-us/windows/win32/api/oaidl/nn-oaidl-idispatch)
	/// COM interface.
	///
	/// Automatically calls
	/// [`Release`](https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
}

impl oleaut_IDispatch for IDispatch {}

/// This trait is enabled with the `oleaut` feature, and provides methods for
/// [`IDispatch`](crate::IDispatch).
///
/// Prefer importing this trait through the prelude:
///
/// ```no_run
/// use winsafe::prelude::*;
/// ```
pub trait oleaut_IDispatch: ole_IUnknown {
	/// [`IDispatch::GetIDsOfNames`](https://learn.microsoft.com/en-us/windows/win32/api/oaidl/nf-oaidl-idispatch-getidsofnames)
	/// method.
	#[must_use]
	fn GetIDsOfNames(&self, names: &[impl AsRef<str>], lcid: LCID) -> HrResult<Vec<i32>> {
		let (_wstrs, pwstrs) = create_wstr_ptr_vecs(names);
		let mut ids = vec![0i32; names.len()];

		HrRet(unsafe {
			(vt::<IDispatchVT>(self).GetIDsOfNames)(
				self.ptr(),
				pcvoid(&co::IID::default()),
				vec_ptr(&pwstrs),
				names.len() as _,
				lcid.into(),
				ids.as_mut_ptr() as _,
			)
		})
		.to_hrresult()
		.map(|_| ids)
	}

	/// [`IDispatch::GetTypeInfoCount`](https://learn.microsoft.com/en-us/windows/win32/api/oaidl/nf-oaidl-idispatch-gettypeinfocount)
	/// method.
	#[must_use]
	fn GetTypeInfoCount(&self) -> HrResult<u32> {
		let mut count = 0u32;
		HrRet(unsafe { (vt::<IDispatchVT>(self).GetTypeInfoCount)(self.ptr(), &mut count) })
			.to_hrresult()
			.map(|_| count)
	}

	/// [`IDispatch::GetTypeInfo`](https://learn.microsoft.com/en-us/windows/win32/api/oaidl/nf-oaidl-idispatch-gettypeinfo)
	/// method.
	#[must_use]
	fn GetTypeInfo(&self, info_type: u32, lcid: LCID) -> HrResult<ITypeInfo> {
		let mut queried = unsafe { ITypeInfo::null() };
		HrRet(unsafe {
			(vt::<IDispatchVT>(self).GetTypeInfo)(
				self.ptr(),
				info_type,
				lcid.into(),
				queried.as_mut(),
			)
		})
		.to_hrresult()
		.map(|_| queried)
	}

	/// [`IDispatch::Invoke`](https://learn.microsoft.com/en-us/windows/win32/api/oaidl/nf-oaidl-idispatch-invoke)
	/// method.
	///
	/// If the remote call raises an exception, the returned error will be an
	/// [`EXCEPINFO`](crate::EXCEPINFO).
	///
	/// This is a low-level method, prefer using:
	/// * [`IDispatch::invoke_get`](crate::prelude::oleaut_IDispatch::invoke_get);
	/// * [`IDispatch::invoke_method`](crate::prelude::oleaut_IDispatch::invoke_method); or
	/// * [`IDispatch::invoke_put`](crate::prelude::oleaut_IDispatch::invoke_put).
	fn Invoke(
		&self,
		disp_id_member: i32,
		lcid: LCID,
		flags: co::DISPATCH,
		disp_params: &mut DISPPARAMS,
	) -> AnyResult<VARIANT> {
		let mut remote_res = VARIANT::default();
		let mut remote_err = EXCEPINFO::default();
		let mut arg_err = 0u32;

		match HrRet(unsafe {
			(vt::<IDispatchVT>(self).Invoke)(
				self.ptr(),
				disp_id_member,
				pcvoid(&GUID::NULL),
				lcid.raw(),
				flags.raw(),
				pvoid(disp_params),
				pvoid(&mut remote_res),
				pvoid(&mut remote_err),
				&mut arg_err,
			)
		})
		.to_hrresult()
		{
			Ok(_) => Ok(remote_res),
			Err(hr) => match hr {
				co::HRESULT::DISP_E_EXCEPTION => Err(Box::new(remote_err)),
				_ => Err(hr.into()),
			},
		}
	}

	/// Calls
	/// [`IDispatch::GetIDsOfNames`](crate::prelude::oleaut_IDispatch::GetIDsOfNames)
	/// and [`IDispatch::Invoke`](crate::prelude::oleaut_IDispatch::Invoke) with
	/// [`co::DISPATCH::PROPERTYGET`](co::DISPATCH::PROPERTYGET).
	///
	/// If the remote call raises an exception, the returned error will be an
	/// [`EXCEPINFO`](crate::EXCEPINFO).
	///
	/// # Examples
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*, co};
	///
	/// let _com_guard = w::CoInitializeEx(
	///     co::COINIT::APARTMENTTHREADED | co::COINIT::DISABLE_OLE1DDE)?;
	///
	/// let excel = w::CoCreateInstance::<w::IDispatch>(
	///     &w::CLSIDFromProgID("Excel.Application")?,
	///     None::<&w::IUnknown>,
	///     co::CLSCTX::LOCAL_SERVER,
	/// )?;
	///
	/// let books = excel.invoke_get("Workbooks", &[])?.unwrap_dispatch();
	///
	/// let workbook = books
	///     .invoke_method("Open", &[&w::Variant::from_str("C:\\Temp\\bar.xlsx")])?
	///     .unwrap_dispatch();
	///
	/// let worksheets = workbook.invoke_get("Worksheets", &[])?.unwrap_dispatch();
	///
	/// let sheet1 = worksheets
	///     .invoke_get("Item", &[&w::Variant::from_str("Sheet1")])?
	///     .unwrap_dispatch();
	/// # w::AnyResult::Ok(())
	/// ```
	fn invoke_get(&self, property_name: &str, params: &[&Variant]) -> AnyResult<Variant> {
		let member_ids = self.GetIDsOfNames(&[property_name], LCID::USER_DEFAULT)?;

		let mut vars = params
			.iter()
			.rev() // in reverse order
			.map(|param| param.to_raw())
			.collect::<HrResult<Vec<_>>>()?;

		let mut dp = DISPPARAMS::default();
		dp.set_rvarg(Some(&mut vars));

		let vari =
			self.Invoke(member_ids[0], LCID::USER_DEFAULT, co::DISPATCH::PROPERTYGET, &mut dp)?;
		Variant::from_raw(&vari).map_err(|err| err.into())
	}

	/// Calls
	/// [`IDispatch::GetIDsOfNames`](crate::prelude::oleaut_IDispatch::GetIDsOfNames)
	/// and [`IDispatch::Invoke`](crate::prelude::oleaut_IDispatch::Invoke) with
	/// [`co::DISPATCH::METHOD`](co::DISPATCH::METHOD).
	///
	/// If the remote call raises an exception, the returned error will be an
	/// [`EXCEPINFO`](crate::EXCEPINFO).
	///
	/// # Examples
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*, co};
	///
	/// let _com_guard = w::CoInitializeEx(
	///     co::COINIT::APARTMENTTHREADED | co::COINIT::DISABLE_OLE1DDE)?;
	///
	/// let excel = w::CoCreateInstance::<w::IDispatch>(
	///     &w::CLSIDFromProgID("Excel.Application")?,
	///     None::<&w::IUnknown>,
	///     co::CLSCTX::LOCAL_SERVER,
	/// )?;
	///
	/// let books = excel.invoke_get("Workbooks", &[])?
	///     .unwrap_dispatch();
	///
	/// let file = books.invoke_method(
	///     "Open",
	///     &[&w::Variant::from_str("C:\\Temp\\foo.xlsx")],
	/// )?
	/// .unwrap_dispatch();
	///
	/// file.invoke_method(
	///     "SaveAs",
	///     &[&w::Variant::from_str("C:\\Temp\\bar.xlsx")],
	/// )?;
	///
	/// file.invoke_method("Close", &[])?;
	/// # w::AnyResult::Ok(())
	/// ```
	fn invoke_method(&self, method_name: &str, params: &[&Variant]) -> AnyResult<Variant> {
		let member_ids = self.GetIDsOfNames(&[method_name], LCID::USER_DEFAULT)?;

		let mut vars = params
			.iter()
			.rev() // in reverse order
			.map(|param| param.to_raw())
			.collect::<HrResult<Vec<_>>>()?;

		let mut dp = DISPPARAMS::default();
		dp.set_rvarg(Some(&mut vars));

		let vari = self.Invoke(member_ids[0], LCID::USER_DEFAULT, co::DISPATCH::METHOD, &mut dp)?;
		Variant::from_raw(&vari).map_err(|err| err.into())
	}

	/// Calls
	/// [`IDispatch::GetIDsOfNames`](crate::prelude::oleaut_IDispatch::GetIDsOfNames)
	/// and [`IDispatch::Invoke`](crate::prelude::oleaut_IDispatch::Invoke) with
	/// [`co::DISPATCH::PROPERTYPUT`](co::DISPATCH::PROPERTYPUT).
	///
	/// If the remote call raises an exception, the returned error will be an
	/// [`EXCEPINFO`](crate::EXCEPINFO).
	fn invoke_put(&self, property_name: &str, param: &Variant) -> AnyResult<Variant> {
		let member_ids = self.GetIDsOfNames(&[property_name], LCID::USER_DEFAULT)?;

		let mut vars = vec![param.to_raw()?]; // single parameter
		let mut named_args = vec![co::DISPID::PROPERTYPUT];

		let mut dp = DISPPARAMS::default();
		dp.set_rvarg(Some(&mut vars));
		dp.set_rgdispidNamedArgs(Some(&mut named_args));

		let vari =
			self.Invoke(member_ids[0], LCID::USER_DEFAULT, co::DISPATCH::PROPERTYPUT, &mut dp)?;
		Variant::from_raw(&vari).map_err(|err| err.into())
	}
}
