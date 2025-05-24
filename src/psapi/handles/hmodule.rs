#![allow(non_camel_case_types, non_snake_case)]

use crate::decl::*;
use crate::prelude::*;

impl psapi_Hmodule for HMODULE {}

/// This trait is enabled with the `psapi` feature, and provides methods for
/// [`HMODULE`](crate::HMODULE).
///
/// Prefer importing this trait through the prelude:
///
/// ```no_run
/// use winsafe::prelude::*;
/// ```
pub trait psapi_Hmodule: kernel_Hmodule {

}