/// Implements `Vtbl` trait, associating with an `IID`.
macro_rules! impl_iid {
	($name:ident, $iid1:expr, $iid2:expr, $iid3:expr, $iid4:expr, $iid5:expr) => {
		impl ComVT for $name {
			fn IID() -> IID {
				IID::new($iid1, $iid2, $iid3, $iid4, $iid5)
			}
		}
	};
}

/// Converts an `HRESULT` into a `WinResult<()>`.
macro_rules! into_result {
	($hresult:expr) => {
		match ERROR($hresult) {
			ERROR::S_OK => Ok(()),
			err => Err(err),
		}
	};
}
