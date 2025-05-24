#![allow(non_camel_case_types, non_snake_case)]

use crate::prelude::*;

handle! { HMODULE;
	/// Handle to a
	/// module.
	/// Originally just a `HANDLE`.
}

impl kernel_Hmodule for HMODULE {}

/// This trait is enabled with the `kernel` feature, and provides methods for
/// [`HMODULE`](crate::HMODULE).
///
/// Prefer importing this trait through the prelude:
///
/// ```no_run
/// use winsafe::prelude::*;
/// ```
pub trait kernel_Hmodule: Handle {
    
}