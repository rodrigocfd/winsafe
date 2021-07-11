/// Implements the conversion from `PPI` to the COM type itself.
macro_rules! impl_send_sync_fromppvt {
	($name:ty) => {
		unsafe impl Send for $name {}
		unsafe impl Sync for $name {}

		impl From<PPI> for $name {
			fn from(ppvt: PPI) -> Self {
				Self { ppvt }
			}
		}
	};
}

/// Creates multiple `GUID`-derived pub const values.
#[allow(unused_macros)]
macro_rules! pub_const_guid {
	(
		$type:ident,
		$($name:ident, $iid1:expr, $iid2:expr, $iid3:expr, $iid4:expr, $iid5:expr,)*
	) => {
		$(
			pub const $name: $type = $type::new($iid1, $iid2, $iid3, $iid4, $iid5);
		)*
	};
}
