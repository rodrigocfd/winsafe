#![allow(non_snake_case)]

use crate::handles::HandleClose;

/// Handle to an
/// [event](https://docs.microsoft.com/en-us/windows/win32/api/synchapi/nf-synchapi-createeventw).
/// Originally just a `HANDLE`.
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct HEVENT(pub(crate) *mut std::ffi::c_void);

impl_handle!(HEVENT);
impl HandleClose for HEVENT {}

/// Handle to a
/// [resource](https://docs.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-findresourcew).
/// Originally just a `HANDLE`.
///
/// For an example, see
/// [`HINSTANCE::LockResource`](crate::HINSTANCE::LockResource).
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct HRSRC(pub(crate) *mut std::ffi::c_void);

impl_handle!(HRSRC);

/// Handle to a resource
/// [memory block](https://docs.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-loadresource).
/// Originally just an `HGLOBAL`.
///
/// For an example, see
/// [`HINSTANCE::LockResource`](crate::HINSTANCE::LockResource).
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct HRSRCMEM(pub(crate) *mut std::ffi::c_void);

impl_handle!(HRSRCMEM);

/// Handle to an
/// [tree view item](https://docs.microsoft.com/en-us/windows/win32/controls/tree-view-controls).
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct HTREEITEM(pub(crate) *mut std::ffi::c_void);

impl_handle!(HTREEITEM);
