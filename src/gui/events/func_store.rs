/// Stores closures, associating them with an identifier.
pub(in crate::gui) struct FuncStore<K: Copy + Eq, F> {
	elems: Vec<(K, F)>,
}

impl<K: Copy + Eq, F> FuncStore<K, F> {
	/// Creates a new, empty store.
	pub fn new() -> FuncStore<K, F> {
		Self {
			elems: Vec::default(),
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
	pub fn find(&self, id: K) -> Option<&F> {
		// Linear search, more performant for small collections.
		// Searches backwards, so the function added last will be chosen.
		self.elems.iter().rev()
			.find(move |elem| elem.0 == id)
			.map(|elem| &elem.1)
	}

	/// Finds all the functions associated to the given identifier, if any, and
	/// returns an iterator to t
	pub fn find_all(&self, id: K) -> impl Iterator<Item = &F> {
		// https://depth-first.com/articles/2020/06/22/returning-rust-iterators
		self.elems.iter()
			.filter(move |elem| elem.0 == id)
			.map(|elem| &elem.1)
	}

	/// Tells whether no functions have been added.
	pub fn is_empty(&self) -> bool {
		self.elems.is_empty()
	}
}
