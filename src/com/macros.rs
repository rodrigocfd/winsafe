/// Implements `ComVT` trait, associating with an `IID`.
macro_rules! impl_iid {
	($name:ident, $iid1:expr, $iid2:expr, $iid3:expr, $iid4:expr, $iid5:expr) => {
		impl ComVT for $name {
			fn IID() -> IID {
				IID::new($iid1, $iid2, $iid3, $iid4, $iid5)
			}
		}
	};
}
