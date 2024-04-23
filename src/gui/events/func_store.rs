/// Stores closures, associating them with an identifier.
pub(in crate::gui) struct FuncStore<K: Copy + Eq, F> {
	elems: Vec<(K, F)>,
}

impl<K: Copy + Eq, F> FuncStore<K, F> {
	/// Creates a new, empty store.
	#[must_use]
	pub(in crate::gui) const fn new() -> Self {
		Self { elems: Vec::new() }
	}

	/// Adds a new function into the store, associated to the given identifier.
	pub(in crate::gui) fn push(&mut self, id: K, func: F) {
		if self.elems.is_empty() {
			self.elems.reserve(16); // arbitrary, prealloc for speed
		}
		self.elems.push((id, func));
	}

	/// Returns an iterator over all the functions associated to the given
	/// identifier, if any.
	#[must_use]
	pub(in crate::gui) fn filter(&self, id: K) -> impl Iterator<Item = &F> {
		// https://depth-first.com/articles/2020/06/22/returning-rust-iterators
		self.elems.iter()
			.filter(move |(elem_id, _)| *elem_id == id)
			.map(|(_, func)| func)
	}

	/// Returns an iterator, in reverse order, over all the functions associated
	/// to the given identifier, if any.
	#[must_use]
	pub(in crate::gui) fn filter_rev(&self, id: K) -> impl Iterator<Item = &F> {
		self.elems.iter()
			.rev()
			.filter(move |(elem_id, _)| *elem_id == id)
			.map(|(_, func)| func)
	}

	/// Tells whether no functions have been added.
	#[must_use]
	pub(in crate::gui) fn is_empty(&self) -> bool {
		self.elems.is_empty()
	}

	/// Removes all identifiers and closures.
	pub(in crate::gui) fn clear(&mut self) {
		self.elems.clear();
	}
}
