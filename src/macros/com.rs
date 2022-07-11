#![allow(unused_macros)]

/// Implements ole_IUnknown trait to COM object, plus all its trait bounds.
macro_rules! impl_iunknown {
	($name:ident, $guid:expr) => {
		impl Drop for $name {
			fn drop(&mut self) {
				let vt = unsafe { &**(self.0.0 as *mut *mut crate::vt::IUnknownVT) };
				(vt.Release)(self.0); // call Release()
			}
		}

		impl Clone for $name {
			fn clone(&self) -> Self {
				let vt = unsafe { &**(self.0.0 as *mut *mut crate::vt::IUnknownVT) };
				(vt.AddRef)(self.0); // call AddRef()
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

			unsafe fn ptr(&self) -> ComPtr {
				self.0
			}
		}
	};
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
