use crate::co;

/// A [`Result` alias](crate#errors-and-result-aliases) for COM error codes,
/// which returns an [`HRESULT`](crate::co::HRESULT) on failure.
///
/// # Examples
///
/// Converting into the generic [`AnyResult`](crate::AnyResult):
///
/// ```no_run
/// use winsafe::prelude::*;
/// use winsafe::{co, AnyResult, HrResult};
///
/// let hr_result: HrResult<()> = Err(co::HRESULT::E_INVALIDARG);
///
/// let err_result: AnyResult<()> = hr_result.map_err(|err| err.into());
/// ```
///
/// Converting from an [`SysResult`](crate::SysResult):
///
/// ```no_run
/// use winsafe::prelude::*;
/// use winsafe::{co, HrResult, SysResult};
///
/// let win_result: SysResult<()> = Err(co::ERROR::FILE_NOT_FOUND);
///
/// let hr_result: HrResult<()> = win_result.map_err(|err| err.to_hresult());
/// ```
pub type HrResult<T> = Result<T, co::HRESULT>;
