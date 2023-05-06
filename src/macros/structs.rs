#![allow(unused_macros)]

/// Implements `Default` trait by zeroing all fields.
macro_rules! impl_default {
	($name:ident $(, $life:lifetime)*) => {
		impl<$($life),*> Default for $name<$($life),*> {
			fn default() -> Self {
				unsafe { std::mem::zeroed::<Self>() }
			}
		}
	};
}

/// Implements `Default` trait by zeroing all fields. Also sets the size field
/// to struct size.
macro_rules! impl_default_with_size {
	($name:ident, $field:ident $(, $life:lifetime)*) => {
		impl<$($life),*> Default for $name<$($life),*> {
			fn default() -> Self {
				let mut obj = unsafe { std::mem::zeroed::<Self>() };
				obj.$field = std::mem::size_of::<Self>() as _;
				obj
			}
		}
	};
}

/// Implements `IntUnderlying`, and other needed traits.
macro_rules! impl_intunderlying {
	($name:ident, $ntype:ty) => {
		unsafe impl Send for $name {}

		impl From<$name> for $ntype {
			fn from(v: $name) -> Self {
				v.0
			}
		}

		impl AsRef<$ntype> for $name {
			fn as_ref(&self) -> &$ntype {
				&self.0
			}
		}

		impl crate::prelude::IntUnderlying for $name {
			type Raw = $ntype;

			unsafe fn as_mut(&mut self) -> &mut Self::Raw {
				&mut self.0
			}
		}

		impl $name {
			/// Constructs a new `IntUnderlying` by wrapping the given integer
			/// value.
			///
			/// # Safety
			///
			/// Be sure the given value is meaningful for the actual type.
			#[must_use]
			pub const unsafe fn from_raw(v: $ntype) -> Self {
				Self(v)
			}

			/// Returns the primitive integer underlying value.
			///
			/// This method is similar to
			/// [`Into`](https://doc.rust-lang.org/std/convert/trait.Into.html),
			/// but it is `const`, therefore it can be used in
			/// [const contexts](https://doc.rust-lang.org/reference/const_eval.html).
			#[must_use]
			pub const fn raw(&self) -> $ntype {
				self.0
			}
		}
	};
}

/// Implements a serialization method.
macro_rules! pub_fn_serialize {
	() => {
		/// Serializes the struct into `&[u8]`.
		pub fn serialize<'seri>(&'seri self) -> &'seri [u8] {
			unsafe {
				std::slice::from_raw_parts(
					self as *const _ as _,
					std::mem::size_of::<Self>(),
				)
			}
		}
	};
}

/// Implements getter and setter methods for the given `BOOL` field.
macro_rules! pub_fn_bool_get_set {
	($field:ident, $setter:ident) => {
		/// Returns the bool field.
		#[must_use]
		pub const fn $field(&self) -> bool {
			self.$field != 0
		}

		/// Sets the bool field.
		pub fn $setter(&mut self, val: bool) {
			self.$field = val as _
		}
	};
}

/// Implements getter and setter methods for the given resource ID field, stored
/// as `*mut u16`.
macro_rules! pub_fn_resource_id_get_set {
	($field:ident, $setter:ident) => {
		/// Returns the resource ID field.
		#[must_use]
		pub fn $field(&self) -> u16 {
			self.$field as _
		}

		/// Sets the resource ID field.
		pub fn $setter(&mut self, val: u16) {
			self.$field = val as _;
		}
	};
}

/// Implements getter and setter methods for the given `*mut u16` field.
macro_rules! pub_fn_string_ptr_get_set {
	($life:lifetime, $field:ident, $setter:ident) => {
		/// Returns the string field, if any.
		#[must_use]
		pub fn $field(&self) -> Option<String> {
			unsafe { self.$field.as_mut() }.map(|psz| {
				WString::from_wchars_nullt(psz).to_string()
			})
		}

		/// Sets the string field.
		pub fn $setter(&mut self, buf: Option<&$life mut WString>) {
			self.$field = buf.map_or(std::ptr::null_mut(), |buf| unsafe { buf.as_mut_ptr() });
		}
	};
}

/// Implements getter and setter methods for the given `*mut u16` and `u32`
/// fields, setting pointer and its actual chars length without terminating
/// null.
macro_rules! pub_fn_string_ptrlen_get_set {
	($life:lifetime, $field:ident, $setter:ident, $length:ident) => {
		/// Returns the string field, if any.
		#[must_use]
		pub fn $field(&self) -> Option<String> {
			unsafe { self.$field.as_mut() }.map(|psz| {
				WString::from_wchars_count(psz, self.$length as _).to_string()
			})
		}

		/// Sets the string field.
		pub fn $setter(&mut self, buf: Option<&$life mut WString>) {
			self.$length = buf.as_ref().map_or(0, |buf| buf.str_len() as _);
			self.$field = buf.map_or(std::ptr::null_mut(), |buf| unsafe { buf.as_mut_ptr() });
		}
	};
}

/// Implements getter and setter methods for the given `[u16; N]` field.
macro_rules! pub_fn_string_arr_get_set {
	($field:ident, $setter:ident) => {
		/// Returns the string field.
		#[must_use]
		pub fn $field(&self) -> String {
			crate::kernel::decl::WString::from_wchars_slice(&self.$field).to_string()
		}

		/// Sets the string field.
		pub fn $setter(&mut self, text: &str) {
			crate::kernel::decl::WString::from_str(text).copy_to_slice(&mut self.$field);
		}
	};
}

/// Implements getter and setter methods for the given `*mut 16` and `i32`
/// fields, setting buffer and its size.
macro_rules! pub_fn_string_buf_get_set {
	($life:lifetime, $field:ident, $setter:ident, $cch:ident) => {
		/// Returns the string field.
		#[must_use]
		pub fn $field(&self) -> Option<String> {
			unsafe { self.$field.as_mut() }.map(|psz| {
				WString::from_wchars_nullt(psz).to_string()
			})
		}

		/// Sets the string field.
		pub fn $setter(&mut self, buf: Option<&$life mut WString>) {
			self.$cch = buf.as_ref().map_or(0, |buf| buf.buf_len() as _);
			self.$field = buf.map_or(std::ptr::null_mut(), |buf| unsafe { buf.as_mut_ptr() });
		}
	};
}

/// Implements getter and setter methods for the given pointer field.
macro_rules! pub_fn_ptr_get_set {
	($life:lifetime, $field:ident, $setter:ident, $ty:ty) => {
		/// Returns the pointer field.
		#[must_use]
		pub fn $field(&self) -> Option<&$life mut $ty> {
			unsafe { self.$field.as_mut() }
		}

		/// Sets the pointer field.
		pub fn $setter(&mut self, obj: Option<&$life mut $ty>) {
			self.$field = obj.map_or(std::ptr::null_mut(), |obj| obj);
		}
	};
}

/// Implements getter and setter methods for the given array + size fields,
/// setting buffer and its size.
macro_rules! pub_fn_array_buf_get_set {
	($life:lifetime, $field:ident, $setter:ident, $cch:ident, $ty:ty) => {
		/// Returns the array field.
		#[must_use]
		pub fn $field(&self) -> Option<&$life mut [$ty]> {
			unsafe {
				self.$field.as_mut().map(|p| {
					std::slice::from_raw_parts_mut(p, self.$cch as _)
				})
			}
		}

		/// Sets the array field.
		pub fn $setter(&mut self, buf: Option<&$life mut [$ty]>) {
			match buf {
				Some(buf) => {
					self.$field = buf as *mut _ as _;
					self.$cch = buf.len() as _;
				},
				None => {
					self.$field = std::ptr::null_mut();
					self.$cch = 0;
				},
			}
		}
	};
}

/// Implements getter and setter methods for the given `ComPtr` field.
macro_rules! pub_fn_comptr_get_set {
	($field:ident, $setter:ident, $trait:ident) => {
		/// Returns the COM object field, by cloning the underlying COM pointer.
		#[must_use]
		pub fn $field<T>(&self) -> Option<T>
			where T: $trait,
		{
			self.$field.as_opt().map(|ptr| {
				let obj = std::mem::ManuallyDrop::new(T::from(*ptr)); // won't release the stored pointer
				let cloned = T::clone(&obj);
				cloned
			})
		}

		/// Sets the COM object field, by cloning the underlying COM pointer.
		pub fn $setter<T>(&mut self, obj: Option<&T>)
			where T: $trait,
		{
			let _ = T::from(self.$field); // if already set, call Release() immediately
			self.$field = obj.map_or_else(
				|| unsafe { crate::ComPtr::null() },
				|obj| {
					let mut cloned = T::clone(obj);
					cloned.leak()
				},
			);
		}
	};
}

/// Implements `Drop` for a `ComPtr` field.
macro_rules! impl_drop_comptr {
	($field:ident, $name:ident $(, $life:lifetime)*) => {
		impl<$($life),*> Drop for $name<$($life),*> {
			fn drop(&mut self) {
				if let Some(p) = self.$field.as_opt() {
					let _ = crate::IUnknown::from(*p); // if a pointer is present, call Release() on it
				}
			}
		}
	};
}
