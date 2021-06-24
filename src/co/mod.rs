//! Win32 constants and types of constants.
//!
//! All types implement bitflag operations, and can be converted to its
//! underlying integer type.

#[macro_use]
mod macros;

mod consts_ab;
mod consts_cd;
mod consts_ef;
mod consts_ghij;
mod consts_kl;
mod consts_mn;
mod consts_opqr;
mod consts_s;
mod consts_t;
mod consts_uv;
mod consts_wxyz;
mod error;

pub use consts_ab::*;
pub use consts_cd::*;
pub use consts_ef::*;
pub use consts_ghij::*;
pub use consts_kl::*;
pub use consts_mn::*;
pub use consts_opqr::*;
pub use consts_s::*;
pub use consts_t::*;
pub use consts_uv::*;
pub use consts_wxyz::*;
pub use error::ERROR;
