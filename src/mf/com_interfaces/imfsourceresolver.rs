#![allow(non_camel_case_types, non_snake_case)]

use crate::co;
use crate::decl::*;
use crate::mf::vts::*;
use crate::ole::privs::*;
use crate::prelude::*;

com_interface! { IMFSourceResolver: "fbe5a32d-a497-4b61-bb85-97b1a848a6e3";
	/// [`IMFSourceResolver`](https://learn.microsoft.com/en-us/windows/win32/api/mfidl/nn-mfidl-imfsourceresolver)
	/// COM interface.
	///
	/// Automatically calls
	/// [`Release`](https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
	///
	/// Usually created with
	/// [`MFCreateSourceResolver`](crate::MFCreateSourceResolver) function.
	///
	/// # Examples
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*};
	///
	/// let source_resolver = w::MFCreateSourceResolver()?;
	/// # w::HrResult::Ok(())
	/// ```
}

impl mf_IMFSourceResolver for IMFSourceResolver {}

/// This trait is enabled with the `mf` feature, and provides methods for
/// [`IMFSourceResolver`](crate::IMFSourceResolver).
///
/// Prefer importing this trait through the prelude:
///
/// ```no_run
/// use winsafe::prelude::*;
/// ```
pub trait mf_IMFSourceResolver: ole_IUnknown {
	/// [`IMFSourceResolver::BeginCreateObjectFromByteStream`](https://learn.microsoft.com/en-us/windows/win32/api/mfidl/nf-mfidl-imfsourceresolver-begincreateobjectfrombytestream)
	/// method.
	///
	/// Returns the cancel cookie.
	fn BeginCreateObjectFromByteStream(&self,
		byte_stream: &impl mf_IMFByteStream,
		url: Option<&str>,
		flags: co::MF_RESOLUTION,
		props: Option<&impl oleaut_IPropertyStore>,
		callback: &IMFAsyncCallback,
		state: Option<&impl ole_IUnknown>,
	) -> HrResult<IUnknown>
	{
		let mut queried = unsafe { IUnknown::null() };
		ok_to_hrresult(
			unsafe {
				(vt::<IMFSourceResolverVT>(self).BeginCreateObjectFromByteStream)(
					self.ptr(),
					byte_stream.ptr(),
					WString::from_opt_str(url).as_ptr(),
					flags.raw(),
					props.map_or(std::ptr::null_mut(), |p| p.ptr()),
					queried.as_mut(),
					callback.ptr(),
					state.map_or(std::ptr::null_mut(), |s| s.ptr()),
				)
			},
		).map(|_| queried)
	}

	/// [`IMFSourceResolver::BeginCreateObjectFromURL`](https://learn.microsoft.com/en-us/windows/win32/api/mfidl/nf-mfidl-imfsourceresolver-begincreateobjectfromurl)
	/// method.
	///
	/// Returns the cancel cookie.
	fn BeginCreateObjectFromURL(&self,
		url: &str,
		flags: co::MF_RESOLUTION,
		props: Option<&impl oleaut_IPropertyStore>,
		callback: &IMFAsyncCallback,
		state: Option<&impl ole_IUnknown>,
	) -> HrResult<IUnknown>
	{
		let mut queried = unsafe { IUnknown::null() };
		ok_to_hrresult(
			unsafe {
				(vt::<IMFSourceResolverVT>(self).BeginCreateObjectFromURL)(
					self.ptr(),
					WString::from_str(url).as_ptr(),
					flags.raw(),
					props.map_or(std::ptr::null_mut(), |p| p.ptr()),
					queried.as_mut(),
					callback.ptr(),
					state.map_or(std::ptr::null_mut(), |s| s.ptr()),
				)
			},
		).map(|_| queried)
	}

	/// [`IMFSourceResolver::CancelObjectCreation`](https://learn.microsoft.com/en-us/windows/win32/api/mfidl/nf-mfidl-imfsourceresolver-cancelobjectcreation)
	/// method.
	fn CancelObjectCreation(&self, cookie: &impl ole_IUnknown) -> HrResult<()> {
		ok_to_hrresult(
			unsafe {
				(vt::<IMFSourceResolverVT>(self).CancelObjectCreation)(
					self.ptr(),
					cookie.ptr(),
				)
			},
		)
	}

	/// [`IMFSourceResolver::CreateObjectFromByteStream`](https://learn.microsoft.com/en-us/windows/win32/api/mfidl/nf-mfidl-imfsourceresolver-createobjectfrombytestream)
	/// method.
	fn CreateObjectFromByteStream(&self,
		byte_stream: &impl mf_IMFByteStream,
		url: Option<&str>,
		flags: co::MF_RESOLUTION,
		props: Option<&impl oleaut_IPropertyStore>,
	) -> HrResult<(co::MF_OBJECT, IUnknown)>
	{
		let mut obj_type = co::MF_OBJECT::default();
		let mut queried = unsafe { IUnknown::null() };

		ok_to_hrresult(
			unsafe {
				(vt::<IMFSourceResolverVT>(self).CreateObjectFromByteStream)(
					self.ptr(),
					byte_stream.ptr(),
					WString::from_opt_str(url).as_ptr(),
					flags.raw(),
					props.map_or(std::ptr::null_mut(), |p| p.ptr()),
					obj_type.as_mut(),
					queried.as_mut(),
				)
			},
		).map(|_| (obj_type, queried))
	}

	/// [`IMFSourceResolver::CreateObjectFromURL`](https://learn.microsoft.com/en-us/windows/win32/api/mfidl/nf-mfidl-imfsourceresolver-createobjectfromurl)
	/// method.
	fn CreateObjectFromURL(&self,
		url: &str,
		flags: co::MF_RESOLUTION,
		props: Option<&impl oleaut_IPropertyStore>,
	) -> HrResult<(co::MF_OBJECT, IUnknown)>
	{
		let mut obj_type = co::MF_OBJECT::default();
		let mut queried = unsafe { IUnknown::null() };

		ok_to_hrresult(
			unsafe {
				(vt::<IMFSourceResolverVT>(self).CreateObjectFromURL)(
					self.ptr(),
					WString::from_str(url).as_ptr(),
					flags.raw(),
					props.map_or(std::ptr::null_mut(), |p| p.ptr()),
					obj_type.as_mut(),
					queried.as_mut(),
				)
			},
		).map(|_| (obj_type, queried))
	}

	/// [`IMFSourceResolver::EndCreateObjectFromByteStream`](https://learn.microsoft.com/en-us/windows/win32/api/mfidl/nf-mfidl-imfsourceresolver-endcreateobjectfrombytestream)
	/// method.
	fn EndCreateObjectFromByteStream(&self,
		result: &impl mf_IMFAsyncResult,
	) -> HrResult<(co::MF_OBJECT, IUnknown)>
	{
		let mut obj_type = co::MF_OBJECT::default();
		let mut queried = unsafe { IUnknown::null() };

		ok_to_hrresult(
			unsafe {
				(vt::<IMFSourceResolverVT>(self).EndCreateObjectFromByteStream)(
					self.ptr(),
					result.ptr(),
					obj_type.as_mut(),
					queried.as_mut(),
				)
			},
		).map(|_| (obj_type, queried))
	}

	/// [`IMFSourceResolver::EndCreateObjectFromURL`](https://learn.microsoft.com/en-us/windows/win32/api/mfidl/nf-mfidl-imfsourceresolver-endcreateobjectfromurl)
	/// method.
	fn EndCreateObjectFromURL(&self,
		result: &impl mf_IMFAsyncResult,
	) -> HrResult<(co::MF_OBJECT, IUnknown)>
	{
		let mut obj_type = co::MF_OBJECT::default();
		let mut queried = unsafe { IUnknown::null() };

		ok_to_hrresult(
			unsafe {
				(vt::<IMFSourceResolverVT>(self).EndCreateObjectFromURL)(
					self.ptr(),
					result.ptr(),
					obj_type.as_mut(),
					queried.as_mut(),
				)
			},
		).map(|_| (obj_type, queried))
	}
}
