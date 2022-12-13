#![allow(non_camel_case_types, non_snake_case)]

use crate::co;
use crate::kernel::ffi_types::{HRES, PCVOID};
use crate::ole::decl::{ComPtr, HrResult};
use crate::ole::privs::ok_to_hrresult;

/// [`IUnknown`](crate::IUnknown) virtual table, base to all COM virtual tables.
#[repr(C)]
pub struct IUnknownVT {
	pub QueryInterface: fn(ComPtr, PCVOID, *mut ComPtr) -> HRES,
	pub AddRef: fn(ComPtr) -> u32,
	pub Release: fn(ComPtr) -> u32,
}

com_interface! { IUnknown: "00000000-0000-0000-c000-000000000046";
	/// [`IUnknown`](https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nn-unknwn-iunknown)
	/// COM interface over [`IUnknownVT`](crate::vt::IUnknownVT). It's the base to
	/// all COM interfaces.
	///
	/// The `clone` method calls
	/// [`AddRef`](https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-addref)
	/// internally.
	///
	/// Automatically calls
	/// [`Release`](https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
}

/// This trait is enabled with the `ole` feature, and provides methods for
/// [`IUnknown`](crate::IUnknown). It is the base trait for all COM traits.
///
/// Prefer importing this trait through the prelude:
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// ```
///
/// Note that the [`IUnknownVT`](crate::vt::IUnknownVT) virtual table has two
/// other methods: `AddRef` and `Release`. While these methods are relevant in
/// C++, here they are abstracted away as it follows:
///
/// * `AddRef` – called along the `clone` method from the
/// [`Clone`](https://doc.rust-lang.org/std/clone/trait.Clone.html) trait;
///
/// * `Release` – called automatically by the
/// [`Drop`](https://doc.rust-lang.org/std/ops/trait.Drop.html) trait, so you
/// don't need to worry about it.
pub trait ole_IUnknown: Clone + From<ComPtr> {
	/// The COM interface ID.
	const IID: co::IID;

	/// Returns the pointer to the underlying COM virtual table and sets the
	/// internal pointer to null, so that
	/// [`Release`](https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// won't be called.
	///
	/// **Note:** Be sure to release the pointer, otherwise, as the name of this
	/// method implies, you will cause a resource leak.
	#[must_use]
	unsafe fn leak(&mut self) -> ComPtr;

	/// Returns the pointer to the underlying COM virtual table.
	#[must_use]
	unsafe fn ptr(&self) -> ComPtr;

	/// Returns a reference to the underlying COM virtual table.
	///
	/// **Note:** Be sure the pointer has the correct virtual table type (or
	/// inherits from it), and it is not null.
	#[must_use]
	unsafe fn vt_ref<T>(&self) -> &T;

	/// [`IUnknown::QueryInterface`](https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-queryinterface(refiid_void))
	/// method.
	#[must_use]
	fn QueryInterface<T>(&self) -> HrResult<T>
		where T: ole_IUnknown,
	{
		unsafe {
			let mut ppv_queried = ComPtr::null();
			let vt = self.vt_ref::<IUnknownVT>();
			ok_to_hrresult(
				(vt.QueryInterface)(
					self.ptr(),
					&T::IID as *const _ as _,
					&mut ppv_queried,
				),
			).map(|_| T::from(ppv_queried))
		}
	}
}
