use crate::co;

/// A specialized
/// [`Result`](https://doc.rust-lang.org/std/result/enum.Result.html) for Win32
/// COM operations, which returns an [`HRESULT`](crate::co::HRESULT) on failure.
#[cfg_attr(docsrs, doc(cfg(feature = "ole")))]
pub type HrResult<T> = Result<T, co::HRESULT>;
