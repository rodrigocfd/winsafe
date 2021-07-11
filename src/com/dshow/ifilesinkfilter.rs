#![allow(non_snake_case)]

use crate::com::iunknown::IUnknownVT;
use crate::com::traits::{ComInterface, PPComVT};
use crate::ffi::{HRESULT, PCSTR, PCVOID, PSTR, PVOID};
use crate::structs::IID;

type PP = PPComVT<IUnknownVT>;

/// [`IFileSinkFilter`](crate::dshow::IFileSinkFilter) virtual table.
pub struct IFileSinkFilterVT {
	pub IUnknownVT: IUnknownVT,
	pub SetFileName: fn(PP, PCSTR, PCVOID) -> HRESULT,
	pub GetCurFile: fn(PP, *mut PSTR, PVOID) -> HRESULT,
}

/// [`IFileSinkFilter`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nn-strmif-ifilesinkfilter)
/// COM interface over
/// [`IFileSinkFilterVT`](crate::dshow::vt::IFileSinkFilterVT). Inherits
/// from [`IUnknown`](crate::IUnknown).
///
/// Automatically calls
/// [`IUnknown::Release`](https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
/// when the object goes out of scope.
pub struct IFileSinkFilter {
	pub(crate) ppvt: PPComVT<IUnknownVT>,
}

impl_send_sync_fromppvt!(IFileSinkFilter);

impl ComInterface for IFileSinkFilter {
	const IID: IID = IID::new(0xa2104830, 0x7c70, 0x11cf, 0x8bce, 0x00aa00a3f1a6);
}

macro_rules! impl_IFileSinkFilter {
	($name:ty, $vt:ty) => {
		use crate::com::dshow::AM_MEDIA_TYPE;
		use crate::com::funcs::CoTaskMemFree;
		use crate::various::WString;

		impl $name {
			fn ifilesinkfilter_vt(&self) -> &IFileSinkFilterVT {
				unsafe { &**(self.ppvt as PPComVT<_>) }
			}

			/// [`IFileSinkFilter::GetCurFile`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-ifilesinkfilter-getcurfile)
			/// method.
			///
			/// If you pass a [`AM_MEDIA_TYPE`](crate::dshow::AM_MEDIA_TYPE)
			/// reference to `pmt`, its `pbFormat` field may return a valid
			/// reference to a format block. If so, you must free it with
			/// [`CoTaskMemFree`](crate::CoTaskMemFree), or you'll have a memory
			/// leak.
			///
			/// # Examples
			///
			/// ```rust,ignore
			/// use winsafe::{CoMemTaskFree, dshow};
			///
			/// let isink: dshow::IFileSinkFilter; // initialized somewhere
			///
			/// let mut ammt = dshow::AM_MEDIA_TYPE::default();
			/// unsafe {
			///     isink.GetCurFile(Some(&mut ammt)).unwrap();
			///     if let Some(pb_format) = ammt.pbFormat::<dshow::DVINFO>() {
			///         CoTaskMemFree(pb_format);
			///     }
			/// }
			/// ```
			pub unsafe fn GetCurFile(&self,
				pmt: Option<&mut AM_MEDIA_TYPE>) -> WinResult<String>
			{
				let mut pstr: *mut u16 = std::ptr::null_mut();
				hr_to_winresult(
					(self.ifilesinkfilter_vt().GetCurFile)(
						self.ppvt,
						&mut pstr,
						pmt.map_or(std::ptr::null_mut(), |p| p as *mut _ as _),
					),
				).map(|_| {
					let name = WString::from_wchars_nullt(pstr);
					CoTaskMemFree(pstr);
					name.to_string()
				})
			}

			/// [`IFileSinkFilter::SetFileName`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-ifilesinkfilter-setfilename)
			/// method.
			pub fn SetFileName(&self,
				pszFileName: &str, pmt: Option<&AM_MEDIA_TYPE>) -> WinResult<()>
			{
				hr_to_winresult(
					(self.ifilesinkfilter_vt().SetFileName)(
						self.ppvt,
						unsafe { WString::from_str(pszFileName).as_ptr() },
						pmt.map_or(std::ptr::null(), |p| p as *const _ as _),
					),
				)
			}
		}
	};
}

impl_IUnknown!(IFileSinkFilter, IFileSinkFilterVT);
impl_IFileSinkFilter!(IFileSinkFilter, IFileSinkFilterVT);
