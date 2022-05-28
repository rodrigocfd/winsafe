use crate::co;

/// A specialized
/// [`Result`](https://doc.rust-lang.org/std/result/enum.Result.html) for Win32
/// COM operations, which returns an [`HRESULT`](crate::co::HRESULT) on failure.
///
/// # Examples
///
/// Converting into the generic [`ErrResult`](crate::ErrResult):
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// use winsafe::{co, ErrResult, HrResult};
///
/// let hr_result: HrResult<()> = Err(co::HRESULT::S_OK);
///
/// let err_result: ErrResult<()> = hr_result.map_err(|err| err.into());
/// ```
#[cfg_attr(docsrs, doc(cfg(feature = "ole")))]
pub type HrResult<T> = Result<T, co::HRESULT>;
