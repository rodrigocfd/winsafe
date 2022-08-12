use crate::co;

/// A [`Result` alias](crate#errors-and-result-aliases) which returns a `Box<dyn
/// Error + Send + Sync>` on failure.
///
/// This is the most generic
/// [`Result`](https://doc.rust-lang.org/std/result/enum.Result.html) possible –
/// any other `Result` can be converted into it.
#[cfg_attr(docsrs, doc(cfg(feature = "kernel")))]
pub type AnyResult<T> = Result<T, Box<dyn std::error::Error + Send + Sync>>;

/// A [`Result` alias](crate#errors-and-result-aliases) for Win32 operations,
/// which returns an [`ERROR`](crate::co::ERROR) on failure.
///
/// # Examples
///
/// Converting into the generic [`AnyResult`](crate::AnyResult):
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// use winsafe::{co, AnyResult, WinResult};
///
/// let win_result: WinResult<()> = Err(co::ERROR::SUCCESS);
///
/// let err_result: AnyResult<()> = win_result.map_err(|err| err.into());
/// ```
#[cfg_attr(docsrs, doc(cfg(feature = "kernel")))]
pub type WinResult<T> = Result<T, co::ERROR>;
