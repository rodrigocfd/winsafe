/// Implements `cref` and `mref` methods to retrieve references to `Obj` within
/// `UnsafeCell` in structs.
macro_rules! cref_mref {
	($name:ident) => {
		impl $name {
			/// Returns a const ref to `Obj` within `Arc` or `Rc`.
			#[allow(dead_code)]
			fn cref(&self) -> &Obj {
				unsafe { &*self.obj.get() }
			}

			/// Returns a mut ref to `Obj` within `Arc` or `Rc`.
			#[allow(dead_code)]
			fn mref(&self) -> &mut Obj {
				unsafe { &mut *self.obj.get() }
			}
		}
	};
}
