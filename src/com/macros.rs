/// Declares a COM virtual table struct, and implements the `ComVT` trait.
macro_rules! pub_struct_vtable {
	(
		$name:ident,
		$(#[$doc:meta])*
		=>
		$iid1:expr, $iid2:expr, $iid3:expr, $iid4:expr, $iid5:expr,
		$($member:ident, $descr:ty)*
	) => {
		$(#[$doc])*
		#[repr(C)]
		pub struct $name {
			$(
				pub $member: $descr,
			)*
		}

		impl crate::com::ComVT for $name {
			fn IID() -> crate::structs::IID {
				crate::structs::IID::new($iid1, $iid2, $iid3, $iid4, $iid5)
			}
		}
	};
}

/// Creates multiple `GUID`-derived pub const values.
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
