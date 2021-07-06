#![allow(non_snake_case)]

macro_rules! pub_struct_IFileSinkFilter {
	(
		$(#[$doc:meta])*
		$name:ident, $vt:ty
	) => {
		use crate::com::dshow::AM_MEDIA_TYPE;
		use crate::com::dshow::vt::IFileSinkFilterVT;
		use crate::com::funcs::CoTaskMemFree;
		use crate::various::WString;

		pub_struct_IUnknown! {
			$(#[$doc])*
			$name, $vt
		}

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

pub_struct_IFileSinkFilter! {
	/// [`IFileSinkFilter`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nn-strmif-ifilesinkfilter)
	/// COM interface over
	/// [`IFileSinkFilterVT`](crate::dshow::vt::IFileSinkFilterVT). Inherits
	/// from [`IUnknown`](crate::IUnknown).
	///
	/// Automatically calls
	/// [`IUnknown::Release`](https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
	IFileSinkFilter, crate::com::dshow::vt::IFileSinkFilterVT
}
