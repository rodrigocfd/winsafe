/// Stores closures, associating them with an identifier.
pub(crate) struct FuncStore<K, F> {
	elems: Vec<(K, F)>,
}

impl<K: Copy + Eq, F> FuncStore<K, F> {
	/// Creates a new, empty store.
	pub fn new() -> FuncStore<K, F> {
		Self {
			elems: Vec::new(),
		}
	}

	/// Inserts a new function into the store, associated to the given
	/// identifier.
	pub fn insert(&mut self, id: K, func: F) {
		if self.elems.is_empty() {
			self.elems.reserve(16); // arbitrary, prealloc for speed
		}
		self.elems.push((id, func));
	}

	/// Finds the last added function associated to the given identifier, if
	/// any.
	pub fn find(&mut self, id: K) -> Option<&mut F> {
		// Linear search, more performant for small collections.
		// Searches backwards, so the function added last will overwrite the first.
		for elem in self.elems.iter_mut().rev() {
			if elem.0 == id {
				return Some(&mut elem.1);
			}
		}
		None
	}

	/// Finds all the functions associated to the given identifier, if any, and
	/// passes each one of them, first to last, to the supplied callback.
	pub fn find_all<C: Fn(&mut F)>(&mut self, id: K, callback: C) {
		for elem in self.elems.iter_mut() {
			if elem.0 == id {
				callback(&mut elem.1);
			}
		}
	}

	/// Tells whether no functions have been added.
	pub fn is_empty(&self) -> bool {
		self.elems.is_empty()
	}
}
