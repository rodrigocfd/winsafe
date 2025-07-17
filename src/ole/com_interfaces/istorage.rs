#![allow(non_camel_case_types, non_snake_case)]

use crate::co;
use crate::decl::*;
use crate::kernel::privs::*;
use crate::ole::{privs::*, vts::*};
use crate::prelude::*;

com_interface! { IStorage: "0000000b-0000-0000-c000-000000000046";
	/// [`IStorage`](https://learn.microsoft.com/en-us/windows/win32/api/objidl/nn-objidl-istorage)
	/// COM interface.
	///
	/// Automatically calls
	/// [`Release`](https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
}

impl ole_IStorage for IStorage {}

/// This trait is enabled with the `ole` feature, and provides methods for
/// [`IStorage`](crate::IStorage).
///
/// Prefer importing this trait through the prelude:
///
/// ```no_run
/// use winsafe::prelude::*;
/// ```
pub trait ole_IStorage: ole_IUnknown {
	/// [`IStorage::Commit`](https://learn.microsoft.com/en-us/windows/win32/api/objidl/nf-objidl-istorage-commit)
	/// method.
	fn Commit(&self, commit_flags: co::STGC) -> HrResult<()> {
		HrRet(unsafe { (vt::<IStorageVT>(self).Commit)(self.ptr(), commit_flags.raw()) })
			.to_hrresult()
	}

	/// [`IStorage::CopyTo`](https://learn.microsoft.com/en-us/windows/win32/api/objidl/nf-objidl-istorage-copyto)
	/// method.
	fn CopyTo(
		&self,
		iid_exclude: &[co::IID],
		snb_exclude: &[impl AsRef<str>],
		stg_dest: &impl ole_IStorage,
	) -> HrResult<()> {
		HrRet(unsafe {
			(vt::<IStorageVT>(self).CopyTo)(
				self.ptr(),
				iid_exclude.len() as _,
				vec_ptr(iid_exclude) as _,
				SNB::from_strs(snb_exclude)?.as_ptr(),
				stg_dest.ptr(),
			)
		})
		.to_hrresult()
	}

	/// [`IStorage::CreateStorage`](https://learn.microsoft.com/en-us/windows/win32/api/objidl/nf-objidl-istorage-createstorage)
	/// method.
	#[must_use]
	fn CreateStorage(&self, name: &str, grf_mode: co::STGM) -> HrResult<IStorage> {
		let mut queried = unsafe { IStorage::null() };
		HrRet(unsafe {
			(vt::<IStorageVT>(self).CreateStorage)(
				self.ptr(),
				WString::from_str(name).as_ptr(),
				grf_mode.raw(),
				0,
				0,
				queried.as_mut(),
			)
		})
		.to_hrresult()
		.map(|_| queried)
	}

	/// [`IStorage::CreateStream`](https://learn.microsoft.com/en-us/windows/win32/api/objidl/nf-objidl-istorage-createstream)
	/// method.
	#[must_use]
	fn CreateStream(&self, name: &str, grf_mode: co::STGM) -> HrResult<IStream> {
		let mut queried = unsafe { IStream::null() };
		HrRet(unsafe {
			(vt::<IStorageVT>(self).CreateStream)(
				self.ptr(),
				WString::from_str(name).as_ptr(),
				grf_mode.raw(),
				0,
				0,
				queried.as_mut(),
			)
		})
		.to_hrresult()
		.map(|_| queried)
	}

	/// [`IStorage::DestroyElement`](https://learn.microsoft.com/en-us/windows/win32/api/objidl/nf-objidl-istorage-destroyelement)
	/// method.
	fn DestroyElement(&self, name: &str) -> HrResult<()> {
		HrRet(unsafe {
			(vt::<IStorageVT>(self).DestroyElement)(self.ptr(), WString::from_str(name).as_ptr())
		})
		.to_hrresult()
	}

	/// [`IStorage::MoveElementTo`](https://learn.microsoft.com/en-us/windows/win32/api/objidl/nf-objidl-istorage-moveelementto)
	/// method.
	fn MoveElementTo(
		&self,
		name: &str,
		stg_dest: &impl ole_IStorage,
		new_name: &str,
		grf_flags: co::STGMOVE,
	) -> HrResult<()> {
		HrRet(unsafe {
			(vt::<IStorageVT>(self).MoveElementTo)(
				self.ptr(),
				WString::from_str(name).as_ptr(),
				stg_dest.ptr(),
				WString::from_str(new_name).as_ptr(),
				grf_flags.raw(),
			)
		})
		.to_hrresult()
	}

	/// [`IStorage::OpenStorage`](https://learn.microsoft.com/en-us/windows/win32/api/objidl/nf-objidl-istorage-openstorage)
	/// method.
	#[must_use]
	fn OpenStorage(&self, name: &str, grf_mode: co::STGM) -> HrResult<IStorage> {
		let mut queried = unsafe { IStorage::null() };
		HrRet(unsafe {
			(vt::<IStorageVT>(self).OpenStorage)(
				self.ptr(),
				WString::from_str(name).as_ptr(),
				std::ptr::null_mut(),
				grf_mode.raw(),
				std::ptr::null_mut(),
				0,
				queried.as_mut(),
			)
		})
		.to_hrresult()
		.map(|_| queried)
	}

	/// [`IStorage::OpenStream`](https://learn.microsoft.com/en-us/windows/win32/api/objidl/nf-objidl-istorage-openstream)
	/// method.
	#[must_use]
	fn OpenStream(&self, name: &str, grf_mode: co::STGM) -> HrResult<IStream> {
		let mut queried = unsafe { IStream::null() };
		HrRet(unsafe {
			(vt::<IStorageVT>(self).OpenStream)(
				self.ptr(),
				WString::from_str(name).as_ptr(),
				std::ptr::null_mut(),
				grf_mode.raw(),
				0,
				queried.as_mut(),
			)
		})
		.to_hrresult()
		.map(|_| queried)
	}

	/// [`IStorage::RenameElement`](https://learn.microsoft.com/en-us/windows/win32/api/objidl/nf-objidl-istorage-renameelement)
	/// method.
	fn RenameElement(&self, old_name: &str, new_name: &str) -> HrResult<()> {
		HrRet(unsafe {
			(vt::<IStorageVT>(self).RenameElement)(
				self.ptr(),
				WString::from_str(old_name).as_ptr(),
				WString::from_str(new_name).as_ptr(),
			)
		})
		.to_hrresult()
	}

	fn_com_noparm! { Revert: IStorageVT;
		/// [`IStorage::Revert`](https://learn.microsoft.com/en-us/windows/win32/api/objidl/nf-objidl-istorage-revert)
		/// method.
	}

	/// [`IStorage::SetClass`](https://learn.microsoft.com/en-us/windows/win32/api/objidl/nf-objidl-istorage-setclass)
	/// method.
	fn SetClass(&self, clsid: &co::CLSID) -> HrResult<()> {
		HrRet(unsafe { (vt::<IStorageVT>(self).SetClass)(self.ptr(), pcvoid(clsid)) }).to_hrresult()
	}

	/// [`IStorage::SetElementTimes`](https://learn.microsoft.com/en-us/windows/win32/api/objidl/nf-objidl-istorage-setelementtimes)
	/// method.
	fn SetElementTimes(
		&self,
		name: Option<&str>,
		creation: Option<&FILETIME>,
		access: Option<&FILETIME>,
		modification: Option<&FILETIME>,
	) -> HrResult<()> {
		HrRet(unsafe {
			(vt::<IStorageVT>(self).SetElementTimes)(
				self.ptr(),
				WString::from_opt_str(name).as_ptr(),
				pcvoid_or_null(creation),
				pcvoid_or_null(access),
				pcvoid_or_null(modification),
			)
		})
		.to_hrresult()
	}

	/// [`IStorage::SetStateBits`](https://learn.microsoft.com/en-us/windows/win32/api/objidl/nf-objidl-istorage-setstatebits)
	/// method.
	fn SetStateBits(&self, state_bits: u32, mask: u32) -> HrResult<()> {
		HrRet(unsafe { (vt::<IStorageVT>(self).SetStateBits)(self.ptr(), state_bits, mask) })
			.to_hrresult()
	}
}
