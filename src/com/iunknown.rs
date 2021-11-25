#![allow(non_snake_case)]

use crate::aliases::WinResult;
use crate::ffi::{HRESULT, PCVOID};
use crate::privs::hr_to_winresult;
use crate::structs::IID;

/// A pointer to a COM virtual table.
#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct ComPtr(pub(in crate::com) *mut *mut IUnknownVT);

impl ComPtr {
	pub(in crate::com) fn null() -> Self {
		ComPtr(std::ptr::null_mut())
	}
}

/// Any
/// [COM](https://docs.microsoft.com/en-us/windows/win32/com/component-object-model--com--portal)
/// object, which encapsulates a COM interface pointer.
pub trait ComInterface: From<ComPtr> {
	/// The COM interface ID.
	const IID: IID;
}

//------------------------------------------------------------------------------

/// [`IUnknown`](crate::IUnknown) virtual table, base to all COM virtual tables.
#[repr(C)]
pub struct IUnknownVT {
	pub QueryInterface: fn(ComPtr, PCVOID, *mut ComPtr) -> HRESULT,
	pub AddRef: fn(ComPtr) -> u32,
	pub Release: fn(ComPtr) -> u32,
}

/// [`IUnknown`](https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nn-unknwn-iunknown)
/// COM interface over [`IUnknownVT`](crate::IUnknownVT). It's the base to all
/// COM interfaces.
///
/// The `clone` method calls
/// [`AddRef`](https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-addref)
/// internally.
///
/// Automatically calls
/// [`Release`](https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
/// when the object goes out of scope.
pub struct IUnknown(ComPtr);

impl_iunknown!(IUnknown, 0x00000000, 0x0000, 0x0000, 0xc000, 0x000000000046);

/// Exposes the [`IUnknown`](crate::IUnknown) methods.
pub trait IUnknownT: ComInterface + Clone {
	/// Returns the pointer to the underlying COM virtual table.
	unsafe fn ptr(&self) -> ComPtr;

	/// [`IUnknown::QueryInterface`](https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-queryinterface(refiid_void))
	/// method.
	fn QueryInterface<T: ComInterface>(&self) -> WinResult<T> {
		let mut ppv_queried = ComPtr::null();
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IUnknownVT);
			hr_to_winresult(
				(vt.QueryInterface)(
					self.ptr(),
					&T::IID as *const _ as _,
					&mut ppv_queried,
				),
			)
		}.map(|_| T::from(ppv_queried))
	}
}
