/// Declares a COM virtual table struct, and implements the `ComVT` trait.
macro_rules! com_virtual_table {
	(
		$name:ident,
		$(#[$doc:meta])*
		->
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
