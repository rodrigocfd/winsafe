use crate::IID;

/// Trait for any COM interface.
pub trait ComInterface {
	fn Iid() -> IID;
}