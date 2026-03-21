use crate::bind::acl::Acl;
use crate::bind::view::View;
use crate::error::Result;
use crate::utils::{parse_kdl, resolve_paths};
use crate::zone::Zone;
use std::ffi::OsStr;
use std::fs;
use std::path::Path;

#[derive(Debug, knus::Decode)]
pub enum Include {
	Zone(Zone),
	Acl(Acl),
	View(View),
}

impl Include {
	fn is_extension_allowed(extension: Option<&OsStr>) -> bool {
		let allowed_extensions = ["kdl"];
		extension
			.and_then(|ext| ext.to_str())
			.is_some_and(|ext_str| allowed_extensions.contains(&ext_str))
	}

	pub fn parse<T: FnMut(Include) -> Result<()>>(
		config_path: &Path,
		include_list: &[String],
		mut func: T,
	) -> Result<()> {
		let resolved_paths = resolve_paths(config_path, include_list)?;

		for path in resolved_paths {
			if path.is_file() && Self::is_extension_allowed(path.extension()) {
				let contents = fs::read_to_string(&path)?;
				for doc in parse_kdl::<Vec<Include>>(&contents)? {
					func(doc)?
				}
			}
		}

		Ok(())
	}
}
