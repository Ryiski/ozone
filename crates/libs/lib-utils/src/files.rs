use std::fs::File;
use std::path::{Path, PathBuf};
use std::{fs, io};

pub fn file_exists(path: &Path) -> bool {
	path.is_file()
}

pub fn directory_exists(path: &Path) -> io::Result<bool> {
	match fs::metadata(path) {
		Ok(meta) => Ok(meta.is_dir()),
		Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(false),
		Err(e) => Err(e),
	}
}
pub fn is_dir_writable(path: &Path) -> bool {
	let test_path: PathBuf = path.join(".perm_check");

	match File::create(&test_path) {
		Ok(_) => {
			let _ = fs::remove_file(test_path);
			true
		}
		Err(_) => false,
	}
}

pub fn create_directory(path: &Path) -> io::Result<()> {
	if !path.exists() {
		tracing::info!("Creating directory {}", path.display());
		return fs::create_dir_all(path);
	}

	Ok(())
}
