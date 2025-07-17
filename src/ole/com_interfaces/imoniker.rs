#![allow(non_camel_case_types, non_snake_case)]

use crate::co;
use crate::decl::*;
use crate::kernel::privs::*;
use crate::ole::{privs::*, vts::*};
use crate::prelude::*;

com_interface! { IMoniker: "0000000f-0000-0000-c000-000000000046";
	/// [`IMoniker`](https://learn.microsoft.com/en-us/windows/win32/api/objidl/nn-objidl-imoniker)
	/// COM interface.
	///
	/// Automatically calls
	/// [`Release`](https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
}

impl ole_IPersist for IMoniker {}
impl ole_IPersistStream for IMoniker {}
impl ole_IMoniker for IMoniker {}

/// This trait is enabled with the `ole` feature, and provides methods for
/// [`IMoniker`](crate::IMoniker).
///
/// Prefer importing this trait through the prelude:
///
/// ```no_run
/// use winsafe::prelude::*;
/// ```
pub trait ole_IMoniker: ole_IPersistStream {
	/// [`IMoniker::BindToObject`](https://learn.microsoft.com/en-us/windows/win32/api/objidl/nf-objidl-imoniker-bindtoobject)
	/// method.
	#[must_use]
	fn BindToObject<T>(
		&self,
		bind_ctx: &impl ole_IBindCtx,
		moniker_to_left: Option<&impl ole_IMoniker>,
	) -> HrResult<T>
	where
		T: ole_IUnknown,
	{
		let mut queried = unsafe { T::null() };
		HrRet(unsafe {
			(vt::<IMonikerVT>(self).BindToObject)(
				self.ptr(),
				bind_ctx.ptr(),
				moniker_to_left.map_or(std::ptr::null_mut(), |m| m.ptr()),
				pcvoid(&T::IID),
				queried.as_mut(),
			)
		})
		.to_hrresult()
		.map(|_| queried)
	}

	/// [`IMoniker::BindToStorage`](https://learn.microsoft.com/en-us/windows/win32/api/objidl/nf-objidl-imoniker-bindtostorage)
	/// method.
	#[must_use]
	fn BindToStorage<T>(
		&self,
		bind_ctx: &impl ole_IBindCtx,
		moniker_to_left: Option<&impl ole_IMoniker>,
	) -> HrResult<T>
	where
		T: ole_IUnknown,
	{
		let mut queried = unsafe { T::null() };
		HrRet(unsafe {
			(vt::<IMonikerVT>(self).BindToStorage)(
				self.ptr(),
				bind_ctx.ptr(),
				moniker_to_left.map_or(std::ptr::null_mut(), |m| m.ptr()),
				pcvoid(&T::IID),
				queried.as_mut(),
			)
		})
		.to_hrresult()
		.map(|_| queried)
	}

	/// [`IMoniker::CommonPrefixWith`](https://learn.microsoft.com/en-us/windows/win32/api/objidl/nf-objidl-imoniker-commonprefixwith)
	/// method.
	#[must_use]
	fn CommonPrefixWith(&self, other: &impl ole_IMoniker) -> HrResult<IMoniker> {
		let mut queried = unsafe { IMoniker::null() };
		HrRet(unsafe {
			(vt::<IMonikerVT>(self).CommonPrefixWith)(self.ptr(), other.ptr(), queried.as_mut())
		})
		.to_hrresult()
		.map(|_| queried)
	}

	/// [`IMoniker::ComposeWith`](https://learn.microsoft.com/en-us/windows/win32/api/objidl/nf-objidl-imoniker-composewith)
	/// method.
	#[must_use]
	fn ComposeWith(
		&self,
		moniker_to_right: &impl ole_IMoniker,
		only_if_not_generic: bool,
	) -> HrResult<IMoniker> {
		let mut queried = unsafe { IMoniker::null() };
		HrRet(unsafe {
			(vt::<IMonikerVT>(self).ComposeWith)(
				self.ptr(),
				moniker_to_right.ptr(),
				only_if_not_generic as _,
				queried.as_mut(),
			)
		})
		.to_hrresult()
		.map(|_| queried)
	}

	/// [`IMoniker::Enum`](https://learn.microsoft.com/en-us/windows/win32/api/objidl/nf-objidl-imoniker-enum)
	/// method.
	#[must_use]
	fn Enum(&self, forward: bool) -> HrResult<IMoniker> {
		let mut queried = unsafe { IMoniker::null() };
		HrRet(unsafe { (vt::<IMonikerVT>(self).Enum)(self.ptr(), forward as _, queried.as_mut()) })
			.to_hrresult()
			.map(|_| queried)
	}

	/// [`IMoniker::GetDisplayName`](https://learn.microsoft.com/en-us/windows/win32/api/objidl/nf-objidl-imoniker-getdisplayname)
	/// method.
	#[must_use]
	fn GetDisplayName(
		&self,
		bind_ctx: &impl ole_IBindCtx,
		moniker_to_left: Option<&impl ole_IMoniker>,
	) -> HrResult<String> {
		let mut pstr = std::ptr::null_mut::<u16>();
		HrRet(unsafe {
			(vt::<IMonikerVT>(self).GetDisplayName)(
				self.ptr(),
				bind_ctx.ptr(),
				moniker_to_left.map_or(std::ptr::null_mut(), |m| m.ptr()),
				&mut pstr,
			)
		})
		.to_hrresult()
		.map(|_| htaskmem_ptr_to_str(pstr))
	}

	/// [`IMoniker::GetTimeOfLastChange`](https://learn.microsoft.com/en-us/windows/win32/api/objidl/nf-objidl-imoniker-gettimeoflastchange)
	/// method.
	#[must_use]
	fn GetTimeOfLastChange(
		&self,
		bind_ctx: &impl ole_IBindCtx,
		moniker_to_left: Option<&impl ole_IMoniker>,
	) -> HrResult<FILETIME> {
		let mut ft = FILETIME::default();
		HrRet(unsafe {
			(vt::<IMonikerVT>(self).GetTimeOfLastChange)(
				self.ptr(),
				bind_ctx.ptr(),
				moniker_to_left.map_or(std::ptr::null_mut(), |m| m.ptr()),
				pvoid(&mut ft),
			)
		})
		.to_hrresult()
		.map(|_| ft)
	}

	/// [`IMoniker::Hash`](https://learn.microsoft.com/en-us/windows/win32/api/objidl/nf-objidl-imoniker-hash)
	/// method.
	#[must_use]
	fn Hash(&self) -> HrResult<u32> {
		let mut hash = 0u32;
		HrRet(unsafe { (vt::<IMonikerVT>(self).Hash)(self.ptr(), &mut hash) })
			.to_hrresult()
			.map(|_| hash)
	}

	fn_com_interface_get! { Inverse: IMonikerVT => IMoniker;
		/// [`IMoniker::Inverse`](https://learn.microsoft.com/en-us/windows/win32/api/objidl/nf-objidl-imoniker-inverse)
		/// method.
	}

	/// [`IMoniker::IsEqual`](https://learn.microsoft.com/en-us/windows/win32/api/objidl/nf-objidl-imoniker-isequal)
	/// method.
	#[must_use]
	fn IsEqual(&self, other_moniker: &impl ole_IMoniker) -> HrResult<bool> {
		HrRet(unsafe { (vt::<IMonikerVT>(self).IsEqual)(self.ptr(), other_moniker.ptr()) })
			.to_bool_hrresult()
	}

	/// [`IMoniker::IsRunning`](https://learn.microsoft.com/en-us/windows/win32/api/objidl/nf-objidl-imoniker-isrunning)
	/// method.
	#[must_use]
	fn IsRunning(
		&self,
		bind_ctx: &impl ole_IBindCtx,
		moniker_to_left: Option<&impl ole_IMoniker>,
		moniker_newly_running: Option<&impl ole_IMoniker>,
	) -> HrResult<bool> {
		HrRet(unsafe {
			(vt::<IMonikerVT>(self).IsRunning)(
				self.ptr(),
				bind_ctx.ptr(),
				moniker_to_left.map_or(std::ptr::null_mut(), |m| m.ptr()),
				moniker_newly_running.map_or(std::ptr::null_mut(), |m| m.ptr()),
			)
		})
		.to_bool_hrresult()
	}

	/// [`IMoniker::IsSystemMoniker](https://learn.microsoft.com/en-us/windows/win32/api/objidl/nf-objidl-imoniker-issystemmoniker)
	/// method.
	#[must_use]
	fn IsSystemMoniker(&self) -> HrResult<(bool, co::MKSYS)> {
		let mut mksys = co::MKSYS::default();
		HrRet(unsafe { (vt::<IMonikerVT>(self).IsSystemMoniker)(self.ptr(), mksys.as_mut()) })
			.to_bool_hrresult()
			.map(|b| (b, mksys))
	}

	/// [`IMoniker::ParseDisplayName`](https://learn.microsoft.com/en-us/windows/win32/api/objidl/nf-objidl-imoniker-parsedisplayname)
	/// method.
	#[must_use]
	fn ParseDisplayName(
		&self,
		bind_ctx: &impl ole_IBindCtx,
		moniker_to_left: &impl ole_IMoniker,
		display_name: &str,
	) -> HrResult<(u32, IMoniker)> {
		let mut ch_eaten = 0u32;
		let mut queried = unsafe { IMoniker::null() };

		HrRet(unsafe {
			(vt::<IMonikerVT>(self).ParseDisplayName)(
				self.ptr(),
				bind_ctx.ptr(),
				moniker_to_left.ptr(),
				WString::from_str(display_name).as_ptr(),
				&mut ch_eaten,
				queried.as_mut(),
			)
		})
		.to_hrresult()
		.map(|_| (ch_eaten, queried))
	}

	/// [`IMoniker::Reduce`](https://learn.microsoft.com/en-us/windows/win32/api/objidl/nf-objidl-imoniker-reduce)
	/// method.
	///
	/// Returns the moniker to the left, and the reduced moniker, respectively.
	#[must_use]
	fn Reduce(
		&self,
		bind_ctx: &impl ole_IBindCtx,
		reduce_how_far: co::MKRREDUCE,
	) -> HrResult<(IMoniker, IMoniker)> {
		let (mut queried, mut queried2) = unsafe { (IMoniker::null(), IMoniker::null()) };

		HrRet(unsafe {
			(vt::<IMonikerVT>(self).Reduce)(
				self.ptr(),
				bind_ctx.ptr(),
				reduce_how_far.raw(),
				queried.as_mut(),
				queried2.as_mut(),
			)
		})
		.to_hrresult()
		.map(|_| (queried, queried2))
	}

	/// [`IMoniker::RelativePathTo`](https://learn.microsoft.com/en-us/windows/win32/api/objidl/nf-objidl-imoniker-relativepathto)
	/// method.
	#[must_use]
	fn RelativePathTo(&self, other_moniker: &impl ole_IMoniker) -> HrResult<IMoniker> {
		let mut queried = unsafe { IMoniker::null() };
		HrRet(unsafe {
			(vt::<IMonikerVT>(self).RelativePathTo)(
				self.ptr(),
				other_moniker.ptr(),
				queried.as_mut(),
			)
		})
		.to_hrresult()
		.map(|_| queried)
	}
}
