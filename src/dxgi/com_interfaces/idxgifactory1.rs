#![allow(non_camel_case_types, non_snake_case)]

use crate::decl::*;
use crate::dxgi::{iterators::*, vts::*};
use crate::ole::privs::*;
use crate::prelude::*;

com_interface! { IDXGIFactory1: "770aae78-f26f-4dba-a829-253c83d1b387";
	/// [`IDXGIFactory1`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi/nn-dxgi-idxgifactory1)
	/// COM interface.
	///
	/// Automatically calls
	/// [`Release`](https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
	///
	/// Usually created with [`CreateDXGIFactory1`](crate::CreateDXGIFactory1)
	/// function.
	///
	/// # Examples
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*};
	///
	/// let factory1 = w::CreateDXGIFactory1()?;
	/// # w::HrResult::Ok(())
	/// ```
}

impl dxgi_IDXGIObject for IDXGIFactory1 {}
impl dxgi_IDXGIFactory for IDXGIFactory1 {}
impl dxgi_IDXGIFactory1 for IDXGIFactory1 {}

/// This trait is enabled with the `dxgi` feature, and provides methods for
/// [`IDXGIFactory1`](crate::IDXGIFactory1).
///
/// Prefer importing this trait through the prelude:
///
/// ```no_run
/// use winsafe::prelude::*;
/// ```
pub trait dxgi_IDXGIFactory1: dxgi_IDXGIFactory {
	/// [`IDXGIFactory1::EnumAdapters1`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi/nf-dxgi-idxgifactory1-enumadapters1)
	/// method.
	///
	/// Returns an iterator over [`IDXGIAdapter1`](crate::IDXGIAdapter1) elements.
	///
	/// # Examples
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*};
	///
	/// let factory: w::IDXGIFactory1; // initialized somewhere
	/// # let factory = unsafe { w::IDXGIFactory1::null() };
	///
	/// for adapter in factory.EnumAdapters1() {
	///     let adapter = adapter?;
	///     // ...
	/// }
	///
	/// // Collecting into a Vec
	/// let adapters: Vec<w::IDXGIAdapter1> =
	///     factory.EnumAdapters1()
	///         .collect::<w::HrResult<_>>()?;
	/// # w::HrResult::Ok(())
	/// ```
	#[must_use]
	fn EnumAdapters1(&self) -> impl Iterator<Item = HrResult<IDXGIAdapter1>> + '_ {
		IdxgifactoryEnumadapters1Iter::new(self)
	}

	/// [`IDXGIFactory1::IsCurrent`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi/nf-dxgi-idxgifactory1-iscurrent)
	/// method.
	#[must_use]
	fn IsCurrent(&self) -> bool {
		unsafe { (vt::<IDXGIFactory1VT>(self).IsCurrent)(self.ptr()) != 0 }
	}
}
