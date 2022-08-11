use crate::gui::very_unsafe_cell::VeryUnsafeCell;

/// An identifier and a closure.
struct Pair<K: Copy + Eq, F> {
	id: K,
	func: F,
}

/// Stores closures, associating them with an identifier.
pub(in crate::gui) struct FuncStore<K: Copy + Eq, F> {
	elems: VeryUnsafeCell<Vec<Pair<K, F>>>,
}

impl<K: Copy + Eq, F> FuncStore<K, F> {
	/// Creates a new, empty store.
	pub(in crate::gui) fn new() -> Self {
		Self { elems: VeryUnsafeCell::new(Vec::default()) }
	}

	/// Adds a new function into the store, associated to the given identifier.
	pub(in crate::gui) fn push(&self, id: K, func: F) {
		let elems = self.elems.as_mut();
		if elems.is_empty() {
			elems.reserve(16); // arbitrary, prealloc for speed
		}
		elems.push(Pair { id, func });
	}

	/// Finds the last added function associated to the given identifier, if
	/// any.
	pub(in crate::gui) fn find(&self, id: K) -> Option<&F> {
		// Linear search, more performant for small collections.
		// Searches backwards, so the function added last will be chosen.
		self.elems.iter().rev()
			.find(move |elem| elem.id == id)
			.map(|elem| &elem.func)
	}

	/// Finds all the functions associated to the given identifier, if any, and
	/// returns an iterator to it.
	pub(in crate::gui) fn find_all(&self, id: K) -> impl Iterator<Item = &F> {
		// https://depth-first.com/articles/2020/06/22/returning-rust-iterators
		self.elems.iter()
			.filter(move |elem| elem.id == id)
			.map(|elem| &elem.func)
	}

	/// Tells whether no functions have been added.
	pub(in crate::gui) fn is_empty(&self) -> bool {
		self.elems.is_empty()
	}
}
