//! Native Win32 constants and types of constants.
//!
//! All types implement the [`NativeConstant`](crate::co::NativeConstant) trait
//! and bitflag operations, and they can be converted from/to their underlying
//! integer type.
//!
//! Among these constant types, three are error types:
//! [`CDERR`](crate::co::CDERR), [`ERROR`](crate::co::ERROR) and
//! [`HRESULT`](crate::co::HRESULT).

#[macro_use] mod macros;

mod consts_ab;
mod consts_c;
mod consts_d;
mod consts_ef;
mod consts_ghij;
mod consts_kl;
mod consts_mn;
mod consts_opqr;
mod consts_s;
mod consts_t;
mod consts_uv;
mod consts_wxyz;
mod e_cderr;
mod e_error;
mod e_hresult;
pub(crate) mod traits;
mod vs;

pub use consts_ab::*;
pub use consts_c::*;
pub use consts_d::*;
pub use consts_ef::*;
pub use consts_ghij::*;
pub use consts_kl::*;
pub use consts_mn::*;
pub use consts_opqr::*;
pub use consts_s::*;
pub use consts_t::*;
pub use consts_uv::*;
pub use consts_wxyz::*;
pub use e_cderr::CDERR;
pub use e_error::ERROR;
pub use e_hresult::HRESULT;
pub use traits::*;
pub use vs::VS;

pub(crate) mod prelude {
	pub use super::traits::*;
}
