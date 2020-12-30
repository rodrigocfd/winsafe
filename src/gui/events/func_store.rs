/// Stores closures, associating them with an identifier.
pub struct FuncStore<K, F> {
	elems: Vec<(K, F)>,
}

impl<K: Eq, F> FuncStore<K, F> {
	/// Creates a new, empty store.
	pub fn new() -> FuncStore<K, F> {
		Self {
			elems: Vec::new(),
		}
	}

	/// Inserts a new function into the store, associated to the given identifier.
	pub fn insert(&mut self, id: K, func: F) {
		self.elems.push((id, func));
	}

	/// Finds the function associated to the given identifier, if any.
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
}
