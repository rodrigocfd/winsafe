#![allow(non_camel_case_types, non_snake_case)]

use crate::co;
use crate::kernel::decl::{FILETIME, WString};
use crate::kernel::ffi_types::{BOOL, HRES, PCSTR, PCVOID, PSTR, PVOID};
use crate::ole::decl::{ComPtr, CoTaskMemFree, HrResult};
use crate::ole::privs::{ok_to_hrresult, okfalse_to_hrresult};
use crate::prelude::{
	ole_IBindCtx, ole_IPersist, ole_IPersistStream, ole_IUnknown,
};
use crate::vt::IPersistStreamVT;

/// [`IMoniker`](crate::IMoniker) virtual table.
#[repr(C)]
pub struct IMonikerVT {
	pub IPersistStreamVT: IPersistStreamVT,
	pub BindToObject: fn(ComPtr, ComPtr, ComPtr, PCVOID, *mut ComPtr) -> HRES,
	pub BindToStorage: fn(ComPtr, ComPtr, ComPtr, PCVOID, *mut ComPtr) -> HRES,
	pub Reduce: fn(ComPtr, ComPtr, u32, *mut ComPtr, *mut ComPtr) -> HRES,
	pub ComposeWith: fn(ComPtr, ComPtr, BOOL, *mut ComPtr) -> HRES,
	pub Enum: fn(ComPtr, BOOL, *mut ComPtr) -> HRES,
	pub IsEqual: fn(ComPtr, ComPtr) -> HRES,
	pub Hash: fn(ComPtr, *mut u32) -> HRES,
	pub IsRunning: fn(ComPtr, ComPtr, ComPtr, ComPtr) -> HRES,
	pub GetTimeOfLastChange: fn(ComPtr, ComPtr, ComPtr, PVOID) -> HRES,
	pub Inverse: fn(ComPtr, *mut ComPtr) -> HRES,
	pub CommonPrefixWith: fn(ComPtr, ComPtr, *mut ComPtr) -> HRES,
	pub RelativePathTo: fn(ComPtr, ComPtr, *mut ComPtr) -> HRES,
	pub GetDisplayName: fn(ComPtr, ComPtr, ComPtr, *mut PSTR) -> HRES,
	pub ParseDisplayName: fn(ComPtr, ComPtr, ComPtr, PCSTR, *mut u32, *mut ComPtr) -> HRES,
	pub IsSystemMoniker: fn(ComPtr, *mut u32) -> HRES,
}

com_interface! { IMoniker: "0000000f-0000-0000-c000-000000000046";
	/// [`IMoniker`](https://learn.microsoft.com/en-us/windows/win32/api/objidl/nn-objidl-imoniker)
	/// COM interface over [`IMonikerVT`](crate::vt::IMonikerVT).
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
/// ```rust,no_run
/// use winsafe::prelude::*;
/// ```
pub trait ole_IMoniker: ole_IPersistStream {
	/// [`IMoniker::BindToObject`](https://learn.microsoft.com/en-us/windows/win32/api/objidl/nf-objidl-imoniker-bindtoobject)
	/// method.
	#[must_use]
	fn BindToObject<T>(&self,
		bind_ctx: &impl ole_IBindCtx,
		moniker_to_left: Option<&impl ole_IMoniker>,
	) -> HrResult<T>
		where T: ole_IUnknown,
	{
		unsafe {
			let mut ppv_queried = ComPtr::null();
			let vt = self.vt_ref::<IMonikerVT>();
			ok_to_hrresult(
				(vt.BindToObject)(
					self.ptr(),
					bind_ctx.ptr(),
					moniker_to_left.map_or(ComPtr::null(), |m| m.ptr()),
					&T::IID as *const _ as _,
					&mut ppv_queried,
				),
			).map(|_| T::from(ppv_queried))
		}
	}

	/// [`IMoniker::BindToStorage`](https://learn.microsoft.com/en-us/windows/win32/api/objidl/nf-objidl-imoniker-bindtostorage)
	/// method.
	#[must_use]
	fn BindToStorage<T>(&self,
		bind_ctx: &impl ole_IBindCtx,
		moniker_to_left: Option<&impl ole_IMoniker>,
	) -> HrResult<T>
		where T: ole_IUnknown,
	{
		unsafe {
			let mut ppv_queried = ComPtr::null();
			let vt = self.vt_ref::<IMonikerVT>();
			ok_to_hrresult(
				(vt.BindToStorage)(
					self.ptr(),
					bind_ctx.ptr(),
					moniker_to_left.map_or(ComPtr::null(), |m| m.ptr()),
					&T::IID as *const _ as _,
					&mut ppv_queried,
				),
			).map(|_| T::from(ppv_queried))
		}
	}

	/// [`IMoniker::CommonPrefixWith`](https://learn.microsoft.com/en-us/windows/win32/api/objidl/nf-objidl-imoniker-commonprefixwith)
	/// method.
	#[must_use]
	fn CommonPrefixWith(&self, other: &impl ole_IMoniker) -> HrResult<IMoniker> {
		unsafe {
			let mut ppv_queried = ComPtr::null();
			let vt = self.vt_ref::<IMonikerVT>();
			ok_to_hrresult(
				(vt.CommonPrefixWith)(self.ptr(), other.ptr(), &mut ppv_queried),
			).map(|_| IMoniker::from(ppv_queried))
		}
	}

	/// [`IMoniker::ComposeWith`](https://learn.microsoft.com/en-us/windows/win32/api/objidl/nf-objidl-imoniker-composewith)
	/// method.
	#[must_use]
	fn ComposeWith(&self,
		moniker_to_right: &impl ole_IMoniker,
		only_if_not_generic: bool,
	) -> HrResult<IMoniker>
	{
		unsafe {
			let mut ppv_queried = ComPtr::null();
			let vt = self.vt_ref::<IMonikerVT>();
			ok_to_hrresult(
				(vt.ComposeWith)(
					self.ptr(),
					moniker_to_right.ptr(),
					only_if_not_generic as _,
					&mut ppv_queried,
				),
			).map(|_| IMoniker::from(ppv_queried))
		}
	}

	/// [`IMoniker::Enum`](https://learn.microsoft.com/en-us/windows/win32/api/objidl/nf-objidl-imoniker-enum)
	/// method.
	#[must_use]
	fn Enum(&self, forward: bool) -> HrResult<IMoniker> {
		unsafe {
			let mut ppv_queried = ComPtr::null();
			let vt = self.vt_ref::<IMonikerVT>();
			ok_to_hrresult((vt.Enum)(self.ptr(), forward as _, &mut ppv_queried))
				.map(|_| IMoniker::from(ppv_queried))
		}
	}

	/// [`IMoniker::GetDisplayName`](https://learn.microsoft.com/en-us/windows/win32/api/objidl/nf-objidl-imoniker-getdisplayname)
	/// method.
	#[must_use]
	fn GetDisplayName(&self,
		bind_ctx: &impl ole_IBindCtx,
		moniker_to_left: Option<&impl ole_IMoniker>,
	) -> HrResult<String>
	{
		let mut pstr: *mut u16 = std::ptr::null_mut();
		unsafe {
			let vt = self.vt_ref::<IMonikerVT>();
			ok_to_hrresult(
				(vt.GetDisplayName)(
					self.ptr(),
					bind_ctx.ptr(),
					moniker_to_left.map_or(ComPtr::null(), |m| m.ptr()),
					&mut pstr,
				),
			).map(|_| {
				let name = WString::from_wchars_nullt(pstr);
				CoTaskMemFree(pstr as _); // https://stackoverflow.com/q/3079508/6923555
				name.to_string()
			})
		}
	}

	/// [`IMoniker::GetTimeOfLastChange`](https://learn.microsoft.com/en-us/windows/win32/api/objidl/nf-objidl-imoniker-gettimeoflastchange)
	/// method.
	#[must_use]
	fn GetTimeOfLastChange(&self,
		bind_ctx: &impl ole_IBindCtx,
		moniker_to_left: Option<&impl ole_IMoniker>,
	) -> HrResult<FILETIME>
	{
		let mut ft = FILETIME::default();
		unsafe {
			let vt = self.vt_ref::<IMonikerVT>();
			ok_to_hrresult(
				(vt.GetTimeOfLastChange)(
					self.ptr(),
					bind_ctx.ptr(),
					moniker_to_left.map_or(ComPtr::null(), |m| m.ptr()),
					&mut ft as *mut _ as _,
				),
			).map(|_| ft)
		}
	}

	/// [`IMoniker::Hash`](https://learn.microsoft.com/en-us/windows/win32/api/objidl/nf-objidl-imoniker-hash)
	/// method.
	#[must_use]
	fn Hash(&self) -> HrResult<u32> {
		let mut hash = u32::default();
		unsafe {
			let vt = self.vt_ref::<IMonikerVT>();
			ok_to_hrresult((vt.Hash)(self.ptr(), &mut hash))
				.map(|_| hash)
		}
	}

	/// [`IMoniker::Inverse`](https://learn.microsoft.com/en-us/windows/win32/api/objidl/nf-objidl-imoniker-inverse)
	/// method.
	#[must_use]
	fn Inverse(&self) -> HrResult<IMoniker> {
		unsafe {
			let mut ppv_queried = ComPtr::null();
			let vt = self.vt_ref::<IMonikerVT>();
			ok_to_hrresult((vt.Inverse)(self.ptr(), &mut ppv_queried))
				.map(|_| IMoniker::from(ppv_queried))
		}
	}

	/// [`IMoniker::IsEqual`](https://learn.microsoft.com/en-us/windows/win32/api/objidl/nf-objidl-imoniker-isequal)
	/// method.
	#[must_use]
	fn IsEqual(&self, other_moniker: &impl ole_IMoniker) -> HrResult<bool> {
		unsafe {
			let vt = self.vt_ref::<IMonikerVT>();
			okfalse_to_hrresult((vt.IsEqual)(self.ptr(), other_moniker.ptr()))
		}
	}

	/// [`IMoniker::IsRunning`](https://learn.microsoft.com/en-us/windows/win32/api/objidl/nf-objidl-imoniker-isrunning)
	/// method.
	#[must_use]
	fn IsRunning(&self,
		bind_ctx: &impl ole_IBindCtx,
		moniker_to_left: Option<&impl ole_IMoniker>,
		moniker_newly_running: Option<&impl ole_IMoniker>,
	) -> HrResult<bool>
	{
		unsafe {
			let vt = self.vt_ref::<IMonikerVT>();
			okfalse_to_hrresult(
				(vt.IsRunning)(
					self.ptr(),
					bind_ctx.ptr(),
					moniker_to_left.map_or(ComPtr::null(), |m| m.ptr()),
					moniker_newly_running.map_or(ComPtr::null(), |m| m.ptr()),
				),
			)
		}
	}

	/// [`IMoniker::IsSystemMoniker](https://learn.microsoft.com/en-us/windows/win32/api/objidl/nf-objidl-imoniker-issystemmoniker)
	/// method.
	#[must_use]
	fn IsSystemMoniker(&self) -> HrResult<(bool, co::MKSYS)> {
		let mut mksys = co::MKSYS::NONE;
		unsafe {
			let vt = self.vt_ref::<IMonikerVT>();
			okfalse_to_hrresult((vt.IsSystemMoniker)(self.ptr(), &mut mksys.0))
				.map(|b| (b, mksys))
		}
	}

	/// [`IMoniker::ParseDisplayName`](https://learn.microsoft.com/en-us/windows/win32/api/objidl/nf-objidl-imoniker-parsedisplayname)
	/// method.
	#[must_use]
	fn ParseDisplayName(&self,
		bind_ctx: &impl ole_IBindCtx,
		moniker_to_left: &impl ole_IMoniker,
		display_name: &str,
	) -> HrResult<(u32, IMoniker)>
	{
		let mut ch_eaten = u32::default();
		unsafe {
			let mut ppv_queried = ComPtr::null();
			let vt = self.vt_ref::<IMonikerVT>();
			ok_to_hrresult(
				(vt.ParseDisplayName)(
					self.ptr(),
					bind_ctx.ptr(),
					moniker_to_left.ptr(),
					WString::from_str(display_name).as_ptr(),
					&mut ch_eaten,
					&mut ppv_queried,
				),
			).map(|_| (ch_eaten, IMoniker::from(ppv_queried)))
		}
	}

	/// [`IMoniker::Reduce`](https://learn.microsoft.com/en-us/windows/win32/api/objidl/nf-objidl-imoniker-reduce)
	/// method.
	///
	/// Returns the moniker to the left, and the reduced moniker, respectively.
	#[must_use]
	fn Reduce(&self,
		bind_ctx: &impl ole_IBindCtx,
		reduce_how_far: co::MKRREDUCE,
	) -> HrResult<(IMoniker, IMoniker)> {
		unsafe {
			let mut ppv_queried = ComPtr::null();
			let mut ppv_queried2 = ComPtr::null();
			let vt = self.vt_ref::<IMonikerVT>();
			ok_to_hrresult(
				(vt.Reduce)(
					self.ptr(),
					bind_ctx.ptr(),
					reduce_how_far.0,
					&mut ppv_queried,
					&mut ppv_queried2,
				),
			).map(|_| (IMoniker::from(ppv_queried), IMoniker::from(ppv_queried2)))
		}
	}

	/// [`IMoniker::RelativePathTo`](https://learn.microsoft.com/en-us/windows/win32/api/objidl/nf-objidl-imoniker-relativepathto)
	/// method.
	#[must_use]
	fn RelativePathTo(&self,
		other_moniker: &impl ole_IMoniker) -> HrResult<IMoniker>
	{
		unsafe {
			let mut ppv_queried = ComPtr::null();
			let vt = self.vt_ref::<IMonikerVT>();
			ok_to_hrresult(
				(vt.RelativePathTo)(
					self.ptr(),
					other_moniker.ptr(),
					&mut ppv_queried,
				),
			).map(|_| IMoniker::from(ppv_queried))
		}
	}
}
