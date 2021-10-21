/// Implements IUnknown trait to COM object, plus all its trait bounds.
macro_rules! impl_iunknown {
	($name:ident, $p1:expr, $p2:expr, $p3:expr, $p4:expr, $p5:expr) => {
		impl Drop for $name {
			fn drop(&mut self) {
				let vt = unsafe { &**(self.0.0 as *mut *mut crate::com::iunknown::IUnknownVT) };
				(vt.Release)(self.0); // call Release()
			}
		}

		impl Clone for $name {
			fn clone(&self) -> Self {
				let vt = unsafe { &**(self.0.0 as *mut *mut crate::com::iunknown::IUnknownVT) };
				(vt.AddRef)(self.0); // call AddRef()
				Self(self.0)
			}
		}

		impl From<ComPtr> for $name {
			fn from(com_ptr: ComPtr) -> Self {
				Self(com_ptr)
			}
		}

		impl crate::com::iunknown::ComInterface for $name {
			const IID: crate::structs::IID =
				crate::structs::IID::new($p1, $p2, $p3, $p4, $p5);
		}

		impl crate::com::iunknown::IUnknownT for $name {
			unsafe fn ptr(&self) -> ComPtr {
				self.0
			}
		}
	};
}

/// Creates multiple `GUID`-derived pub const values.
#[allow(unused_macros)]
macro_rules! pub_const_guid {
	(
		$type:ident,
		$($name:ident, $iid1:expr, $iid2:expr, $iid3:expr, $iid4:expr, $iid5:expr,)*
	) => {
		$(
			pub const $name: $type = $type::new($iid1, $iid2, $iid3, $iid4, $iid5);
		)*
	};
}
