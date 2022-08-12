use crate::co;

/// A [`Result` alias](crate#errors-and-result-aliases) for Win32 COM
/// operations, which returns an [`HRESULT`](crate::co::HRESULT) on failure.
///
/// # Examples
///
/// Converting into the generic [`AnyResult`](crate::AnyResult):
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// use winsafe::{co, AnyResult, HrResult};
///
/// let hr_result: HrResult<()> = Err(co::HRESULT::E_INVALIDARG);
///
/// let err_result: AnyResult<()> = hr_result.map_err(|err| err.into());
/// ```
///
/// Converting from an [`WinResult`](crate::WinResult):
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// use winsafe::{co, HrResult, WinResult};
///
/// let win_result: WinResult<()> = Err(co::ERROR::FILE_NOT_FOUND);
///
/// let hr_result: HrResult<()> = win_result.map_err(|err| err.to_hresult());
/// ```
#[cfg_attr(docsrs, doc(cfg(feature = "ole")))]
pub type HrResult<T> = Result<T, co::HRESULT>;
