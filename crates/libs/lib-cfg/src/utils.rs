use crate::error::Result;
use glob::glob;
use knus::DecodeChildren;
use knus::span::Span;
use std::path::{Path, PathBuf};

pub fn parse_kdl<T>(s: &str) -> Result<T>
where
	T: DecodeChildren<Span>,
{
	let contents = kdl::KdlDocument::v2_to_v1(s)?;

	let config = knus::parse::<T>("", &contents)?;

	Ok(config)
}

pub fn write_kdl<T>(s: &str) -> Result<T>
where
	T: DecodeChildren<Span>,
{
	let contents = kdl::KdlDocument::v2_to_v1(s)?;

	let config = knus::parse::<T>("", &contents)?;

	Ok(config)
}

pub fn resolve_paths(config_path: &Path, patterns: &[String]) -> Result<Vec<PathBuf>> {
	let config_dir = config_path
		.canonicalize()?
		.parent()
		.ok_or(std::io::Error::other(
			"Could not get parent directory of config file",
		))?
		.to_path_buf();

	let mut resolved_paths = Vec::new();

	for pattern in patterns {
		let path = PathBuf::from(pattern);

		let absolute_path = if path.is_relative() {
			config_dir.join(path)
		} else {
			path
		};

		if absolute_path.exists() && absolute_path.is_file() {
			resolved_paths.push(absolute_path);
		} else {
			let pattern_str = absolute_path.to_string_lossy();
			for path in glob(&pattern_str)?.flatten() {
				if path.exists() && path.is_file() {
					resolved_paths.push(path);
				}
			}
		}
	}

	Ok(resolved_paths)
}
