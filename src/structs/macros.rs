/// Implements Default by zeroing all members.
macro_rules! impl_default_zero {
	($name:ident) => {
		impl Default for $name {
			fn default() -> Self {
				unsafe { std::mem::zeroed::<Self>() }
			}
		}
	};
}