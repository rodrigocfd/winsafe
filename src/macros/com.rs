#![allow(unused_macros)]

/// Declares an ordinary COM interface, and implements ole_IUnknown trait.
macro_rules! com_interface {
	(
		$name:ident : $feature:literal;
		$guid:expr;
		$( #[$doc:meta] )*
	) => {
		$( #[$doc] )*
		#[cfg_attr(docsrs, doc(cfg(feature = $feature)))]
		#[repr(transparent)]
		pub struct $name(ComPtr);

		impl Drop for $name {
			fn drop(&mut self) {
				if !self.0.is_null() {
					let vt = unsafe { &**(self.0.0 as *mut *mut crate::vt::IUnknownVT) };
					(vt.Release)(self.0);
				}
			}
		}

		impl Clone for $name {
			fn clone(&self) -> Self {
				let vt = unsafe { &**(self.0.0 as *mut *mut crate::vt::IUnknownVT) };
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

			unsafe fn leak(&mut self) -> ComPtr {
				let ptr = self.0;
				self.0 = ComPtr::null();
				ptr
			}

			unsafe fn ptr(&self) -> ComPtr {
				self.0
			}

			unsafe fn vt_ref<T>(&self) -> &T {
				&**(self.0.0 as *mut *mut T)
			}
		}
	}
}

/// Creates multiple `GUID`-derived pub const values.
macro_rules! const_guid_values {
	(
		$name:ident $(: $feature:literal)*;
		$($pubname:ident $guid:expr)*
	) => {
		$( #[cfg_attr(docsrs, doc(cfg(feature = $feature)))] )*
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
		$name:ident : $feature:literal;
		$( #[$doc:meta] )*
		=>
		$(
			$pubname:ident $guid:expr
		)*
	) => {
		$( #[$doc] )*
		#[cfg_attr(docsrs, doc(cfg(feature = $feature)))]
		#[repr(transparent)]
		#[derive(Clone, Copy, Eq, PartialEq, Hash)]
		pub struct $name(crate::ole::decl::GUID);

		impl From<crate::ole::decl::GUID> for $name {
			fn from(guid: crate::ole::decl::GUID) -> Self {
				Self(guid)
			}
		}

		impl AsRef<crate::ole::decl::GUID> for $name {
			fn as_ref(&self) -> &crate::ole::decl::GUID {
				&self.0
			}
		}

		impl std::fmt::Display for $name {
			fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
				self.0.fmt(f)
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
			pub const fn new(guid_str: &str) -> $name {
				Self(crate::ole::decl::GUID::new(guid_str))
			}
		}

		const_guid_values! {
			$name;
			$(
				$pubname $guid
			)*
		}
	};
}
