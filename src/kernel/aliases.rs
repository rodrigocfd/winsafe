use crate::co;

/// A specialized
/// [`Result`](https://doc.rust-lang.org/std/result/enum.Result.html) which
/// returns a `Box<dyn Error + Send + Sync>` on failure.
#[cfg_attr(docsrs, doc(cfg(feature = "kernel")))]
pub type ErrResult<T> = Result<T, Box<dyn std::error::Error + Send + Sync>>;

/// A specialized
/// [`Result`](https://doc.rust-lang.org/std/result/enum.Result.html) for Win32
/// operations, which returns an [`ERROR`](crate::co::ERROR) on failure.
#[cfg_attr(docsrs, doc(cfg(feature = "kernel")))]
pub type WinResult<T> = Result<T, co::ERROR>;
