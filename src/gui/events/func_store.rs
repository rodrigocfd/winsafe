use crate::gui::very_unsafe_cell::VeryUnsafeCell;

/// Stores closures, associating them with an identifier.
pub(in crate::gui) struct FuncStore<K: Copy + Eq, F> {
	elems: VeryUnsafeCell<Vec<(K, F)>>,
}

impl<K: Copy + Eq, F> FuncStore<K, F> {
	/// Creates a new, empty store.
	pub(in crate::gui) fn new() -> FuncStore<K, F> {
		Self {
			elems: VeryUnsafeCell::new(Vec::default()),
		}
	}

	/// Inserts a new function into the store, associated to the given
	/// identifier.
	pub(in crate::gui) fn insert(&self, id: K, func: F) {
		let elems = self.elems.as_mut();
		if elems.is_empty() {
			elems.reserve(16); // arbitrary, prealloc for speed
		}
		elems.push((id, func));
	}

	/// Finds the last added function associated to the given identifier, if
	/// any.
	pub(in crate::gui) fn find(&self, id: K) -> Option<&F> {
		// Linear search, more performant for small collections.
		// Searches backwards, so the function added last will be chosen.
		self.elems.iter().rev()
			.find(move |elem| elem.0 == id)
			.map(|elem| &elem.1)
	}

	/// Finds all the functions associated to the given identifier, if any, and
	/// returns an iterator to t
	pub(in crate::gui) fn find_all(&self, id: K) -> impl Iterator<Item = &F> {
		// https://depth-first.com/articles/2020/06/22/returning-rust-iterators
		self.elems.iter()
			.filter(move |elem| elem.0 == id)
			.map(|elem| &elem.1)
	}

	/// Tells whether no functions have been added.
	pub(in crate::gui) fn is_empty(&self) -> bool {
		self.elems.is_empty()
	}
}
