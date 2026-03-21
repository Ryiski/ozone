#[derive(Default, Debug, Clone, knus::Decode)]
pub struct View {
	#[knus(property)]
	pub name: String,
	#[knus(property, default)]
	pub recursion: bool,
	#[knus(child, unwrap(children, unwrap(argument)), default)]
	pub acls: Vec<String>,
	#[knus(child, unwrap(children, unwrap(argument)), default)]
	pub zones: Vec<String>,
	#[knus(child, unwrap(children, unwrap(argument)), default)]
	pub redirect: Vec<String>,
	#[knus(children(name = "redirect-zone"), default)]
	pub redirect_zones: Vec<RedirectZone>,
	#[knus(children(name="include"), unwrap(argument), default = vec![])]
	pub includes: Vec<String>,
}

#[derive(Default, Debug, Clone, knus::Decode)]
pub struct RedirectZone {
	#[knus(property)]
	pub domain: String,
	#[knus(children, unwrap(argument), default = vec![])]
	pub networks: Vec<String>,
}
