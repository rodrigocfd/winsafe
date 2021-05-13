/// Implements `Default` trait by zeroing all members.
macro_rules! impl_default_zero {
	($name:ident $(, $life:lifetime)*) => {
		impl<$($life),*> Default for $name<$($life),*> {
			fn default() -> Self {
				unsafe { std::mem::zeroed::<Self>() }
			}
		}
	};
}

/// Implements `Default` trait by zeroing all members. Also sets the size field
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

/// Implements getter and setter methods for the given `*mut u16` member.
macro_rules! string_ptr_get_set {
	($life:lifetime, $field:ident, $setter:ident) => {
		/// Returns the string field, if any.
		pub fn $field(&self) -> Option<String> {
			unsafe { self.$field.as_mut() }
				.map(|psz| WString::from_wchars_nullt(psz).to_string())
		}

		/// Sets the string field.
		pub fn $setter(&mut self, buf: &$life mut WString) {
			self.$field = unsafe { buf.as_mut_ptr() };
		}
	};
}

/// Implements getter and setter methods for the given `[u16; N]` member.
macro_rules! string_arr_get_set {
	($field:ident, $setter:ident) => {
		/// Returns the string field.
		pub fn $field(&self) -> String {
			WString::from_wchars_slice(&self.$field).to_string()
		}

		/// Sets the string field.
		pub fn $setter(&mut self, text: &str) {
			WString::from_str(text).copy_to_slice(&mut self.$field);
		}
	};
}
