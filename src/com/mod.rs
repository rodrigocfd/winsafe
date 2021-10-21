#[macro_use] mod macros;

#[cfg(feature = "autom")] pub mod autom;
#[cfg(feature = "dshow")] pub mod dshow;
#[cfg(feature = "idl")]   pub mod idl;
#[cfg(feature = "shell")] pub mod shell;

mod funcs;
mod iunknown;

pub use funcs::*;
pub use iunknown::{ComPtr, IUnknown, IUnknownVT};

pub(crate) mod prelude {
	pub use super::iunknown::{ComInterface, IUnknownT};
	#[cfg(feature = "autom")] pub use super::autom::prelude::*;
	#[cfg(feature = "dshow")] pub use super::dshow::prelude::*;
	#[cfg(feature = "idl")]   pub use super::idl::prelude::*;
	#[cfg(feature = "shell")] pub use super::shell::prelude::*;
}
