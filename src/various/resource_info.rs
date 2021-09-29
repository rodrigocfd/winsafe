use crate::aliases::WinResult;
use crate::co;
use crate::funcs::{GetFileVersionInfo, VarQueryValue};
use crate::structs::{LANGID, VS_FIXEDFILEINFO};
use crate::various::WString;

/// Retrieves data from an embedded resource, which can be read from an
/// executable file or a DLL.
pub struct ResourceInfo {
	res_buf: Vec<u8>,
}

macro_rules! pub_fn_string_info {
	($fun:ident, $name:expr) => {
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

	/// Returns the version information.
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
