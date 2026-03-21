use argh::FromArgs;
use std::path::PathBuf;

#[derive(Debug, Default, FromArgs)]
/// Config
pub struct Cli {
	/// path to the configuration file
	#[argh(option, short = 'c')]
	pub config: Option<PathBuf>,

	/// set log level
	// #[argh(option, short = 'l')]
	// pub log_level: Option<Level>,

	/// show verbose output
	#[argh(switch, short = 'v')]
	pub verbose: bool,

	/// generate output but do not write files
	#[argh(switch, short = 'd')]
	pub dry_run: bool,

	/// show version information
	#[argh(switch, short = 'V')]
	pub version: bool,
}
