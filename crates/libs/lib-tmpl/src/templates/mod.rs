pub mod bind;
pub mod zones;

use crate::templates::{
	bind::{BindTemplate, BindZoneTemplate},
	zones::ZoneTmplData,
};
use askama::Template;
use std::{fs, path::Path};

pub enum GeneratorTemplate<'a> {
	BindZone(BindZoneTemplate<'a>),
	Bind(BindTemplate<'a>),
	Zone(ZoneTmplData<'a>),
}

impl<'a> GeneratorTemplate<'a> {
	pub fn render_to_file(&self, path: &Path, dry_run: bool) -> Result<(), String> {
		if dry_run {
			tracing::info!("dry-run → would generate {}", path.display());
			return Ok(());
		}

		let content = match self {
			GeneratorTemplate::BindZone(tmpl) => tmpl.render().map_err(|e| e.to_string())?,
			GeneratorTemplate::Bind(tmpl) => tmpl.render().map_err(|e| e.to_string())?,
			GeneratorTemplate::Zone(tmpl) => tmpl.render().map_err(|e| e.to_string())?,
		};

		if let Some(parent) = path.parent() {
			lib_utils::files::create_directory(parent).ok(); // Ensure parent directory exists
		}

		fs::write(path, content)
			.map_err(|e| format!("Failed to write to {}: {}", path.display(), e))?;
		tracing::info!("Generated {}", path.display());
		Ok(())
	}
}
