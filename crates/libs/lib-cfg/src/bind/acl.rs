#[derive(Default, Debug, Clone, knus::Decode)]
pub struct Acl {
	#[knus(property)]
	pub name: String,
	#[knus(children, unwrap(argument), default = vec![])]
	pub networks: Vec<String>,
}
