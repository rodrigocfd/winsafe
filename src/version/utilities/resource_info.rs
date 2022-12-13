use crate::co;
use crate::kernel::decl::{LANGID, SysResult, WString};
use crate::version::decl::{GetFileVersionInfo, VarQueryValue, VS_FIXEDFILEINFO};

/// Retrieves data from an embedded resource, which can be read from an
/// EXE or a DLL file.
///
/// # Examples
///
/// Reading version information:
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// use winsafe::{HINSTANCE, ResourceInfo};
///
/// let exe_name = HINSTANCE::NULL.GetModuleFileName()?;
/// let res_info = ResourceInfo::read_from(&exe_name)?;
///
/// if let Some(ver_info) = res_info.version_info() {
///     let ver = ver_info.dwFileVersion();
///     println!("Version: {}.{}.{}.{}",
///         ver[0], ver[1], ver[2], ver[3]);
/// }
/// # Ok::<_, winsafe::co::ERROR>(())
/// ```
///
/// Reading information strings. An embedded resource can have multiple string
/// blocks, and each block is identified by a language/code page pair. Each
/// block can have their own information strings:
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// use winsafe::{HINSTANCE, ResourceInfo};
///
/// let exe_name = HINSTANCE::NULL.GetModuleFileName()?;
/// let res_info = ResourceInfo::read_from(&exe_name)?;
///
/// for block in res_info.blocks() {
///     if let Some(product_name) = block.product_name() {
///         println!("Product name: {}", product_name);
///     }
///     if let Some(copyright) = block.legal_copyright() {
///         println!("Copyright: {}", copyright);
///     }
/// }
/// # Ok::<_, winsafe::co::ERROR>(())
/// ```
pub struct ResourceInfo {
	res_buf: Vec<u8>,
}

impl ResourceInfo {
	/// Reads and stores the resource data from an executable file or a DLL.
	#[must_use]
	pub fn read_from(exe_file: &str) -> SysResult<ResourceInfo> {
		Ok(Self { res_buf: GetFileVersionInfo(exe_file)? })
	}

	/// Returns an iterator over the language blocks, if any, which are composed
	/// of a language ID and a code page.
	///
	/// These blocks allow retrieval of version information strings in their
	/// respective languages.
	#[must_use]
	pub fn blocks<'a>(&'a self) -> impl Iterator<Item = ResourceInfoBlock> + 'a {
		unsafe {
			VarQueryValue::<(LANGID, co::CP)>(&self.res_buf, "\\VarFileInfo\\Translation")
				.ok()
				.map(|(plangs, sz)|
					std::slice::from_raw_parts(
						plangs,
						sz as usize / std::mem::size_of::<(LANGID, co::CP)>(),
					)
				)
				.unwrap_or(&[])
				.iter()
				.map(|(lang_id, code_page)|
					ResourceInfoBlock {
						res_info: self,
						lang_id: *lang_id,
						code_page: *code_page,
					}
				)
		}
	}

	/// Returns the version information, if any.
	#[must_use]
	pub fn version_info(&self) -> Option<&VS_FIXEDFILEINFO> {
		unsafe {
			VarQueryValue::<VS_FIXEDFILEINFO>(&self.res_buf, "\\")
				.ok()
				.map(|(pvsf, _)| &*pvsf)
		}
	}
}

/// An language block of [`ResourceInfo`](crate::ResourceInfo), composed of a
/// language ID and a code page.
pub struct ResourceInfoBlock<'a> {
	res_info: &'a ResourceInfo,
	lang_id: LANGID,
	code_page: co::CP,
}

impl<'a> ResourceInfoBlock<'a> {
	#[must_use] pub const fn lang_id(&self) -> LANGID { self.lang_id }
	#[must_use] pub const fn code_page(&self) -> co::CP { self.code_page }

	#[must_use] pub fn comments(&self) -> Option<String> { self.generic_string_info("Comments") }
	#[must_use] pub fn company_name(&self) -> Option<String> { self.generic_string_info("CompanyName") }
	#[must_use] pub fn file_description(&self) -> Option<String> { self.generic_string_info("FileDescrition") }
	#[must_use] pub fn file_version(&self) -> Option<String> { self.generic_string_info("FileVersion") }
	#[must_use] pub fn internal_name(&self) -> Option<String> { self.generic_string_info("InternalName") }
	#[must_use] pub fn legal_copyright(&self) -> Option<String> { self.generic_string_info("LegalCopyright") }
	#[must_use] pub fn legal_trademarks(&self) -> Option<String> { self.generic_string_info("LegalTrademarks") }
	#[must_use] pub fn original_filename(&self) -> Option<String> { self.generic_string_info("OriginalFilename") }
	#[must_use] pub fn product_name(&self) -> Option<String> { self.generic_string_info("ProductName") }
	#[must_use] pub fn product_version(&self) -> Option<String> { self.generic_string_info("ProductVersion") }
	#[must_use] pub fn private_build(&self) -> Option<String> { self.generic_string_info("PrivateBuild") }
	#[must_use] pub fn special_build(&self) -> Option<String> { self.generic_string_info("SpecialBuild") }

	fn generic_string_info(&self, info: &str) -> Option<String> {
		unsafe {
			VarQueryValue::<u16>(
				&self.res_info.res_buf,
				&format!("\\StringFileInfo\\{:04x}{:04x}\\{}",
					u16::from(self.lang_id), u16::from(self.code_page), info),
			).ok()
				.map(|(pstr, len)| {
					WString::from_wchars_slice(
						std::slice::from_raw_parts(pstr, len as _),
					).to_string()
				})
		}
	}
}
