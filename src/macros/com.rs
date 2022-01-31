#![allow(unused_macros)]

/// Implements IUnknown trait to COM object, plus all its trait bounds.
macro_rules! impl_iunknown {
	($name:ident, $p1:expr, $p2:expr, $p3:expr, $p4:expr, $p5:expr) => {
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

		impl crate::prelude::ComInterface for $name {
			const IID: crate::ole::decl::IID =
				crate::ole::decl::IID::new($p1, $p2, $p3, $p4, $p5);
		}

		impl crate::prelude::OleIUnknown for $name {
			unsafe fn ptr(&self) -> ComPtr {
				self.0
			}
		}
	};
}

/// Creates a safe wrapper to a `GUID`.
macro_rules! pub_guid_wrapper {
	(
		$name:ident : $feature:literal;
		$( #[$doc:meta] )*
	) => {
		$( #[$doc] )*
		#[cfg_attr(docsrs, doc(cfg(feature = $feature)))]
		#[repr(transparent)]
		#[derive(Copy, Clone, Eq, PartialEq, Hash)]
		pub struct $name(crate::ole::decl::GUID);

		impl From<crate::ole::decl::GUID> for $name {
			fn from(guid: crate::ole::decl::GUID) -> Self {
				Self(guid)
			}
		}

		impl AsRef<GUID> for $name {
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
			/// Creates a new object from hex numbers, which can be copied
			/// straight from standard GUID definitions.
			///
			/// # Examples
			///
			/// ```rust,no_run
			/// use winsafe::prelude::*;
			/// use winsafe::GUID;
			///
			/// let g = GUID::new(0x00000000, 0x0000, 0x0000, 0xc000, 0x000000000046);
			/// ```
			pub const fn new(p1: u32, p2: u16, p3: u16, p4: u16, p5: u64) -> $name {
				Self(crate::ole::decl::GUID::new(p1, p2, p3, p4, p5))
			}
		}
	};
}

/// Creates multiple `GUID`-derived pub const values.
macro_rules! pub_const_guid {
	(
		$type:ident: $feature:literal;
		$($name:ident $iid1:expr, $iid2:expr, $iid3:expr, $iid4:expr, $iid5:expr)*
	) => {
		#[cfg_attr(docsrs, doc(cfg(feature = $feature)))]
		impl $type {
			$(
				pub const $name: $type = $type::new($iid1, $iid2, $iid3, $iid4, $iid5);
			)*
		}
	};
}
