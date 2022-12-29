/// A pointer to pointer to a COM virtual table.
#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct ComPtr(pub(crate) *mut *mut std::ffi::c_void);

impl From<ComPtr> for usize {
	fn from(com_ptr: ComPtr) -> Self {
		com_ptr.0 as _ // in practice, this serializes the pointer
	}
}

impl ComPtr {
	/// Converts the underlying raw pointer converted to a specific virtual
	/// table pointer.
	///
	/// # Safety
	///
	/// Be sure the pointer actually points to the given virtual table type.
	///
	/// This method is used internally by the library, and not intended to be
	/// used externally.
	#[must_use]
	pub const unsafe fn into_ptr<T>(self) -> *mut *mut T {
		self.0 as *mut *mut T
	}

	/// Creates a null pointer to a COM virtual table.
	///
	/// # Safety
	///
	/// A null pointer to a COM virtual table has no practical use, and calling
	/// methods upon it is undefined behavior.
	///
	/// This method is used internally by the library, and not intended to be
	/// used externally.
	#[must_use]
	pub const unsafe fn null() -> Self {
		Self(std::ptr::null_mut())
	}

	/// Returns `true` if the pointer is null.
	#[must_use]
	pub fn is_null(&self) -> bool {
		self.0.is_null()
	}
}
