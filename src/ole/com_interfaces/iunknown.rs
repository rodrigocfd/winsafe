#![allow(non_camel_case_types, non_snake_case)]

use crate::co;
use crate::kernel::ffi_types::{HRES, PCVOID};
use crate::ole::decl::HrResult;
use crate::ole::privs::ok_to_hrresult;

/// A pointer to a COM virtual table.
#[cfg_attr(docsrs, doc(cfg(feature = "ole")))]
#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct ComPtr(pub(crate) *mut *mut IUnknownVT);

impl From<ComPtr> for usize {
	fn from(com_ptr: ComPtr) -> Self {
		com_ptr.0 as _
	}
}

impl ComPtr {
	/// Creates a null pointer to a COM virtual table.
	///
	/// Used internally by the library.
	#[must_use]
	pub const unsafe fn null() -> Self {
		Self(std::ptr::null_mut())
	}

	/// Returns `true` if the pointer is null.
	#[must_use]
	pub fn is_null(&self) -> bool {
		self.0.is_null()
	}
}

//------------------------------------------------------------------------------

/// [`IUnknown`](crate::IUnknown) virtual table, base to all COM virtual tables.
#[cfg_attr(docsrs, doc(cfg(feature = "ole")))]
#[repr(C)]
pub struct IUnknownVT {
	pub QueryInterface: fn(ComPtr, PCVOID, *mut ComPtr) -> HRES,
	pub AddRef: fn(ComPtr) -> u32,
	pub Release: fn(ComPtr) -> u32,
}

/// [`IUnknown`](https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nn-unknwn-iunknown)
/// COM interface over [`IUnknownVT`](crate::vt::IUnknownVT). It's the base to
/// all COM interfaces.
///
/// The `clone` method calls
/// [`AddRef`](https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-addref)
/// internally.
///
/// Automatically calls
/// [`Release`](https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
/// when the object goes out of scope.
#[cfg_attr(docsrs, doc(cfg(feature = "ole")))]
pub struct IUnknown(ComPtr);

impl_iunknown!(IUnknown, "00000000-0000-0000-c000-000000000046");

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
#[cfg_attr(docsrs, doc(cfg(feature = "ole")))]
pub trait ole_IUnknown: Clone + From<ComPtr> {
	/// The COM interface ID.
	const IID: co::IID;

	/// Returns the pointer to the underlying COM virtual table and sets the
	/// internal pointer to null, so that
	/// [`Release`](https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// won't be called.
	///
	/// **Note:** Be sure to release the pointer, otherwise, as the name of this
	/// method implies, you will cause a resource leak.
	#[must_use]
	unsafe fn leak(&mut self) -> ComPtr;

	/// Returns the pointer to the underlying COM virtual table.
	#[must_use]
	unsafe fn ptr(&self) -> ComPtr;

	/// [`IUnknown::QueryInterface`](https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-queryinterface(refiid_void))
	/// method.
	#[must_use]
	fn QueryInterface<T>(&self) -> HrResult<T>
		where T: ole_IUnknown,
	{
		unsafe {
			let mut ppv_queried = ComPtr::null();
			let vt = &**(self.ptr().0 as *mut *mut IUnknownVT);
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
