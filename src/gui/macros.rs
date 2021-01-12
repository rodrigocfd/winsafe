/// Implements `cref` and `mref` methods to retrieve references to `Obj` within
/// `UnsafeCell` in structs.
///
/// A wrapper over `Arc` would be ideal, but unfortunately it's not possible due
/// to a [compiler bug](https://github.com/rust-lang/rust/issues/26925).
macro_rules! cref_mref {
	($name:ident) => {
		impl $name {
			/// Returns a const ref to `Obj` within `UnsafeCell`.
			#[allow(dead_code)]
			fn cref(&self) -> &Obj {
				unsafe { &*self.obj.get() }
			}

			/// Returns a mut ref to `Obj` within `UnsafeCell`.
			#[allow(dead_code)]
			fn mref(&self) -> &mut Obj {
				unsafe { &mut *self.obj.get() }
			}
		}
	};
}
