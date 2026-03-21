use askama::Template;

#[derive(Template)]
#[template(path = "bind-zone.tmpl", escape = "none")]
pub struct BindZoneTemplate<'a> {
	pub zone: &'a str,
	pub path: &'a str,
}

pub struct RenderView<'a> {
	pub view: &'a lib_cfg::bind::view::View,
	pub reverse_zones: Vec<&'a String>,
}

#[derive(Template)]
#[template(path = "bind.tmpl", escape = "none")]
pub struct BindTemplate<'a> {
	pub cfg: &'a lib_cfg::Config,
	pub out_dir: &'a str,
	pub views: Vec<RenderView<'a>>,
}
