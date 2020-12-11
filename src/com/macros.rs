/// Declares a COM virtual table type.
macro_rules! vtbl_type {
	(
		$(#[$attr:meta])*
		$name:ident,
		$iid1:expr, $iid2:expr, $iid3:expr, $iid4:expr, $iid5:expr,
		$($cname:ident, $cval:ty)*
	) => {
		$(#[$attr])*
		#[repr(C)]
		pub struct $name {
			$($cname: $cval,)*
		}

		impl Vtbl for $name {
			fn IID() -> IID {
				IID::new($iid1, $iid2, $iid3, $iid4, $iid5)
			}
		}
	};
}