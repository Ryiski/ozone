mod error;

use error::{Error, Result};
use lib_cfg::Config;
use lib_cfg::cli::Cli;
use std::path::Path;

#[tokio::main]
async fn main() -> Result<()> {
	let cli: Cli = argh::from_env();

	if cli.version {
		println!("Ozone DNS Generator Version: {}", env!("CARGO_PKG_VERSION"));
		return Ok(());
	}

	let cfg_file_path = if let Some(file_path) = &cli.config {
		file_path
	} else {
		&Config::default_config_path()?
	};

	if !Path::new(cfg_file_path).exists() {
		return Err(Error::other("Config file not found"));
	}

	let cfg = Config::load(cfg_file_path, Some(&cli))?;

	let out_dir = cfg.out_dir();

	if let Err(err) = cfg.validate_config_directory() {
		tracing::error!("Target directory not writable!: {}", err);
		std::process::exit(1);
	} else {
		let directory_check = match lib_utils::files::directory_exists(&out_dir) {
			Ok(v) => v,
			Err(err) => {
				tracing::error!("Error: {}", err);
				false
			}
		};

		if !directory_check
			&& !cfg.dry_run
			&& let Err(err) = lib_utils::files::create_directory(&out_dir)
		{
			tracing::error!("Failed to create directory: {}", err);
			std::process::exit(1);
		}
	}

	let out_dir_config = out_dir.join("config");
	let out_dir_zones = out_dir.join("zones");

	if !cfg.dry_run {
		if let Err(err) = lib_utils::files::create_directory(&out_dir_config) {
			tracing::error!("Failed to create directory: {}", err);
			std::process::exit(1);
		}
		if let Err(err) = lib_utils::files::create_directory(&out_dir_zones) {
			tracing::error!("Failed to create directory: {}", err);
			std::process::exit(1);
		}
	}

	lib_tmpl::generator::bind::generate_bind_core(&cfg, &out_dir_config).ok();
	lib_tmpl::generator::bind::generate_forward_zone_includes(&cfg, &out_dir_zones).ok();
	lib_tmpl::generator::bind::generate_reverse_zone_includes(&cfg, &out_dir_zones).ok();
	lib_tmpl::generator::bind::generate_includes_conf(&cfg, &out_dir_config).ok();

	lib_tmpl::generator::adblock::generate_adblock_zone_template(&cfg, &out_dir_zones).ok();
	lib_tmpl::generator::zones::generate_forward_zones(&cfg, &out_dir_zones).ok();
	lib_tmpl::generator::zones::generate_reverse_zones(&cfg, &out_dir_zones).ok();

	Ok(())
}
