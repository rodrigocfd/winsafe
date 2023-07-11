#![allow(unused_macros)]

/// Declares an ordinary COM interface, and implements ole_IUnknown trait.
macro_rules! com_interface {
	(
		$name:ident : $guid:expr;
		$( #[$doc:meta] )*
	) => {
		$( #[$doc] )*
		#[repr(transparent)]
		pub struct $name(crate::kernel::ffi_types::COMPTR);

		impl Drop for $name {
			fn drop(&mut self) {
				if !self.0.is_null() {
					unsafe {
						use crate::{prelude::ole_IUnknown, vt::IUnknownVT};
						(crate::ole::privs::vt::<IUnknownVT>(self).Release)(self.ptr());
					}
				}
			}
		}

		impl Clone for $name {
			fn clone(&self) -> Self {
				use crate::{prelude::ole_IUnknown, vt::IUnknownVT};
				unsafe { (crate::ole::privs::vt::<IUnknownVT>(self).AddRef)(self.ptr()); }
				Self(self.ptr())
			}
		}

		impl crate::prelude::ole_IUnknown for $name {
			const IID: crate::co::IID = crate::co::IID::new($guid);

			unsafe fn from_ptr(p: *mut std::ffi::c_void) -> Self {
				Self(p)
			}

			unsafe fn as_mut(&mut self) -> &mut *mut std::ffi::c_void {
				&mut self.0
			}

			fn ptr(&self) -> *mut std::ffi::c_void {
				self.0
			}
		}
	};
}

/// Creates multiple `GUID`-derived pub const values.
macro_rules! const_guid_values {
	(
		$name:ident;
		$( $pubname:ident $guid:expr )*
	) => {
		impl $name {
			$(
				pub const $pubname: $name = $name::new($guid);
			)*
		}
	};
}

/// Declares the type of a GUID constant, along with public values.
macro_rules! const_guid {
	(
		$name:ident;
		$( #[$doc:meta] )*
		=>
		$( $pubname:ident $guid:expr )*
	) => {
		$( #[$doc] )*
		#[repr(transparent)]
		#[derive(Clone, Copy, PartialEq, Eq, Hash)]
		pub struct $name(crate::kernel::decl::GUID);

		impl From<crate::kernel::decl::GUID> for $name {
			fn from(guid: crate::kernel::decl::GUID) -> Self {
				Self(guid)
			}
		}

		impl AsRef<crate::kernel::decl::GUID> for $name {
			fn as_ref(&self) -> &crate::kernel::decl::GUID {
				&self.0
			}
		}

		impl std::fmt::Display for $name {
			fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
				self.0.fmt(f)
			}
		}

		impl Default for $name {
			fn default() -> Self {
				Self::new("00000000-0000-0000-c000-000000000046") // IUnknown GUID
			}
		}

		impl $name {
			/// Creates a new `GUID` from a representative hex string, which can
			/// be copied straight from standard `GUID` declarations.
			///
			/// # Examples
			///
			/// ```rust,no_run
			/// use winsafe::prelude::*;
			/// use winsafe::GUID;
			///
			/// let g = GUID::new("00000000-0000-0000-c000-000000000046");
			/// ```
			#[must_use]
			pub const fn new(guid_str: &str) -> Self {
				Self(crate::kernel::decl::GUID::new(guid_str))
			}
		}

		const_guid_values! {
			$name;
			$( $pubname $guid )*
		}
	};
}

/// Implements a trait function with no parameters.
macro_rules! fn_com_noparm {
	(
		$method:ident : $vt:ty;
		$( #[$doc:meta] )*
	) => {
		$( #[$doc] )*
		fn $method(&self) -> HrResult<()> {
			crate::ole::privs::ok_to_hrresult(
				unsafe {
					(crate::ole::privs::vt::<$vt>(self).$method)(self.ptr())
				},
			)
		}
	};
}

/// Implements a trait function for a COM interface getter, no parameters.
macro_rules! fn_com_get {
	(
		$method:ident : $vt:ty, $iface:ty;
		$( #[$doc:meta] )*
	) => {
		$( #[$doc] )*
		#[must_use]
		fn $method(&self) -> HrResult<$iface> {
			use crate::prelude::ole_IUnknown;
			let mut queried = unsafe { <$iface>::null() };
			crate::ole::privs::ok_to_hrresult(
				unsafe {
					(crate::ole::privs::vt::<$vt>(self).$method)(self.ptr(), queried.as_mut())
				},
			).map(|_| queried)
		}
	};
}

/// Implements a trait function for a BSTR getter, no parameters.
macro_rules! fn_bstr_get {
	(
		$method:ident : $vt:ty;
		$( #[$doc:meta] )*
	) => {
		$( #[$doc] )*
		#[must_use]
		fn $method(&self) -> HrResult<String> {
			let mut pstr = std::ptr::null_mut::<u16>();
			crate::ole::privs::ok_to_hrresult(
				unsafe {
					(crate::ole::privs::vt::<$vt>(self).$method)(self.ptr(), &mut pstr)
				},
			).map(|_| {
				let bstr = unsafe { crate::oleaut::decl::BSTR::from_ptr(pstr) };
				bstr.to_string()
			})
		}
	};
}

/// Implements a trait function for a BSTR setter, single parameter.
macro_rules! fn_bstr_set {
	(
		$method:ident : $vt:ty, $arg:ident;
		$( #[$doc:meta] )*
	) => {
		$( #[$doc] )*
		fn $method(&self, $arg: &str) -> HrResult<()> {
			crate::ole::privs::ok_to_hrresult(
				unsafe {
					(crate::ole::privs::vt::<$vt>(self).$method)(
						self.ptr(),
						crate::oleaut::decl::BSTR::SysAllocString($arg)?.as_ptr(),
					)
				},
			)
		}
	};
}
