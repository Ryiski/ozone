use std::{
	fs,
	path::{Path, PathBuf},
};

use crate::{
	templates::{
		GeneratorTemplate,
		bind::{BindTemplate, BindZoneTemplate, RenderView},
	},
	utils,
};

pub fn generate_bind_core(cfg: &lib_cfg::Config, out_dir: &Path) -> Result<(), String> {
	let bind_path = out_dir.join("named.core.conf");

	let rev_zones_per_domain = utils::build_reverse_zones_per_zone(cfg);

	let out_dir_cow_str = out_dir.to_string_lossy();
	let out_dir_string = out_dir_cow_str.into_owned();

	let render_views: Vec<RenderView> = cfg
		.bind
		.views
		.iter()
		.map(|view| {
			let mut view_rev_zones: Vec<&String> = Vec::new(); // references

			for domain in &view.zones {
				if let Some(revs) = rev_zones_per_domain.get(domain) {
					view_rev_zones.extend(revs.iter()); // just references, no clone
				}
			}

			RenderView {
				view,
				reverse_zones: view_rev_zones,
			}
		})
		.collect();

	let template_data = BindTemplate {
		cfg,
		out_dir: &out_dir_string,
		views: render_views,
	};

	GeneratorTemplate::Bind(template_data).render_to_file(&bind_path, cfg.dry_run)
}

pub fn generate_forward_zone_includes(cfg: &lib_cfg::Config, out_dir: &Path) -> Result<(), String> {
	for zone in &cfg.zones {
		let zone_file = out_dir.join(format!("{}.zone", zone.domain));
		write_zone_include(
			&zone.domain,
			&zone_file.to_string_lossy(),
			out_dir,
			cfg.dry_run,
		)?;
	}
	Ok(())
}

pub fn generate_reverse_zone_includes(cfg: &lib_cfg::Config, out_dir: &Path) -> Result<(), String> {
	let reverse_zones = utils::build_reverse_zones(cfg);

	for rev_zone in reverse_zones {
		let zone_file = out_dir.join(format!("rev.{}.zone", rev_zone));

		write_zone_include(
			&format!("rev.{}", rev_zone),
			&zone_file.to_string_lossy(),
			out_dir,
			cfg.dry_run,
		)?;
	}
	Ok(())
}

pub fn write_zone_include(
	zone_name: &str,
	zone_file: &str,
	out_dir: &Path,
	dry_run: bool,
) -> Result<(), String> {
	let zone_path = out_dir
		.join("..")
		.join("config")
		.join(format!("{}.zone.conf", zone_name));
	let template_data = BindZoneTemplate {
		zone: zone_name,
		path: zone_file,
	};
	GeneratorTemplate::BindZone(template_data).render_to_file(&zone_path, dry_run)
}

pub fn generate_includes_conf(cfg: &lib_cfg::Config, out_dir: &Path) -> Result<(), String> {
	let includes_conf_path = out_dir.join("..").join("includes.conf");

	let mut includes_content = String::new();
	if out_dir.exists() {
		let files = fs::read_dir(out_dir).map_err(|e| {
			format!(
				"Failed to read config directory {}: {}",
				out_dir.display(),
				e
			)
		})?;

		for entry in files {
			let entry: fs::DirEntry =
				entry.map_err(|e| format!("Failed to read directory entry: {}", e))?;

			// println!("{}", entry.());
			let path = entry.path();
			if path.is_file() {
				let file_name = path.file_name().unwrap_or_default().to_string_lossy();
				if file_name.ends_with(".zone.conf") {
					includes_content
						.push_str(&format!("include \"{}\";\n", path.to_string_lossy()));
				}
			}
		}
	}

	if cfg.dry_run {
		tracing::info!("dry-run → would generate {}", includes_conf_path.display());
		Ok(())
	} else {
		lib_utils::files::create_directory(&PathBuf::from(out_dir)).ok();
		fs::write(&includes_conf_path, includes_content)
			.map_err(|e| format!("Failed to write {}: {}", includes_conf_path.display(), e))?;
		tracing::info!("Generated {}", includes_conf_path.display());
		Ok(())
	}
}
