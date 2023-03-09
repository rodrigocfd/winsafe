#![allow(unused_macros)]

/// Declares an ordinary COM interface, and implements ole_IUnknown trait.
macro_rules! com_interface {
	(
		$name:ident : $guid:expr;
		$( #[$doc:meta] )*
	) => {
		$( #[$doc] )*
		#[repr(transparent)]
		pub struct $name(ComPtr);

		impl Drop for $name {
			fn drop(&mut self) {
				if let Some(p) = self.0.as_opt() {
					use crate::{prelude::ole_IUnknown, vt::IUnknownVT};
					let vt = unsafe { self.vt_ref::<IUnknownVT>() };
					(vt.Release)(*p);
				}
			}
		}

		impl Clone for $name {
			fn clone(&self) -> Self {
				use crate::{prelude::ole_IUnknown, vt::IUnknownVT};
				let vt = unsafe { self.vt_ref::<IUnknownVT>() };
				(vt.AddRef)(self.0);
				Self(self.0)
			}
		}

		impl From<ComPtr> for $name {
			fn from(com_ptr: ComPtr) -> Self {
				Self(com_ptr)
			}
		}

		impl crate::prelude::ole_IUnknown for $name {
			const IID: crate::co::IID = crate::co::IID::new($guid);

			fn leak(&mut self) -> ComPtr {
				let ptr = self.0;
				self.0 = unsafe { ComPtr::null() };
				ptr
			}

			unsafe fn ptr(&self) -> ComPtr {
				self.0
			}

			unsafe fn vt_ref<T>(&self) -> &T {
				&**self.0.into_ptr::<T>()
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
		#[derive(Clone, Copy, Eq, PartialEq, Hash)]
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
