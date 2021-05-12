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

/// Implements getter and setter methods for the given `*mut u16` member.
macro_rules! string_get_set {
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
