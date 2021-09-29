use crate::aliases::WinResult;
use crate::co;
use crate::funcs::{GetFileVersionInfo, VarQueryValue};
use crate::structs::{LANGID, VS_FIXEDFILEINFO};
use crate::various::WString;

/// Retrieves data from an embedded resource, which can be read from an
/// executable file or a DLL.
///
/// # Examples
///
/// Reading version information:
///
/// ```rust,ignore
/// use winsafe::{HINSTANCE, ResourceInfo};
///
/// let exe_name = HINSTANCE::NULL.GetModuleFileName()?;
/// let ri = ResourceInfo::read_from(&exe_name)?;
///
/// if let Some(ffi) = ri.fixed_file_info() {
///     let ver = ffi.dwFileVersion();
///     println!("Version: {}.{}.{}.{}",
///         ver[0], ver[1], ver[2], ver[3]);
/// }
/// ```
///
/// Reading information strings. An embedded resource can have multiple string
/// blocks, and each block is identified by a language/code page pair. Each
/// block can have their own information strings:
///
/// ```rust,ignore
/// use winsafe::{HINSTANCE, ResourceInfo};
///
/// let exe_name = HINSTANCE::NULL.GetModuleFileName()?;
/// let ri = ResourceInfo::read_from(&exe_name)?;
///
/// if let Some(langs_cps) = ri.langs_and_code_pages() {
///     for (lang, code_page) in langs_cps.iter() {
///
///         if let Some(product_name) = ri.product_name(*lang, *code_page) {
///             println!("Product name: {}", product_name);
///         }
///
///         if let Some(copyright) = ri.legal_copyright(*lang, *code_page) {
///             println!("Copyright: {}", copyright);
///         }
///
///     }
/// }
/// ```
pub struct ResourceInfo {
	res_buf: Vec<u8>,
}

macro_rules! pub_fn_string_info {
	($fun:ident, $name:expr) => {
		/// Retrieves the version information string, if any.
		pub fn $fun(&self, lang: LANGID, code_page: co::CP) -> Option<String> {
			self.generic_string_info(lang, code_page, $name)
		}
	};
}

impl ResourceInfo {
	/// Reads and stores the resource data from an executable file or a DLL.
	pub fn read_from(exe_file: &str) -> WinResult<ResourceInfo> {
		Ok(Self { res_buf: GetFileVersionInfo(exe_file)? })
	}

	/// Returns the version information, if any.
	pub fn fixed_file_info(&self) -> Option<&VS_FIXEDFILEINFO> {
		unsafe {
			VarQueryValue::<VS_FIXEDFILEINFO>(&self.res_buf, "\\")
				.ok()
				.map(|(pvsf, _)| &*pvsf)
		}
	}

	/// Return the languages and code pages available in the resource.
	pub fn langs_and_code_pages(&self) -> Option<&[(LANGID, co::CP)]> {
		unsafe {
			VarQueryValue::<(LANGID, co::CP)>(&self.res_buf, "\\VarFileInfo\\Translation")
				.ok()
				.map(|(plangs, sz)| {
					std::slice::from_raw_parts(
						plangs,
						sz as usize / std::mem::size_of::<(LANGID, co::CP)>(),
					)
				})
		}
	}

	pub_fn_string_info!(comments, "Comments");
	pub_fn_string_info!(company_name, "CompanyName");
	pub_fn_string_info!(file_description, "FileDescription");
	pub_fn_string_info!(file_version, "FileVersion");
	pub_fn_string_info!(internal_name, "InternalName");
	pub_fn_string_info!(legal_copyright, "LegalCopyright");
	pub_fn_string_info!(legal_trademarks, "LegalTrademarks");
	pub_fn_string_info!(original_filename, "OriginalFilename");
	pub_fn_string_info!(product_name, "ProductName");
	pub_fn_string_info!(product_version, "ProductVersion");
	pub_fn_string_info!(private_build, "PrivateBuild");
	pub_fn_string_info!(special_build, "SpecialBuild");

	fn generic_string_info(&self,
		lang: LANGID, code_page: co::CP, info: &str) -> Option<String>
	{
		unsafe {
			VarQueryValue::<u16>(
				&self.res_buf,
				&format!("\\StringFileInfo\\{:04x}{:04x}\\{}",
					u16::from(lang), u16::from(code_page), info),
			).ok()
				.map(|(pstr, len)| {
					WString::from_wchars_slice(
						std::slice::from_raw_parts(pstr, len as _),
					).to_string()
				})
		}
	}
}
