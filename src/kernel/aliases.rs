use crate::co;

/// A specialized
/// [`Result`](https://doc.rust-lang.org/std/result/enum.Result.html) which
/// returns a `Box<dyn Error + Send + Sync>` on failure.
///
/// This is the most generic `Result` possible – any other `Result` can be
/// converted into it.
#[cfg_attr(docsrs, doc(cfg(feature = "kernel")))]
pub type ErrResult<T> = Result<T, Box<dyn std::error::Error + Send + Sync>>;

/// A specialized
/// [`Result`](https://doc.rust-lang.org/std/result/enum.Result.html) for Win32
/// operations, which returns an [`ERROR`](crate::co::ERROR) on failure.
///
/// # Examples
///
/// Converting into the generic [`ErrResult`](crate::ErrResult):
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// use winsafe::{co, ErrResult, WinResult};
///
/// let win_result: WinResult<()> = Err(co::ERROR::SUCCESS);
///
/// let err_result: ErrResult<()> = win_result.map_err(|err| err.into());
/// ```
#[cfg_attr(docsrs, doc(cfg(feature = "kernel")))]
pub type WinResult<T> = Result<T, co::ERROR>;
