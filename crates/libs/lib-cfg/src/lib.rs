pub mod bind;
pub mod cli;
pub mod error;
pub mod include;
pub mod records;
pub mod utils;
pub mod zone;

use crate::bind::Bind;
use crate::include::Include;
use crate::utils::parse_kdl;
use crate::zone::Adblock;
use crate::zone::Zone;
use chrono::Local;
use cli::Cli;
use error::{CfgError, Result};
use path_clean::PathClean;
use std::env;
use std::fmt;
use std::fs;
use std::io;
use std::panic;
use std::path::Path;
use std::path::PathBuf;
use std::process;
use tracing::Level;
use tracing_panic::panic_hook;
use tracing_subscriber::fmt::time::FormatTime;

#[derive(Debug, Clone, knus::Decode)]
pub struct Defaults {
	// Added fields
	pub ttl: String,
	// pub ns: String,
	// pub email: String,
	pub soa_refresh: String,
	pub soa_retry: String,
	pub soa_expire: String,
	pub soa_min_ttl: String,
}

impl Default for Defaults {
	fn default() -> Self {
		Self {
			// ttl: Default::default(),
			// ns: Default::default(),
			// email: Default::default(),
			// soa_refresh: Default::default(),
			// soa_retry: Default::default(),
			// soa_expire: Default::default(),
			// soa_min_ttl: Default::default(),
			ttl: "1h".into(),
			soa_refresh: "6h".into(),
			soa_retry: "1h".into(),
			soa_expire: "1w".into(),
			soa_min_ttl: "30m".into(),
		}
	}
}

#[derive(Default, Debug, Clone, knus::Decode)]
pub struct Config {
	pub dry_run: bool,
	pub log_level: LogLevel,
	pub defaults: Defaults,

	#[knus(child, unwrap(argument))]
	pub out_dir: String,
	#[knus(child)]
	pub adblock: Adblock,
	#[knus(child)]
	pub bind: Bind,
	#[knus(children(name = "zone"))]
	pub zones: Vec<Zone>,
	#[knus(children(name="include"), unwrap(argument), default = vec![])]
	pub includes: Vec<String>,
}

impl Config {
	pub fn load(cfg_path: &Path, cli: Option<&Cli>) -> Result<Self> {
		let contents = fs::read_to_string(cfg_path)?;

		let mut cfg = parse_kdl::<Self>(&contents)?;

		if let Some(cli) = cli {
			cfg.apply_cli(cli);
		}

		cfg.include(cfg_path)?;
		cfg.bind.include(cfg_path)?;

		tracing_subscriber::fmt()
			.with_max_level(tracing::level_filters::LevelFilter::from_level(
				cfg.log_level.into(),
			))
			.with_timer(TracingTimer)
			.with_target(false)
			.with_file(false)
			.with_line_number(false)
			.with_level(true)
			.init();

		panic::set_hook(Box::new(panic_hook));

		Ok(cfg)
	}

	pub fn out_dir(&self) -> PathBuf {
		let out_dir = PathBuf::from(&self.out_dir);
		if out_dir.is_absolute() {
			return out_dir;
		}

		match env::current_dir() {
			Ok(cwd) => cwd.join(&self.out_dir).clean(),
			Err(e) => {
				tracing::error!("Error getting CWD: {}", e);
				process::exit(1)
			}
		}
	}

	pub fn default_config_path() -> Result<PathBuf> {
		match dirs::config_dir() {
			Some(dir) => {
				let file_path = Path::new(&dir).join("ozone").join("config.kdl");

				Ok(file_path)
			}
			None => Err(CfgError::other("Failed to find config directory")),
		}
	}

	pub fn apply_cli(&mut self, cli: &Cli) {
		let Cli {
			config: _,
			verbose,
			dry_run,
			version: _,
		} = cli;

		self.dry_run = *dry_run;

		if *verbose {
			self.log_level = Level::DEBUG.into();
		}
	}

	pub fn get_default_config_path() -> String {
		if let Some(mut config_dir) = dirs::config_dir() {
			config_dir.push("ozone");
			config_dir.push("config.kdl");
			config_dir.to_string_lossy().into_owned()
		} else {
			tracing::warn!(
				"Could not determine user config directory, falling back to 'config.kdl'"
			);
			"config.kdl".to_string()
		}
	}

	pub fn validate_config_directory(&self) -> Result<()> {
		let path = self.out_dir();

		if lib_utils::files::directory_exists(&path)? {
			if lib_utils::files::is_dir_writable(&path) {
				return Ok(());
			}

			return Err(io::Error::new(
				io::ErrorKind::PermissionDenied,
				"Directory is not writable!",
			)
			.into());
		}

		// Directory doesn't exist → check parent
		let parent = path.parent().unwrap_or_else(|| Path::new("/"));

		if lib_utils::files::is_dir_writable(parent) {
			return Ok(());
		}

		Err(io::Error::new(
			io::ErrorKind::PermissionDenied,
			"Directory does NOT exist AND parent is not writable!",
		)
		.into())
	}
	pub fn include(&mut self, config_path: &Path) -> Result<()> {
		Include::parse(config_path, &self.includes, |include| {
			#[allow(irrefutable_let_patterns)]
			if let Include::Zone(zone) = include {
				self.zones.push(zone);
			}

			Ok(())
		})?;

		Ok(())
	}
}

struct TracingTimer;

impl FormatTime for TracingTimer {
	fn format_time(&self, w: &mut tracing_subscriber::fmt::format::Writer<'_>) -> fmt::Result {
		write!(w, "{}", Local::now().format("%d-%m-%Y %H:%M:%S"))
	}
}

#[derive(Debug, Clone, Copy)]
pub struct LogLevel(Level);

impl Default for LogLevel {
	fn default() -> Self {
		LogLevel(Level::WARN)
	}
}

impl From<tracing::Level> for LogLevel {
	fn from(lvl: tracing::Level) -> Self {
		Self(lvl)
	}
}

impl From<LogLevel> for tracing::Level {
	fn from(value: LogLevel) -> Self {
		value.0
	}
}
